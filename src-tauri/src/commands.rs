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
