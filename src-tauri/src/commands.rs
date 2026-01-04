use crate::scanner::Scanner;
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
