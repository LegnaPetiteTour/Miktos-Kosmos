use std::fs;
use std::path::{Path, PathBuf};
use std::time::Instant;
use chrono::Utc;
use crate::types::*;

/// Handle duplicate filenames by appending a counter
fn resolve_duplicate_filename(destination: &Path) -> PathBuf {
    if !destination.exists() {
        return destination.to_path_buf();
    }

    let parent = destination.parent().unwrap();
    let file_stem = destination.file_stem().unwrap().to_str().unwrap();
    let extension = destination.extension().and_then(|e| e.to_str()).unwrap_or("");

    let mut counter = 1;
    loop {
        let new_name = if extension.is_empty() {
            format!("{file_stem}_{counter}")
        } else {
            format!("{file_stem}_{counter}.{extension}")
        };
        
        let new_path = parent.join(new_name);
        if !new_path.exists() {
            return new_path;
        }
        counter += 1;
    }
}

/// Execute organization plan - copy or move files
pub fn execute_organization_plan(
    plan: OrganizationPlan,
    source_files: Vec<FileMetadata>,
) -> Result<OperationResult, String> {
    let start_time = Instant::now();
    let mut operations: Vec<FileOperation> = Vec::new();
    let mut successful_count = 0;
    let mut failed_count = 0;
    let mut skipped_count = 0;
    let mut total_size_processed = 0u64;

    // Create a map of file paths to their destinations
    let mut file_destinations: std::collections::HashMap<String, String> = std::collections::HashMap::new();
    
    for folder in &plan.folders {
        for file_name in &folder.files {
            // Find the source file
            if let Some(source_file) = source_files.iter().find(|f| f.file_name == *file_name) {
                let dest_path = Path::new(&folder.path).join(file_name);
                file_destinations.insert(source_file.path.clone(), dest_path.to_string_lossy().to_string());
            }
        }
    }

    // Process each file
    for source_file in source_files {
        let source_path = Path::new(&source_file.path);
        
        if !source_path.exists() {
            operations.push(FileOperation {
                source_path: source_file.path.clone(),
                destination_path: String::new(),
                status: OperationStatus::Skipped,
                error_message: Some("Source file does not exist".to_string()),
            });
            skipped_count += 1;
            continue;
        }

        // Get destination path
        let dest_path_str = match file_destinations.get(&source_file.path) {
            Some(path) => path,
            None => {
                operations.push(FileOperation {
                    source_path: source_file.path.clone(),
                    destination_path: String::new(),
                    status: OperationStatus::Skipped,
                    error_message: Some("No destination found".to_string()),
                });
                skipped_count += 1;
                continue;
            }
        };

        let dest_path = Path::new(dest_path_str);
        
        // Create destination directory
        if let Some(parent) = dest_path.parent() {
            if let Err(e) = fs::create_dir_all(parent) {
                operations.push(FileOperation {
                    source_path: source_file.path.clone(),
                    destination_path: dest_path.to_string_lossy().to_string(),
                    status: OperationStatus::Failed,
                    error_message: Some(format!("Failed to create directory: {e}")),
                });
                failed_count += 1;
                continue;
            }
        }

        // Resolve duplicate filename if necessary
        let final_dest = resolve_duplicate_filename(dest_path);

        // Perform operation
        let result = match plan.mode {
            OperationMode::Copy => fs::copy(source_path, &final_dest),
            OperationMode::Move => {
                // Copy first, then delete if successful
                match fs::copy(source_path, &final_dest) {
                    Ok(bytes) => {
                        // Delete source after successful copy
                        if let Err(e) = fs::remove_file(source_path) {
                            // Copy succeeded but delete failed - not ideal but file is safe
                            eprintln!("Warning: Failed to delete source file after copy: {e}");
                        }
                        Ok(bytes)
                    }
                    Err(e) => Err(e),
                }
            }
        };

        match result {
            Ok(_) => {
                operations.push(FileOperation {
                    source_path: source_file.path.clone(),
                    destination_path: final_dest.to_string_lossy().to_string(),
                    status: OperationStatus::Success,
                    error_message: None,
                });
                successful_count += 1;
                total_size_processed += source_file.file_size;
            }
            Err(e) => {
                operations.push(FileOperation {
                    source_path: source_file.path.clone(),
                    destination_path: final_dest.to_string_lossy().to_string(),
                    status: OperationStatus::Failed,
                    error_message: Some(e.to_string()),
                });
                failed_count += 1;
            }
        }
    }

    let duration_ms = start_time.elapsed().as_millis() as u64;
    let success = failed_count == 0 && skipped_count < operations.len();

    Ok(OperationResult {
        success,
        operations,
        successful_count,
        failed_count,
        skipped_count,
        total_size_processed,
        duration_ms,
        timestamp: Utc::now(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use tempfile::TempDir;

    #[test]
    fn test_resolve_duplicate_filename() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        
        // Create original file
        File::create(&file_path).unwrap();
        
        // Should return test_1.txt
        let resolved = resolve_duplicate_filename(&file_path);
        assert_eq!(resolved.file_name().unwrap().to_str().unwrap(), "test_1.txt");
    }
}
