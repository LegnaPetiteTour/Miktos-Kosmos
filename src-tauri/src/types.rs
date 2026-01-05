use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FileType {
    Image,
    Video,
    Document,   // PDF, DOCX, TXT, etc.
    Audio,
    Archive,    // ZIP, RAR, etc.
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileMetadata {
    pub path: String,
    pub file_name: String,
    pub file_size: u64,
    pub file_type: FileType,
    pub created_at: Option<DateTime<Utc>>,
    pub modified_at: Option<DateTime<Utc>>,
    
    // Optional metadata (depends on file type)
    pub date_taken: Option<DateTime<Utc>>,  // For photos/videos
    pub width: Option<u32>,                  // For images/videos
    pub height: Option<u32>,                 // For images/videos
    pub duration: Option<u32>,               // For videos/audio (seconds)
    pub page_count: Option<u32>,             // For documents
    
    pub hash: String,
    pub is_screenshot: bool,
    pub is_duplicate: bool,
    
    // Media-specific (for backward compatibility)
    pub camera_make: Option<String>,
    pub camera_model: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanProgress {
    pub total_files: usize,
    pub processed_files: usize,
    pub current_file: String,
    pub percentage: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileTypeStats {
    pub images: usize,
    pub videos: usize,
    pub documents: usize,
    pub audio: usize,
    pub archives: usize,
    pub other: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanStats {
    pub total_files: usize,
    pub file_types: FileTypeStats,
    pub screenshots: usize,
    pub duplicates: usize,
    pub total_size: u64,
    pub date_range: Option<(DateTime<Utc>, DateTime<Utc>)>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanResult {
    pub files: Vec<FileMetadata>,  // Changed from 'photos'
    pub stats: ScanStats,
}
