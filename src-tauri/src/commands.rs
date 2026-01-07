use crate::scanner::Scanner;
use crate::organizer::generate_organization_plan;
use crate::executor::execute_organization_plan;
use crate::types::*;

#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {name}! Welcome to Miktos Kosmos.")
}

#[tauri::command]
pub async fn scan_directory(path: String) -> Result<ScanResult, String> {
    let scanner = Scanner::new(&path);
    scanner.scan().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_scan_stats(result: ScanResult) -> ScanStats {
    result.stats
}

// ============================================================================
// ORGANIZATION COMMANDS
// ============================================================================

#[tauri::command]
pub async fn create_organization_plan(
    files: Vec<FileMetadata>,
    destination_root: String,
    strategy: OrganizationStrategy,
    mode: OperationMode,
) -> Result<OrganizationPlan, String> {
    generate_organization_plan(files, destination_root, strategy, mode)
}

#[tauri::command]
pub async fn execute_organization(
    plan: OrganizationPlan,
    source_files: Vec<FileMetadata>,
) -> Result<OperationResult, String> {
    execute_organization_plan(plan, source_files)
}

// ============================================================================
// FOLDER NAVIGATION COMMANDS
// ============================================================================

#[tauri::command]
pub fn get_home_dir() -> Result<String, String> {
    use std::env;
    env::var("HOME")
        .or_else(|_| env::var("USERPROFILE"))
        .map_err(|e| format!("Failed to get home directory: {e}"))
}

#[tauri::command]
pub fn list_directory(path: String) -> Result<Vec<serde_json::Value>, String> {
    use std::fs;
    
    let entries = fs::read_dir(&path)
        .map_err(|e| format!("Failed to read directory: {e}"))?;
    
    let mut items = Vec::new();
    
    for entry in entries.flatten() {
        if let Ok(metadata) = entry.metadata() {
            let name = entry.file_name().to_string_lossy().to_string();
            let path = entry.path().to_string_lossy().to_string();
            
            items.push(serde_json::json!({
                "name": name,
                "path": path,
                "is_dir": metadata.is_dir()
            }));
        }
    }
    
    Ok(items)
}
