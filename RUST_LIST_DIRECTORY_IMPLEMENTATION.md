# Rust Backend Implementation for list_directory

Add this to your `src-tauri/src/main.rs` file to provide complete file metadata:

```rust
use std::fs;
use std::path::Path;
use serde::{Serialize, Deserialize};
use std::time::SystemTime;

#[derive(Serialize, Deserialize)]
struct FileInfo {
    name: String,
    path: String,
    is_dir: bool,
    size: u64,
    modified: Option<u64>,
    created: Option<u64>,
}

#[tauri::command]
fn list_directory(path: String) -> Result<Vec<FileInfo>, String> {
    let dir_path = Path::new(&path);
    
    if !dir_path.exists() {
        return Err(format!("Path does not exist: {}", path));
    }
    
    if !dir_path.is_dir() {
        return Err(format!("Path is not a directory: {}", path));
    }
    
    let entries = fs::read_dir(&path)
        .map_err(|e| format!("Failed to read directory: {}", e))?;
    
    let mut files = Vec::new();
    
    for entry in entries {
        if let Ok(entry) = entry {
            if let Ok(metadata) = entry.metadata() {
                let file_path = entry.path();
                let name = entry.file_name().to_string_lossy().to_string();
                let path_str = file_path.to_string_lossy().to_string();
                
                // Get modified time
                let modified = metadata.modified()
                    .ok()
                    .and_then(|time| time.duration_since(SystemTime::UNIX_EPOCH).ok())
                    .map(|duration| duration.as_secs());
                
                // Get created time
                let created = metadata.created()
                    .ok()
                    .and_then(|time| time.duration_since(SystemTime::UNIX_EPOCH).ok())
                    .map(|duration| duration.as_secs());
                
                files.push(FileInfo {
                    name,
                    path: path_str,
                    is_dir: metadata.is_dir(),
                    size: metadata.len(),
                    modified,
                    created,
                });
            }
        }
    }
    
    // Sort: folders first, then alphabetically
    files.sort_by(|a, b| {
        if a.is_dir && !b.is_dir {
            std::cmp::Ordering::Less
        } else if !a.is_dir && b.is_dir {
            std::cmp::Ordering::Greater
        } else {
            a.name.to_lowercase().cmp(&b.name.to_lowercase())
        }
    });
    
    Ok(files)
}

// Also add get_home_dir if not already present:
#[tauri::command]
fn get_home_dir() -> Result<String, String> {
    use std::env;
    env::var("HOME")
        .or_else(|_| env::var("USERPROFILE"))
        .map_err(|e| format!("Failed to get home directory: {}", e))
}
```

## Add to invoke_handler

In your `main.rs`, make sure both commands are registered:

```rust
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            // ... your existing commands ...
            get_home_dir,
            list_directory
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

## What This Provides

Each file will have:
- `name`: Filename
- `path`: Full path
- `is_dir`: Boolean for folder vs file
- `size`: File size in bytes (0 for folders)
- `modified`: Unix timestamp (seconds since epoch)
- `created`: Unix timestamp (seconds since epoch)

This matches exactly what the ContentPanel expects!
