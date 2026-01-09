// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod scanner;
mod types;
mod commands;
mod organizer;
mod executor;

use commands::*;
use tauri::menu::{Menu, MenuItem, Submenu, PredefinedMenuItem};
use tauri::Manager;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            // Create menu
            let handle = app.handle();
            
            // App menu (macOS)
            let about = MenuItem::with_id(app, "about", "About Miktos Kosmos", true, None::<&str>)?;
            let settings = MenuItem::with_id(app, "settings", "Settings...", true, Some("CmdOrCtrl+,"))?;
            let quit = PredefinedMenuItem::quit(app, Some("Quit"))?;
            
            let app_menu = Submenu::with_items(
                app,
                "Miktos Kosmos",
                true,
                &[
                    &about,
                    &PredefinedMenuItem::separator(app)?,
                    &settings,
                    &PredefinedMenuItem::separator(app)?,
                    &PredefinedMenuItem::hide(app, None)?,
                    &PredefinedMenuItem::hide_others(app, None)?,
                    &PredefinedMenuItem::show_all(app, None)?,
                    &PredefinedMenuItem::separator(app)?,
                    &quit,
                ]
            )?;
            
            // Help menu
            let learn = MenuItem::with_id(app, "learn", "Learn Miktos Kosmos", true, None::<&str>)?;
            let help_menu = Submenu::with_items(
                app,
                "Help",
                true,
                &[&learn]
            )?;
            
            // Build menu
            let menu = Menu::with_items(
                app,
                &[
                    &app_menu,
                    &help_menu,
                ]
            )?;
            
            // Set menu
            app.set_menu(menu)?;
            
            // Handle menu events
            app.on_menu_event(|app, event| {
                match event.id().as_ref() {
                    "about" => {
                        // Emit event to frontend to show About dialog
                        app.emit("show-about", ()).ok();
                    }
                    "settings" => {
                        // Emit event to frontend to show Settings dialog
                        app.emit("show-settings", ()).ok();
                    }
                    "learn" => {
                        // Emit event to frontend to show Learn dialog
                        app.emit("show-learn", ()).ok();
                    }
                    _ => {}
                }
            });
            
            Ok(())
        })
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
