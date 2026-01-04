# Miktos Kosmos

> Privacy-first family photo organizer. Transform 10 years of digital chaos into a beautifully organized family archive.

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Built with Tauri](https://img.shields.io/badge/Built%20with-Tauri-24C8DB?logo=tauri)](https://tauri.app/)
[![Svelte](https://img.shields.io/badge/Svelte-4.0-FF3E00?logo=svelte)](https://svelte.dev/)

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

### Smart Detection (Local AI)

- **Face Detection** - Group photos by family members
- **Screenshot Detection** - Separate screenshots from real photos
- **Duplicate Detection** - Find exact and near-duplicates across sources
- **Quality Scoring** - Identify blurry, dark, or low-quality photos

### Multi-Source Intelligence

- Merge photos from multiple sources without duplicates
- Handle different filename formats (WhatsApp, Facebook, iPhone, etc.)
- Preserve the best version when duplicates exist
- Smart date extraction from EXIF and filenames

### Organization Options

- Date-based folders (Year/Month or Year/Month/Day)
- Event-based organization
- People-based organization
- Custom folder structures

### Safe & Reversible

- Non-destructive preview mode
- Copy by default (never moves originals)
- Undo functionality
- Detailed operation logs

## 📦 Installation

### macOS

```bash
# Download the latest .dmg from Releases
# Or install via Homebrew
brew install photoarchive
```

### Windows

```bash
# Download the latest .msi from Releases
# Or install via winget
winget install photoarchive
```

### Linux

```bash
# Download the latest .appImage from Releases
# Or install via package manager
sudo apt install photoarchive
```

## 🏃 Quick Start

1. **Launch PhotoArchive**
2. **Select Source Folder** - Point to your messy photo collection
3. **Scan & Analyze** - Let PhotoArchive analyze your photos
4. **Review Results** - See what it found (duplicates, screenshots, etc.)
5. **Preview Organization** - See how your archive will look
6. **Apply** - Create your organized family archive

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

### v1.0 (MVP) - Current Focus
- [x] Project setup and architecture
- [ ] Core scanning engine
- [ ] EXIF extraction
- [ ] Basic duplicate detection

- [ ] Screenshot detection
- [ ] Date-based organization
- [ ] Preview mode
- [ ] Beautiful UI


### v1.1
- [ ] Face detection
- [ ] Quality scoring
- [ ] Advanced duplicate detection
- [ ] Undo functionality


### v2.0
- [ ] Timeline view
- [ ] Search and filtering
- [ ] Batch editing
- [ ] Export options

### Future
- [ ] Mobile companion app
- [ ] End-to-end encrypted cloud sync
- [ ] Shared family albums
- [ ] AI-powered organization suggestions

## 📄 License

MIT License - see [LICENSE](LICENSE) for details.

## 🙏 AcknowledgmentsLegnaPetiteTour/Miktos-Kosmos/issues)
- Discussions: [Join the conversation](https://github.com/LegnaPetiteTour/Miktos-Kosmos
- ExifTool by Phil Harvey
- Tauri team for the amazing framework
- Svelte community
- All contributors

## 📧 Contact

- GitHub Issues: [Report a bug or request a feature](https://github.com/yourusername/photoarchive/issues)
- Discussions: [Join the conversation](https://github.com/yourusername/photoarchive/discussions)

---

**Built with ❤️ for families who want to preserve their memories without sacrificing privacy.**
