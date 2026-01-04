use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhotoMetadata {
    pub path: String,
    pub file_name: String,
    pub file_size: u64,
    pub created_at: Option<DateTime<Utc>>,
    pub modified_at: Option<DateTime<Utc>>,
    pub date_taken: Option<DateTime<Utc>>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub hash: String,
    pub is_screenshot: bool,
    pub is_duplicate: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanProgress {
    pub total_files: usize,
    pub processed_files: usize,
    pub current_file: String,
    pub percentage: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanStats {
    pub total_photos: usize,
    pub total_videos: usize,
    pub screenshots: usize,
    pub duplicates: usize,
    pub total_size: u64,
    pub date_range: Option<(DateTime<Utc>, DateTime<Utc>)>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanResult {
    pub photos: Vec<PhotoMetadata>,
    pub stats: ScanStats,
}
