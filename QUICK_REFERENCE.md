# 🚀 Miktos Kosmos - Quick Reference

## ⚡ Installation (First Time)

```bash
cd /Users/atorrella/Desktop/Miktos\ Kosmos
pnpm install
pnpm tauri dev
```

## 📋 Daily Development Commands

```bash

# Start development server

pnpm tauri dev

# Build for production

pnpm tauri build

# Format Rust code

cd src-tauri && cargo fmt && cd ..

# Check for errors

cd src-tauri && cargo clippy && cd ..

# Run tests

cd src-tauri && cargo test && cd ..
```

## 📂 Key Files to Edit

### Frontend

- `src/routes/+page.svelte` - Main page
- `src/lib/components/` - Components (create as needed)
- `src/app.css` - Global styles

### Backend

- `src-tauri/src/scanner.rs` - File scanning
- `src-tauri/src/commands.rs` - API endpoints
- `src-tauri/src/types.rs` - Data structures

## 🎯 Week 1 Checklist

- [ ] Run `pnpm install`
- [ ] Run `pnpm tauri dev`
- [ ] Verify app launches
- [ ] Add EXIF extraction
- [ ] Build scan progress UI
- [ ] Create photo grid
- [ ] Test with DCIM folder

## 📚 Documentation

1. **Start Here**: `GETTING_STARTED.md`
2. **Development**: `DEVELOPMENT.md`
3. **Architecture**: `ARCHITECTURE.md`
4. **Roadmap**: `ROADMAP.md`

## 🆘 Troubleshooting

#### Error: "command not found: tauri"

```bash
pnpm install
```

#### Error: Rust compilation fails

```bash
cd src-tauri
cargo clean
cargo build
```

#### Error: Port 5173 in use

```bash
lsof -ti:5173 | xargs kill -9
```

## 🎓 Learning Resources

- Tauri: <https://tauri.app/>
- SvelteKit: <https://kit.svelte.dev/>
- Rust: <https://doc.rust-lang.org/book/>

## 💡 Philosophy

1. Privacy First
2. Fast & Efficient
3. Safe by Default
4. Simple UI
5. Quality > Features

---

#### Keep this file open while developing!
