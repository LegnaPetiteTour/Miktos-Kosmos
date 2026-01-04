# Contributing to PhotoArchive

Thank you for your interest in contributing to PhotoArchive! This project aims to help people organize their family photos with privacy and simplicity in mind.

## 🎯 Project Philosophy

1. **Privacy First** - All processing must happen locally, no cloud uploads
2. **Simple & Beautiful** - Clean UI, not enterprise software
3. **Safe by Default** - Non-destructive operations, preview everything
4. **Quality Over Features** - Better to do one thing perfectly than many things poorly
5. **Real Users First** - Features should solve actual problems people have

## 🚀 Getting Started

### Prerequisites
- Node.js 18+ and pnpm
- Rust 1.70+
- Git

### Setup Development Environment
```bash
# Clone your fork
git clone https://github.com/YOUR_USERNAME/photoarchive.git
cd photoarchive

# Install dependencies
pnpm install

# Run in dev mode
pnpm tauri dev
```

## 📋 How to Contribute

### Reporting Bugs
- Use the GitHub issue tracker
- Include OS version, PhotoArchive version, and steps to reproduce
- Add screenshots if relevant
- Check if the issue already exists

### Suggesting Features
- Open a discussion first for major features
- Explain the problem you're trying to solve
- Consider if it fits the project philosophy
- Provide use cases

### Submitting Code
1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Test thoroughly
5. Commit with clear messages (`git commit -m 'Add amazing feature'`)
6. Push to your fork (`git push origin feature/amazing-feature`)
7. Open a Pull Request

## 🏗️ Architecture Overview

### Frontend (SvelteKit)
- `/src/routes` - Pages and routing
- `/src/lib/components` - Reusable components
- `/src/lib/stores` - State management
- `/src/lib/utils` - Helper functions

### Backend (Rust)
- `/src-tauri/src/main.rs` - Entry point
- `/src-tauri/src/scanner.rs` - File scanning
- `/src-tauri/src/exif.rs` - EXIF extraction
- `/src-tauri/src/duplicates.rs` - Duplicate detection
- `/src-tauri/src/db.rs` - SQLite operations

## ✅ Code Standards

### Rust
- Run `cargo fmt` before committing
- Run `cargo clippy` and fix warnings
- Add tests for new functionality
- Document public APIs

### TypeScript/Svelte
- Use TypeScript for type safety
- Follow existing code style
- Keep components small and focused
- Add comments for complex logic

### Git Commits
- Use present tense ("Add feature" not "Added feature")
- First line: short summary (50 chars max)
- Blank line, then detailed explanation if needed
- Reference issues/PRs when relevant

## 🧪 Testing

```bash
# Run Rust tests
cd src-tauri
cargo test

# Run frontend tests (when available)
pnpm test

# Manual testing checklist
- [ ] Scans complete successfully
- [ ] Preview shows accurate results
- [ ] Organization works as expected
- [ ] UI is responsive
- [ ] No console errors
```

## 🐛 Debugging

### Enable Debug Logs
```bash
# Rust backend
RUST_LOG=debug pnpm tauri dev

# Frontend
# Check browser console in DevTools
```

### Common Issues
- **Build fails**: Try `rm -rf node_modules && pnpm install`
- **Tauri errors**: Check Rust version with `rustc --version`
- **Permission issues**: Verify folder permissions

## 📝 Documentation

- Update README.md for user-facing changes
- Add inline comments for complex code
- Update API documentation for new functions
- Include examples where helpful

## 🎨 Design Guidelines

- Follow existing UI patterns
- Use Tailwind CSS utilities
- Maintain consistency
- Test on different screen sizes
- Consider accessibility

## ⚡ Performance

- Profile before optimizing
- Avoid blocking the main thread
- Use Rust for heavy operations
- Lazy load when possible
- Test with large photo libraries (10,000+ photos)

## 🔒 Security

- Never expose file system paths unnecessarily
- Validate all user inputs
- Use safe Rust practices
- Report security issues privately

## 💬 Communication

- Be respectful and constructive
- Ask questions if unclear
- Help others when you can
- Provide context in discussions

## 📜 License

By contributing, you agree that your contributions will be licensed under the MIT License.

## 🙏 Recognition

Contributors will be added to:
- README.md acknowledgments
- GitHub contributors page
- Release notes for significant contributions

---

**Thank you for making PhotoArchive better for everyone!** 🎉
