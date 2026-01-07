// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod scanner;
mod types;
mod commands;
mod organizer;
mod executor;

use commands::*;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            scan_directory,
            get_scan_stats,
            create_organization_plan,
            execute_organization,
            get_home_dir,
            list_directory
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
