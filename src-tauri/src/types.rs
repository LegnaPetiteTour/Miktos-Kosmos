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
pub struct QualityIssues {
    pub screenshots: usize,
    pub duplicates: usize,
    pub low_resolution: usize,      // Images below 1080p
    pub small_files: usize,          // Compressed/low quality (< 500KB)
    pub missing_metadata: usize,     // No EXIF date
    pub potential_memes: usize,      // Suspicious filenames
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanStats {
    pub total_files: usize,
    pub file_types: FileTypeStats,
    pub screenshots: usize,          // Deprecated - use quality.screenshots
    pub duplicates: usize,            // Deprecated - use quality.duplicates
    pub total_size: u64,
    pub date_range: Option<(DateTime<Utc>, DateTime<Utc>)>,
    pub quality: QualityIssues,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanResult {
    pub root_path: String,              // The scanned folder path
    pub files: Vec<FileMetadata>,       // Changed from 'photos'
    pub stats: ScanStats,
}

// ============================================================================
// ORGANIZATION / TRANSFORM TYPES
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OrganizationStrategy {
    Date,           // YYYY/MM-Month format
    Year,           // YYYY only
    YearMonth,      // YYYY/MM
    FileType,       // Images/Videos/Documents
    DateAndType,    // YYYY/MM-Month/Images
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OperationMode {
    Copy,   // Safe: Keep originals
    Move,   // Destructive: Remove originals
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FolderPreview {
    pub path: String,
    pub file_count: usize,
    pub total_size: u64,
    pub files: Vec<String>, // File names that will go here
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganizationPlan {
    pub destination_root: String,
    pub strategy: OrganizationStrategy,
    pub mode: OperationMode,
    pub folders: Vec<FolderPreview>,
    pub total_files: usize,
    pub total_size: u64,
    pub files_without_dates: usize, // Files that will go to "Unknown"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OperationStatus {
    Success,
    Failed,
    Skipped,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileOperation {
    pub source_path: String,
    pub destination_path: String,
    pub status: OperationStatus,
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationResult {
    pub success: bool,
    pub operations: Vec<FileOperation>,
    pub successful_count: usize,
    pub failed_count: usize,
    pub skipped_count: usize,
    pub total_size_processed: u64,
    pub duration_ms: u64,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationProgress {
    pub current_file: String,
    pub processed: usize,
    pub total: usize,
    pub percentage: f32,
}
