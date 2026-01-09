# Miktos Kosmos

> Privacy-first family photo organizer. Transform 10 years of digital chaos into a beautifully organized family archive.

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Built with Tauri](https://img.shields.io/badge/Built%20with-Tauri-24C8DB?logo=tauri)](https://tauri.app/)
[![Svelte](https://img.shields.io/badge/Svelte-4.0-FF3E00?logo=svelte)](https://svelte.dev/)

## ⚠️ ALPHA SOFTWARE - DEVELOPMENT IN PROGRESS

**Miktos Kosmos is in active alpha development.** Currently implemented:
- ✅ Folder scanning and analysis
- ✅ Duplicate detection (exact hash matching)
- ✅ Screenshot identification
- ✅ Quality issue detection (low resolution, small files, missing dates)
- ✅ Real-time scan progress

**Not yet implemented:**
- ❌ File organization/moving (coming soon with safety features)
- ❌ Undo functionality (will be added before any file operations)
- ❌ Preview mode (in development)
- ❌ Face detection (future feature)

**Do not use on irreplaceable photos without backups.** Always test on a copy of your data first.

## 🎯 The Problem

You have thousands of photos scattered across:

- iPhone backups
- WhatsApp exports  
- Google Photos downloads
- Old hard drives
- Facebook archives

They're mixed with screenshots, memes, duplicates, and blurry shots. You want a clean, organized family archive but you're overwhelmed by the chaos.

## ✨ The Solution

Miktos Kosmos is specifically designed to turn messy, multi-source photo libraries into beautifully organized family archives.

**What Makes It Different:**

- 🔒 **Privacy-First** - All processing happens locally, zero cloud uploads
- 🧹 **Smart Cleanup** - Automatically detects screenshots, duplicates, and low-quality photos
- ⚡ **Blazing Fast** - Rust-powered backend processes thousands of photos in minutes
- 🎨 **Beautiful UI** - Modern, simple interface that doesn't feel like enterprise software
- 💾 **Safe by Default** - Preview everything before committing, easy undo
- 🎯 **One Job, Done Right** - Not a universal tool, focused on family archives

## 🚀 Features

### Currently Working

- **Folder Scanning** - Fast recursive scanning of photo directories
- **Screenshot Detection** - Identifies screenshots by filename patterns and dimensions
- **Duplicate Detection** - Finds exact duplicates using SHA256 hashing
- **Quality Analysis** - Detects:
  - Low resolution images (below 1920×1080)
  - Small/compressed files (under 500KB)
  - Missing EXIF metadata
  - Potential memes/downloads (filename patterns)
- **Real-time Progress** - Live updates during scanning
- **Modern UI** - Clean, responsive interface

### In Development

- **Preview Mode** - See proposed changes before applying
- **Safe File Operations** - Copy/move with transaction logs
- **Undo System** - Rollback any changes
- **Date-based Organization** - Organize by Year/Month structure

### Planned Features

- Near-duplicate detection (perceptual hashing)
- Face detection and grouping
- Timeline view
- Advanced filtering and search
- Batch operations

## 📦 Installation

**Miktos Kosmos is currently in alpha and must be built from source.**

### Prerequisites

- Node.js 18+ ([Download](https://nodejs.org/))
- Rust 1.70+ ([Install via rustup](https://rustup.rs/))
- pnpm (`npm install -g pnpm`)

### Build from Source

```bash
# Clone the repository
git clone https://github.com/LegnaPetiteTour/Miktos-Kosmos.git
cd Miktos-Kosmos

# Install dependencies
pnpm install

# Run in development mode
pnpm tauri dev

# Build for production
pnpm tauri build
```

### Future Distribution

Once out of alpha, we'll provide:
- macOS: .dmg installer and Homebrew
- Windows: .msi installer and winget
- Linux: .AppImage, .deb, and .rpm packages

## 🏃 Quick Start

1. **Launch Miktos Kosmos** - Run `pnpm tauri dev`
2. **Select Source Folder** - Click "Scan Folder" and choose a directory
3. **Scan & Analyze** - Watch real-time progress as it analyzes your photos
4. **Review Results** - See summary with:
   - Total files and size
   - File types breakdown
   - Quality issues detected (duplicates, screenshots, low-res, etc.)

**Note:** File organization features are coming soon. Currently, the app only scans and analyzes without modifying files.

## 🛠️ Development

Miktos Kosmos is built with modern technologies:

- **Frontend**: SvelteKit + TypeScript
- **Backend**: Rust + Tauri 2.0
- **Database**: SQLite
- **Image Processing**: ExifTool + Rust image libraries
- **AI Models**: Local ONNX models (no cloud)

### Prerequisites

- Node.js 18+
- Rust 1.70+
- pnpm (or npm)

### Setup

```bash
# Clone the repository
git clone https://github.com/yourusername/photoarchive.git
cd photoarchive

# Install dependencies
pnpm install

# Run in development mode
pnpm tauri dev


```textuild for production
pnpm tauri build
```

### Project Structure

```text
photoarchive/
├── src/              # SvelteKit frontend
│   ├── lib/          # Components and utilities
│   ├── routes/       # Pages
│   └── app.html      # HTML template
├── src-tauri/        # Rust backend
│   ├── src/          # Rust source code
│   ├── Cargo.toml    # Rust dependencies
│   └── tauri.conf.json
├── static/           # Static assets
├── tests/            # Test files
└── docs/             # DoMiktos Kosmos is built to help people, and your help makes it better.

### Ways to Contribute

## 🤝 Contributing

We welcome contributions! PhotoArchive is built to help people, and your help makes it better.

### Ways to Contribute
- 🐛 Report bugs
- 💡 Suggest features
- 📖 Improve documentation
- 🔧 Submit pull requests
- ⭐ Star the project


See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## 🗺️ Roadmap

### v0.3 (Current - Alpha)
- [x] Project setup and architecture
- [x] Core scanning engine with walkdir
- [x] EXIF extraction (dates, dimensions)
- [x] Duplicate detection (exact, via SHA256)
- [x] Screenshot detection
- [x] Quality metrics (low-res, small files, missing metadata)
- [x] Real-time scan progress
- [x] Modern UI with Summary section
- [ ] Preview mode (in progress)
- [ ] Operation logging system

### v0.4 (Next - Safety Infrastructure)
- [ ] SQLite operation log
- [ ] Transaction-based file operations
- [ ] Undo/rollback system
- [ ] Crash recovery
- [ ] Preview mode before applying changes

### v0.5 (File Operations)
- [ ] Copy mode (safe default)
- [ ] Date-based folder organization
- [ ] Move mode (with confirmations)
- [ ] Batch operations with progress

### v1.0 (Public Beta)
- [ ] Near-duplicate detection (perceptual hashing)
- [ ] Configurable quality thresholds
- [ ] Timeline view
- [ ] Search and filtering
- [ ] Comprehensive testing suite

### Future (v2.0+)
- [ ] Face detection and grouping
- [ ] Local ML models for quality scoring
- [ ] Event detection
- [ ] Mobile companion app
- [ ] Optional encrypted backup

## 📄 License

MIT License - see [LICENSE](LICENSE) for details.

## 🙏 Acknowledgments
- ExifTool by Phil Harvey
- Tauri team for the amazing framework
- Svelte community
- All contributors

## 📧 Contact

- GitHub Issues: [Report a bug or request a feature](https://github.com/LegnaPetiteTour/Miktos-Kosmos/issues)
- Discussions: [Join the conversation](https://github.com/LegnaPetiteTour/Miktos-Kosmos/discussions)

---

**Built with ❤️ for families who want to preserve their memories without sacrificing privacy.**
