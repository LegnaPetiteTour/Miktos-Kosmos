use std::path::{Path, PathBuf};
use std::fs;
use std::collections::HashMap;
use walkdir::WalkDir;
use sha2::{Sha256, Digest};
use chrono::{DateTime, Utc, NaiveDateTime};
use std::io::Read;
use exif::{Reader, In, Tag};
use tauri::{AppHandle, Emitter};

use crate::types::*;

pub struct Scanner {
    root_path: PathBuf,
}

impl Scanner {
    pub fn new(root_path: impl AsRef<Path>) -> Self {
        Self {
            root_path: root_path.as_ref().to_path_buf(),
        }
    }

    pub fn scan(&self, app_handle: &AppHandle) -> Result<ScanResult, Box<dyn std::error::Error>> {
        // Step 1: Quick count of total files
        let total_files = self.count_total_files();
        
        let mut files = Vec::new();
        let mut total_size = 0u64;
        let mut screenshot_count = 0;
        let mut image_count = 0;
        let mut video_count = 0;
        let mut processed_count = 0;

        for entry in WalkDir::new(&self.root_path)
            .follow_links(false)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if entry.file_type().is_file() {
                let file_name = entry.path()
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string();
                
                if let Some(ext) = entry.path().extension() {
                    let ext = ext.to_string_lossy().to_lowercase();
                    
                    // Check if it's an image file
                    if is_image_extension(&ext) {
                        if let Ok(metadata) = self.process_image(entry.path()) {
                            total_size += metadata.file_size;
                            if metadata.is_screenshot {
                                screenshot_count += 1;
                            }
                            image_count += 1;
                            files.push(metadata);
                        }
                    }
                    // Check if it's a video file
                    else if is_video_extension(&ext) {
                        if let Ok(metadata) = self.process_video(entry.path()) {
                            total_size += metadata.file_size;
                            video_count += 1;
                            files.push(metadata);
                        }
                    }
                }
                
                // Update progress
                processed_count += 1;
                
                // Emit progress every 10 files or on last file
                if processed_count % 10 == 0 || processed_count == total_files {
                    let percentage = if total_files > 0 {
                        (processed_count as f32 / total_files as f32) * 100.0
                    } else {
                        0.0
                    };
                    
                    let progress = ScanProgress {
                        total_files,
                        processed_files: processed_count,
                        current_file: file_name,
                        percentage,
                    };
                    
                    // Emit progress event (ignore errors)
                    let _ = app_handle.emit("scan-progress", &progress);
                }
            }
        }

        // Calculate date range
        let date_range = files.iter()
            .filter_map(|f| f.date_taken.or(f.modified_at))
            .fold(None, |acc, date| {
                match acc {
                    None => Some((date, date)),
                    Some((min, max)) => Some((min.min(date), max.max(date)))
                }
            });

        // Calculate quality issues
        let image_files: Vec<&FileMetadata> = files.iter()
            .filter(|f| matches!(f.file_type, FileType::Image))
            .collect();

        // 1. Count screenshots (already detected)
        let screenshots = screenshot_count;

        // 2. Detect duplicates by hash
        let mut hash_map: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
        for file in &files {
            *hash_map.entry(file.hash.clone()).or_insert(0) += 1;
        }
        let duplicates = hash_map.values().filter(|&&count| count > 1)
            .map(|&count| count - 1)
            .sum();

        // 3. Low resolution images (below 1920×1080)
        let low_resolution = image_files.iter()
            .filter(|f| {
                if let (Some(w), Some(h)) = (f.width, f.height) {
                    (w as u64) * (h as u64) < (1920 * 1080)
                } else {
                    false
                }
            })
            .count();

        // 4. Small files (compressed/low quality - below 500KB)
        let small_files = image_files.iter()
            .filter(|f| f.file_size < 500 * 1024)
            .count();

        // 5. Missing metadata (no EXIF date)
        let missing_metadata = image_files.iter()
            .filter(|f| f.date_taken.is_none())
            .count();

        // 6. Potential memes/downloads (suspicious filenames)
        let suspicious_patterns = vec![
            "meme", "funny", "lol", "image", "download", "untitled",
            "img_", "pic_", "photo_", "picture_", "file_", "temp"
        ];
        let potential_memes = files.iter()
            .filter(|f| {
                let file_name = f.file_name.to_lowercase();
                suspicious_patterns.iter().any(|pattern| file_name.contains(pattern))
            })
            .count();

        let stats = ScanStats {
            total_files: files.len(),
            file_types: FileTypeStats {
                images: image_count,
                videos: video_count,
                documents: 0,
                audio: 0,
                archives: 0,
                other: 0,
            },
            screenshots: screenshot_count,
            duplicates,
            total_size,
            date_range,
            quality: QualityIssues {
                screenshots,
                duplicates,
                low_resolution,
                small_files,
                missing_metadata,
                potential_memes,
            },
        };

        Ok(ScanResult { 
            root_path: self.root_path.to_string_lossy().to_string(),
            files, 
            stats 
        })
    }
    
    // Quick count of total files (just metadata, no processing)
    fn count_total_files(&self) -> usize {
        WalkDir::new(&self.root_path)
            .follow_links(false)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
            .count()
    }

    fn process_image(&self, path: &Path) -> Result<FileMetadata, Box<dyn std::error::Error>> {
        let metadata = fs::metadata(path)?;
        let file_size = metadata.len();
        
        // Calculate file hash
        let hash = self.calculate_hash(path)?;
        
        // Try to read image dimensions
        let (width, height) = match image::image_dimensions(path) {
            Ok((w, h)) => (Some(w), Some(h)),
            Err(_) => (None, None),
        };

        // Extract dates
        let created_at = metadata.created().ok()
            .map(DateTime::<Utc>::from);
        let modified_at = metadata.modified().ok()
            .map(DateTime::<Utc>::from);

        // Extract EXIF data for actual date_taken
        let date_taken = self.extract_exif_date(path).or(modified_at);

        // Detect if it's likely a screenshot
        let is_screenshot = self.is_likely_screenshot(path, width, height);

        Ok(FileMetadata {
            path: path.to_string_lossy().to_string(),
            file_name: path.file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string(),
            file_size,
            file_type: FileType::Image,
            created_at,
            modified_at,
            date_taken,
            width,
            height,
            duration: None,
            page_count: None,
            hash,
            is_screenshot,
            is_duplicate: false,
            camera_make: None,
            camera_model: None,
        })
    }

    fn process_video(&self, path: &Path) -> Result<FileMetadata, Box<dyn std::error::Error>> {
        let metadata = fs::metadata(path)?;
        let file_size = metadata.len();
        
        // Calculate file hash
        let hash = self.calculate_hash(path)?;

        // Extract dates
        let created_at = metadata.created().ok()
            .map(DateTime::<Utc>::from);
        let modified_at = metadata.modified().ok()
            .map(DateTime::<Utc>::from);

        Ok(FileMetadata {
            path: path.to_string_lossy().to_string(),
            file_name: path.file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string(),
            file_size,
            file_type: FileType::Video,
            created_at,
            modified_at,
            date_taken: None,
            width: None,  // Could extract with ffmpeg later
            height: None,
            duration: None,  // Could extract with ffmpeg later
            page_count: None,
            hash,
            is_screenshot: false,
            is_duplicate: false,
            camera_make: None,
            camera_model: None,
        })
    }

    fn calculate_hash(&self, path: &Path) -> Result<String, Box<dyn std::error::Error>> {
        let mut file = fs::File::open(path)?;
        let mut hasher = Sha256::new();
        let mut buffer = [0; 8192];

        loop {
            let count = file.read(&mut buffer)?;
            if count == 0 {
                break;
            }
            hasher.update(&buffer[..count]);
        }

        Ok(format!("{:x}", hasher.finalize()))
    }

    fn is_likely_screenshot(&self, path: &Path, width: Option<u32>, height: Option<u32>) -> bool {
        // Check filename patterns
        let file_name = path.file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_lowercase();

        if file_name.contains("screenshot") || 
           file_name.contains("screen shot") ||
           file_name.starts_with("screen_") {
            return true;
        }

        // Check common screenshot dimensions
        if let (Some(w), Some(h)) = (width, height) {
            // Common mobile screenshot dimensions
            let aspect_ratio = w as f64 / h as f64;
            let is_mobile_aspect = (aspect_ratio - 9.0/16.0).abs() < 0.1 || 
                                  (aspect_ratio - 16.0/9.0).abs() < 0.1;
            
            // Common screenshot sizes
            let is_common_screenshot_size = 
                (w == 1920 && h == 1080) ||
                (w == 1080 && h == 1920) ||
                (w == 1080 && h == 2340) ||
                (w == 1080 && h == 2400) ||
                (w == 1284 && h == 2778);

            if is_mobile_aspect && is_common_screenshot_size {
                return true;
            }
        }

        false
    }

    fn extract_exif_date(&self, path: &Path) -> Option<DateTime<Utc>> {
        let file = fs::File::open(path).ok()?;
        let mut bufreader = std::io::BufReader::new(&file);
        let exifreader = Reader::new();
        let exif = exifreader.read_from_container(&mut bufreader).ok()?;

        // Try DateTimeOriginal first (when photo was taken)
        if let Some(field) = exif.get_field(Tag::DateTimeOriginal, In::PRIMARY) {
            if let Some(datetime_str) = field.display_value().to_string().split('"').nth(1) {
                return Self::parse_exif_datetime(datetime_str);
            }
        }

        // Fall back to DateTime (when file was created)
        if let Some(field) = exif.get_field(Tag::DateTime, In::PRIMARY) {
            if let Some(datetime_str) = field.display_value().to_string().split('"').nth(1) {
                return Self::parse_exif_datetime(datetime_str);
            }
        }

        None
    }

    fn parse_exif_datetime(datetime_str: &str) -> Option<DateTime<Utc>> {
        // EXIF datetime format: "YYYY:MM:DD HH:MM:SS"
        let parts: Vec<&str> = datetime_str.split(' ').collect();
        if parts.len() != 2 {
            return None;
        }

        let date_parts: Vec<&str> = parts[0].split(':').collect();
        let time_parts: Vec<&str> = parts[1].split(':').collect();

        if date_parts.len() != 3 || time_parts.len() != 3 {
            return None;
        }

        let year = date_parts[0].parse::<i32>().ok()?;
        let month = date_parts[1].parse::<u32>().ok()?;
        let day = date_parts[2].parse::<u32>().ok()?;
        let hour = time_parts[0].parse::<u32>().ok()?;
        let minute = time_parts[1].parse::<u32>().ok()?;
        let second = time_parts[2].parse::<u32>().ok()?;

        NaiveDateTime::parse_from_str(
            &format!("{year:04}-{month:02}-{day:02} {hour:02}:{minute:02}:{second:02}"),
            "%Y-%m-%d %H:%M:%S"
        )
        .ok()
        .map(|dt| DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc))
    }
}

fn is_image_extension(ext: &str) -> bool {
    matches!(
        ext,
        "jpg" | "jpeg" | "png" | "gif" | "bmp" | "webp" | "heic" | "heif" | "tiff" | "tif"
    )
}

fn is_video_extension(ext: &str) -> bool {
    matches!(
        ext,
        "mp4" | "mov" | "avi" | "mkv" | "m4v" | "wmv" | "flv" | "webm" | "3gp"
    )
}
