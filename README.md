# 🚀 Update - System Update Manager

<div align="center">
  <img src="https://img.shields.io/badge/Made%20with-Rust-red.svg?style=for-the-badge&logo=rust">
  <img src="https://img.shields.io/badge/License-MIT-blue.svg?style=for-the-badge">
  <img src="https://img.shields.io/badge/Platform-macOS-lightgrey.svg?style=for-the-badge&logo=apple">
  <img src="https://img.shields.io/badge/Terminal-Blazing%20Fast-orange.svg?style=for-the-badge">
</div>

<br>

**Update** is a blazing fast, interactive command-line application written in Rust that streamlines system maintenance tasks. With a beautiful, colorful interface and lightning-fast performance, it makes managing your development environment effortless.

## ✨ Features

- 🎯 **Interactive Multi-Select Menu**: Beautiful terminal UI with intuitive navigation
- 🌈 **Colorful Output**: Vibrant, easy-to-read terminal experience with Unicode symbols
- ⚡ **Blazing Fast**: Built with Rust for maximum performance
- 🔧 **Extensible Architecture**: Easily add new commands and categories
- 📦 **Package Manager Support**: Homebrew, npm, bun, gem, cargo, and more
- 🔗 **Blockchain Tools**: Solana CLI integration
- 🛠️ **Development Tools**: Doom Emacs, repository management
- 🧹 **System Maintenance**: macOS updates and cleanup
- 📊 **Progress Tracking**: Real-time progress bars and status indicators
- 🎨 **Beautiful UI**: Box-drawing characters and emoji icons

## 🖥️ Screenshot

```
    +----------------------------------------------------------+
    |                 UPDATE - System Manager                  |
    |                      Built with Rust                     |
    +----------------------------------------------------------+
    
    Select tasks to execute:
    
     Update Homebrew
     Update npm packages
     Update bun
     Update Ruby gems
     Update Rust/Cargo packages
     Sync and upgrade Doom Emacs
     Set Solana config to mainnet-beta
     Check Solana address
```

## 📋 Supported Commands

### 📦 Package Managers
- **Homebrew**: Update and upgrade all packages
- **npm**: Update packages and show funding info
- **bun**: Update bun itself and upgrade
- **Ruby Gems**: Update all installed gems
- **Cargo**: Update all Rust packages

### 🔧 Development Tools
- **Doom Emacs**: Sync, upgrade, and run doctor
- **Repository Management**: Clone/update projects with dependency installation

### 🌐 Blockchain
- **Solana CLI**: Network configuration, balance checking, airdrops

### 🖥️ System Maintenance
- **macOS Updates**: Install all available system updates
- **System Cleanup**: Clean caches and temporary files

## 🚀 Installation

### Prerequisites
- Rust 1.70+ installed
- macOS (primary support)
- Terminal with Unicode support
- **[Nerd Fonts](https://www.nerdfonts.com/)** installed (required for icons)

#### Installing Nerd Fonts

```bash
# Install via Homebrew
brew tap homebrew/cask-fonts
brew install --cask font-fira-code-nerd-font

# Or install manually from https://www.nerdfonts.com/
# Recommended fonts: FiraCode, JetBrains Mono, or Hack
```

### Build from Source

```bash
git clone https://github.com/z3r0dr34d/Update.git
cd Update
cargo build --release
```

### Quick Install

```bash
# Install directly from source
cargo install --git https://github.com/z3r0dr34d/Update.git

# Or if you have the repo locally
cargo install --path .
```

## 🎮 Usage

### Basic Usage

```bash
# Run the interactive menu
./target/release/update

# Or if installed via cargo
update
```

### Controls

- **Space**: Select/deselect commands
- **Enter**: Execute selected commands (or all if none selected)
- **Ctrl+C**: Quit the application

### Example Workflow

1. 🚀 Launch the application
2. 👀 Browse available commands with beautiful icons
3. ✅ Select desired tasks using spacebar
4. 🔥 Press Enter to execute with real-time progress
5. 🎉 Enjoy blazing fast updates!

## 🔧 Configuration

The application supports JSON configuration for customization:

```json
{
  "app_name": "System Update Manager",
  "version": "0.1.0",
  "author": "z3r0dr34d"
}
```

## 🤝 Contributing

We welcome contributions! Here's how you can help:

1. 🍴 Fork the repository
2. 🌿 Create a feature branch (`git checkout -b feature/amazing-feature`)
3. 💾 Commit your changes (`git commit -m 'Add amazing feature'`)
4. 📤 Push to the branch (`git push origin feature/amazing-feature`)
5. 🔄 Open a Pull Request

### Adding New Commands

```rust
// In src/commands.rs
UpdateCommand::new(
    "Your Command Name",
    "your-shell-command",
    CommandType::Shell,
    "🎯",  // Your emoji
    "Description of what this does",
    "Category"
)
```

## 📈 Performance

- **Startup Time**: < 30ms
- **Memory Usage**: < 5MB
- **Binary Size**: 591KB (optimized with LTO)
- **Execution**: Asynchronous with minimal delays
- **Platform**: Optimized for macOS terminals
- **Compilation**: Link-time optimization enabled
- **Progress**: Real-time with 50ms refresh rate

## 🎨 Design Philosophy

- **Speed First**: Every operation is optimized for performance
- **Beautiful UX**: Terminal applications should be visually appealing
- **Extensible**: Easy to add new commands and features
- **Reliable**: Robust error handling and user feedback

## 🛣️ Roadmap

- [ ] 🐧 Linux support
- [ ] 🪟 Windows support
- [ ] 📱 Configuration file management
- [ ] 🔄 Automatic update checking
- [ ] 📊 Command execution history
- [ ] 🎨 Customizable themes
- [ ] 🔌 Plugin system

## 🐛 Known Issues

- Some Unicode characters may not display properly in certain terminals
- Requires sudo permissions for system updates

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 💖 Support

If you find this project helpful, please consider:
- ⭐ Starring the repository
- 🐛 Reporting issues
- 💡 Suggesting new features
- 🔄 Sharing with fellow developers

---

<div align="center">
  <strong>Made with ❤️ and ⚡ by z3r0dr34d</strong>
</div>
