# Development Guide

## Quick Start

### Prerequisites
```bash
# Check versions
node --version   # Should be 18+
rustc --version  # Should be 1.70+
pnpm --version   # Install with: npm install -g pnpm
```

### First Time Setup
```bash
cd PhotoArchive

# Install dependencies
pnpm install

# Run in development mode
pnpm tauri dev
```

## Project Structure

```
PhotoArchive/
├── src/                    # Frontend (SvelteKit)
│   ├── lib/
│   │   ├── components/    # Reusable UI components
│   │   ├── stores/        # State management
│   │   └── utils/         # Helper functions
│   ├── routes/            # Pages and routing
│   │   ├── +layout.svelte
│   │   └── +page.svelte
│   ├── app.css            # Global styles
│   └── app.html           # HTML template
│
├── src-tauri/             # Backend (Rust)
│   ├── src/
│   │   ├── main.rs        # Entry point
│   │   ├── scanner.rs     # File scanning logic
│   │   ├── types.rs       # Data structures
│   │   └── commands.rs    # Tauri commands (RPC)
│   ├── Cargo.toml         # Rust dependencies
│   └── tauri.conf.json    # Tauri configuration
│
├── static/                # Static assets (icons, etc.)
├── tests/                 # Tests
└── docs/                  # Documentation
```

## Development Workflow

### Running the App
```bash
# Development mode (hot reload)
pnpm tauri dev

# Build for production
pnpm tauri build
```

### Code Formatting
```bash
# Format Rust code
cd src-tauri
cargo fmt

# Check for Rust issues
cargo clippy

# Format TypeScript/Svelte
pnpm format
```

### Testing
```bash
# Run Rust tests
cd src-tauri
cargo test

# Run frontend tests (when available)
pnpm test
```

## Architecture

### Frontend → Backend Communication

PhotoArchive uses Tauri's IPC (Inter-Process Communication) system:

**Frontend (TypeScript)**:
```typescript
import { invoke } from '@tauri-apps/api/core';

// Call a Rust command
const result = await invoke('scan_directory', { path: '/some/path' });
```

**Backend (Rust)**:
```rust
#[tauri::command]
async fn scan_directory(path: String) -> Result<ScanResult, String> {
    // Implementation
}
```

### Data Flow

1. **User Action** → UI component
2. **UI Component** → Svelte store (state management)
3. **Store** → Tauri command (invoke)
4. **Rust Backend** → Process request
5. **Response** → Update store
6. **Store** → Update UI

### State Management

We use Svelte stores for state:

```typescript
// lib/stores/scan.ts
import { writable } from 'svelte/store';

export const scanProgress = writable({
    isScanning: false,
    progress: 0,
    currentFile: ''
});
```

## Key Components

### Scanner (Rust)
- Walks directory tree
- Reads file metadata
- Calculates file hashes
- Detects image dimensions
- Identifies screenshots

### UI Components (Svelte)
- File picker
- Progress bar
- Photo grid
- Statistics dashboard
- Settings panel

## Common Tasks

### Adding a New Tauri Command

1. Define the command in `src-tauri/src/commands.rs`:
```rust
#[tauri::command]
pub fn my_new_command(param: String) -> Result<String, String> {
    Ok(format!("Processed: {}", param))
}
```

2. Register it in `src-tauri/src/main.rs`:
```rust
.invoke_handler(tauri::generate_handler![
    greet,
    scan_directory,
    my_new_command  // Add here
])
```

3. Call it from the frontend:
```typescript
const result = await invoke('my_new_command', { param: 'test' });
```

### Adding a New Page

1. Create `src/routes/newpage/+page.svelte`
2. The route will automatically be `/newpage`
3. Add navigation links in other components

### Adding a Dependency

**Frontend**:
```bash
pnpm add package-name
```

**Backend**:
```bash
cd src-tauri
cargo add package-name
```

## Debugging

### Frontend Debugging
- Open DevTools in the app (Ctrl+Shift+I / Cmd+Opt+I)
- Check browser console for errors
- Use Svelte DevTools extension

### Backend Debugging
```bash
# Run with debug output
RUST_LOG=debug pnpm tauri dev

# Use println! or dbg! macro
println!("Debug: {:?}", some_variable);
```

### Performance Profiling
```bash
# Rust
cargo build --release
cargo flamegraph

# Frontend
# Use browser's Performance tab
```

## Build for Distribution

### macOS
```bash
pnpm tauri build
# Output: src-tauri/target/release/bundle/dmg/
```

### Windows
```bash
pnpm tauri build
# Output: src-tauri/target/release/bundle/msi/
```

### Linux
```bash
pnpm tauri build
# Output: src-tauri/target/release/bundle/appimage/
```

## Troubleshooting

### "Command not found: tauri"
```bash
pnpm install
```

### Build errors in Rust
```bash
cd src-tauri
cargo clean
cargo build
```

### Frontend not updating
- Check if dev server is running on port 5173
- Clear browser cache
- Restart `pnpm tauri dev`

## Resources

- [Tauri Documentation](https://tauri.app/)
- [SvelteKit Documentation](https://kit.svelte.dev/)
- [Rust Book](https://doc.rust-lang.org/book/)
- [Project Roadmap](./ROADMAP.md)
- [Contributing Guide](./CONTRIBUTING.md)

---

Questions? Open an issue or discussion on GitHub!
