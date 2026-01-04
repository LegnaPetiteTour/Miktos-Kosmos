# PhotoArchive Architecture

## System Overview

PhotoArchive is a desktop application built with:
- **Frontend**: SvelteKit + TypeScript + Tailwind CSS
- **Backend**: Rust + Tauri 2.0
- **Database**: SQLite (planned)
- **AI/ML**: Local ONNX models (planned)

```
┌─────────────────────────────────────────────────────┐
│                   User Interface                     │
│              (SvelteKit + Tailwind)                  │
└───────────────────┬─────────────────────────────────┘
                    │ IPC (Tauri Commands)
┌───────────────────▼─────────────────────────────────┐
│                 Rust Backend                         │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐          │
│  │ Scanner  │  │ Organizer│  │   AI     │          │
│  │  Module  │  │  Module  │  │  Module  │          │
│  └────┬─────┘  └────┬─────┘  └────┬─────┘          │
│       │             │              │                 │
│  ┌────▼─────────────▼──────────────▼─────┐          │
│  │       SQLite Database                  │          │
│  └────────────────────────────────────────┘          │
└─────────────┬───────────────────────────────────────┘
              │
┌─────────────▼───────────────────────────────────────┐
│              File System                             │
│         (Photos, Videos, Metadata)                   │
└─────────────────────────────────────────────────────┘
```

## Core Modules

### 1. Scanner Module (`scanner.rs`)

**Responsibility**: Discover and analyze files

**Key Functions**:
- `scan()` - Walk directory tree and find image/video files
- `process_photo()` - Extract metadata from individual photos
- `calculate_hash()` - Generate SHA-256 hash for duplicate detection
- `is_likely_screenshot()` - Heuristic screenshot detection

**Data Flow**:
```
Directory Path → WalkDir → Filter by Extension → 
Process Photo → Extract Metadata → Store in Database
```

**Performance Considerations**:
- Uses `rayon` for parallel processing
- Streams results to avoid memory overflow
- Skips hidden files and system directories

### 2. Organizer Module (planned)

**Responsibility**: Create organized folder structures

**Key Functions**:
- `preview_organization()` - Show what will happen without making changes
- `apply_organization()` - Execute the organization
- `undo_organization()` - Revert changes

**Organization Strategies**:
```rust
enum OrganizationStrategy {
    ByDate {
        year_month: bool,
        year_month_day: bool,
    },
    ByEvent {
        time_gap: Duration,  // Photos within X hours = same event
    },
    ByPeople {
        use_face_detection: bool,
    },
    Custom {
        template: String,  // e.g., "{year}/{month}/{event}"
    }
}
```

### 3. AI Module (planned)

**Responsibility**: Smart photo analysis

**Features**:
- **Face Detection**: Local ONNX model (no cloud)
- **Quality Scoring**: Blur detection, exposure analysis
- **Scene Classification**: Indoor/outdoor, landscape/portrait
- **Duplicate Detection**: Perceptual hashing

**Privacy**:
- All models run locally
- No data sent to external servers
- Models bundled with app

### 4. Database Module (planned)

**Responsibility**: Store and query metadata

**Schema**:
```sql
CREATE TABLE photos (
    id INTEGER PRIMARY KEY,
    path TEXT NOT NULL UNIQUE,
    file_name TEXT NOT NULL,
    file_size INTEGER NOT NULL,
    hash TEXT NOT NULL,
    perceptual_hash TEXT,
    created_at TEXT,
    modified_at TEXT,
    date_taken TEXT,
    width INTEGER,
    height INTEGER,
    is_screenshot BOOLEAN DEFAULT 0,
    is_duplicate BOOLEAN DEFAULT 0,
    quality_score REAL,
    INDEX idx_hash (hash),
    INDEX idx_date_taken (date_taken)
);

CREATE TABLE faces (
    id INTEGER PRIMARY KEY,
    photo_id INTEGER,
    embedding BLOB,  -- Face embedding vector
    person_id INTEGER,
    FOREIGN KEY (photo_id) REFERENCES photos(id)
);

CREATE TABLE duplicates (
    id INTEGER PRIMARY KEY,
    photo1_id INTEGER,
    photo2_id INTEGER,
    similarity REAL,
    FOREIGN KEY (photo1_id) REFERENCES photos(id),
    FOREIGN KEY (photo2_id) REFERENCES photos(id)
);
```

## Frontend Architecture

### Component Structure

```
App
├── Layout
│   ├── Header
│   └── Sidebar
│
└── Pages
    ├── Home (landing page)
    ├── Scan
    │   ├── DirectoryPicker
    │   ├── ScanProgress
    │   └── ScanResults
    ├── Organize
    │   ├── OrganizationPreview
    │   ├── StrategySelector
    │   └── ApplyButton
    ├── Review
    │   ├── PhotoGrid
    │   ├── FilterBar
    │   └── DetailView
    └── Settings
        ├── GeneralSettings
        ├── PrivacySettings
        └── AdvancedSettings
```

### State Management

Using Svelte stores:

```typescript
// stores/scan.ts
export const scanState = writable({
    isScanning: false,
    progress: 0,
    currentFile: '',
    stats: {
        totalPhotos: 0,
        totalVideos: 0,
        screenshots: 0,
        duplicates: 0,
    }
});

// stores/photos.ts
export const photosStore = writable<PhotoMetadata[]>([]);

// stores/filters.ts
export const filtersStore = writable({
    showScreenshots: true,
    showDuplicates: true,
    dateRange: null,
});
```

## IPC Communication

### Frontend → Backend

```typescript
import { invoke } from '@tauri-apps/api/core';

// Scan a directory
const result = await invoke<ScanResult>('scan_directory', {
    path: '/Users/atorrella/Desktop/DCIM'
});

// Get statistics
const stats = await invoke<ScanStats>('get_scan_stats', {
    result: scanResult
});
```

### Backend → Frontend (Events)

```rust
// Emit progress updates
app.emit_all("scan-progress", ScanProgress {
    total_files: 1000,
    processed_files: 500,
    current_file: "IMG_1234.jpg".to_string(),
    percentage: 50.0,
})?;
```

```typescript
// Listen for events
import { listen } from '@tauri-apps/api/event';

await listen('scan-progress', (event) => {
    scanProgress.set(event.payload);
});
```

## Performance Optimizations

### Rust Backend
1. **Parallel Processing**: Use `rayon` for multi-threaded file scanning
2. **Lazy Evaluation**: Stream results instead of loading all in memory
3. **Caching**: Cache file hashes to avoid recalculation
4. **Memory Mapping**: Use `memmap2` for large file operations

### Frontend
1. **Virtual Scrolling**: Only render visible photos in grid
2. **Lazy Loading**: Load thumbnails on demand
3. **Web Workers**: Offload heavy processing
4. **Code Splitting**: Load features on demand

## Security Considerations

### File System Access
- Use Tauri's scope system to limit file access
- Validate all file paths
- Prevent directory traversal attacks

### Data Privacy
- Never send data to external servers
- Use local AI models only
- Clear sensitive data from memory after use
- No analytics or tracking

### Safe Operations
- Always preview before applying changes
- Use atomic file operations
- Maintain operation logs
- Implement undo functionality

## Testing Strategy

### Unit Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_screenshot_detection() {
        let scanner = Scanner::new("test_data");
        assert!(scanner.is_likely_screenshot(
            Path::new("screenshot.png"),
            Some(1080),
            Some(1920)
        ));
    }
}
```

### Integration Tests
- Test full scan → organize workflow
- Test duplicate detection accuracy
- Test undo functionality
- Test large library performance (10,000+ photos)

### Manual Testing Checklist
- [ ] Scan completes successfully
- [ ] Progress updates are accurate
- [ ] Statistics are correct
- [ ] Preview shows expected results
- [ ] Organization creates correct structure
- [ ] Undo reverts all changes
- [ ] App handles errors gracefully

## Error Handling

### Rust Backend
```rust
#[derive(Debug, thiserror::Error)]
pub enum PhotoArchiveError {
    #[error("Failed to read file: {0}")]
    FileReadError(#[from] std::io::Error),
    
    #[error("Invalid image format: {0}")]
    ImageError(#[from] image::ImageError),
    
    #[error("Database error: {0}")]
    DatabaseError(String),
}
```

### Frontend
```typescript
try {
    const result = await invoke('scan_directory', { path });
} catch (error) {
    console.error('Scan failed:', error);
    showErrorNotification('Failed to scan directory');
}
```

## Future Architecture Enhancements

### Cloud Sync (Optional, End-to-End Encrypted)
```
Local App ←→ [E2E Encryption] ←→ Cloud Storage
                                   (Encrypted Blobs Only)
```

### Plugin System
```rust
pub trait PhotoArchivePlugin {
    fn name(&self) -> &str;
    fn process_photo(&self, photo: &PhotoMetadata) -> Result<()>;
}
```

### Mobile Companion App
- View organized photos
- Remote triggers for scans
- Share photos with family
- End-to-end encrypted sync

---

This architecture is designed for:
- **Privacy**: Local processing, no cloud
- **Performance**: Fast, efficient, scalable
- **Safety**: Non-destructive, preview-first
- **Extensibility**: Plugin system, modular design
