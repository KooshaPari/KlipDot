# KlipDot - Universal Terminal Image Interceptor

A high-performance, universal terminal image interceptor that automatically captures, processes, and replaces image interactions with file paths across all CLI/TUI applications. Built with Rust for maximum performance and reliability.

## Demonstrations

See KlipDot in action with real functionality demonstrations:

<div align="center">

### 🖼️ Terminal Image Preview (chafa + timg)
<img src="https://raw.githubusercontent.com/KooshaPari/KlipDot/main/demos/terminal-preview.gif" width="800" alt="KlipDot Terminal Image Preview">

### 🔧 Working Features Overview
<img src="https://raw.githubusercontent.com/KooshaPari/KlipDot/main/demos/working-features.gif" width="800" alt="KlipDot Working Features">

### ⚡ Real Preview System
<img src="https://raw.githubusercontent.com/KooshaPari/KlipDot/main/demos/real-preview.gif" width="800" alt="KlipDot Real Preview Demo">

### 📋 Clipboard Workflow
<img src="https://raw.githubusercontent.com/KooshaPari/KlipDot/main/demos/clipboard-workflow.gif" width="800" alt="KlipDot Clipboard Workflow">

</div>

📁 **[View all demonstrations →](demos/)**

## 🖥️ Terminal Image Display

KlipDot supports multiple terminal image display methods:

- **chafa**: High-quality ASCII art conversion
- **timg**: Advanced terminal graphics with Sixel support  
- **qlmanage**: macOS QuickLook integration
- **Image info**: File dimensions, size, and metadata display

### Quick Terminal Preview Example

```bash
# Load ZSH integration
source ~/.klipdot/zsh-preview-integration.zsh

# Quick preview any image
klipdot_quick_preview ~/.klipdot/screenshots/demo.png

# Output:
# 📸 demo.png
# 📏 Size: 462.9 KB  
# 🖼️ Dimensions: 1216x1320
# 📁 /Users/you/.klipdot/screenshots/demo.png
```

## Features

- **🎯 Universal Compatibility**: Works with any CLI/TUI application, terminal emulator, or shell
- **🚀 High Performance**: Rust-based core with sub-second response times
- **📋 Comprehensive Interception**: Monitors clipboard, file operations, drag & drop, STDIN, and process output
- **🔄 Real-time Processing**: Event-driven architecture with continuous monitoring
- **💾 Smart Storage**: Organized file system with automatic cleanup and history management
- **🛠️ Shell Integration**: Deep ZSH and Bash hooks for seamless terminal integration
- **⚙️ Configurable**: Extensive configuration options for all aspects of operation
- **🔒 Secure**: Local-only processing with no network calls
- **📊 Performance Optimized**: Memory-efficient with configurable resource limits
- **🌐 Cross-Platform**: Full support for macOS, Linux, and Windows

## Installation

### Binary Installation

```bash
# Build the klipdot binary
cargo build --release

# Install to user PATH (recommended)
mkdir -p ~/bin
cp target/release/klipdot ~/bin/
chmod +x ~/bin/klipdot

# Or install system-wide
sudo cp target/release/klipdot /usr/local/bin/

# Verify installation
klipdot --version
klipdot --help
```

### Complete Setup

```bash
# Clone repository
git clone https://github.com/your-repo/klipdot.git
cd klipdot

# Install dependencies and setup
cargo build --release
./install.sh

# Start the service
klipdot start --daemon
```

## Usage

### Basic Commands

```bash
# Start the image interceptor
klipdot start

# Start as background daemon
klipdot start --daemon

# Check status and recent screenshots
klipdot status

# List recent screenshots
klipdot list --recent 10

# Clean up old screenshots
klipdot cleanup --days 30

# Show configuration
klipdot config show

# Update configuration
klipdot config set max_file_size 20MB

# Show help
klipdot help
```

### Service Management

```bash
# Service control
klipdot service start
klipdot service stop
klipdot service restart
klipdot service status

# View logs
klipdot logs --tail 50

# Enable auto-start
klipdot service enable
```

## Usage Examples

### With Popular CLI Tools

```bash
# Vim/Neovim - paste image while editing
# KlipDot automatically converts clipboard images to file paths
vim document.md
# In insert mode: Cmd+V → gets "/Users/you/.klipdot/screenshots/image-2024-01-01-uuid.png"

# Git commit with screenshot
git add .
git commit -m "Add screenshot: $(pbpaste)"  # Auto-converts to file path

# Markdown files
echo "![Screenshot]($(pbpaste))" >> README.md

# Terminal file managers (ranger, lf, etc.)
# Images are automatically intercepted during drag & drop operations

# Image processing tools
convert $(pbpaste) -resize 50% output.png
```

### With TUI Applications

```bash
# Emacs with image support
emacs document.org
# Insert image: C-c C-l → automatic file path insertion

# Terminal browsers (w3m, lynx)
# Images automatically processed and linked

# Note-taking apps (nb, joplin-terminal)
nb add "Meeting notes with diagram: $(pbpaste)"
```

## Configuration

### Main Configuration File

KlipDot creates and manages `~/.klipdot/config.json`:

```json
{
  "enabled": true,
  "autoStart": false,
  "daemon": {
    "enabled": false,
    "pidFile": "~/.klipdot/klipdot.pid",
    "logFile": "~/.klipdot/klipdot.log"
  },
  "interception": {
    "clipboard": true,
    "fileOperations": true,
    "dragDrop": true,
    "stdin": true,
    "processMonitoring": true
  },
  "storage": {
    "directory": "~/.klipdot/screenshots",
    "maxFileSize": "10MB",
    "compressionQuality": 90,
    "retentionDays": 30,
    "autoCleanup": true
  },
  "imageFormats": ["png", "jpg", "jpeg", "gif", "bmp", "webp", "svg"],
  "performance": {
    "clipboardPollInterval": 1000,
    "fileWatchInterval": 500,
    "processPollInterval": 5000,
    "maxConcurrentProcessing": 4
  },
  "security": {
    "allowExternalAccess": false,
    "restrictedPaths": [],
    "maxImageSize": "50MB"
  }
}
```

### Configuration Commands

```bash
# View current configuration
klipdot config show

# Edit configuration
klipdot config edit

# Set specific values
klipdot config set storage.maxFileSize 20MB
klipdot config set performance.clipboardPollInterval 500

# Reset to defaults
klipdot config reset

# Validate configuration
klipdot config validate
```

## Shell Integration

### ZSH Setup

KlipDot automatically installs ZSH hooks during setup:

```bash
# ~/.zshrc additions (automatic)
source ~/.klipdot/hooks/zsh-integration.zsh

# Manual setup if needed
echo 'source ~/.klipdot/hooks/zsh-integration.zsh' >> ~/.zshrc
```

### Bash Setup

```bash
# ~/.bashrc additions (automatic)
source ~/.klipdot/hooks/bash-integration.bash

# Manual setup if needed
echo 'source ~/.klipdot/hooks/bash-integration.bash' >> ~/.bashrc
```

### Shell Features

```bash
# Enhanced aliases with image interception
alias cp='klipdot_cp'    # Intercepts image copies
alias mv='klipdot_mv'    # Intercepts image moves
alias scp='klipdot_scp'  # Intercepts secure copies

# Command hooks
preexec_klipdot()  # Before command execution
precmd_klipdot()   # After command completion

# Utility functions
klipdot_handle_image()  # Process image files
klipdot_check_paste()   # Check clipboard for images
```

## Service Management

### Daemon Mode

```bash
# Start as daemon
klipdot start --daemon

# Check daemon status
klipdot service status

# View daemon logs
klipdot logs --follow

# Stop daemon
klipdot service stop
```

### Auto-Start Configuration

```bash
# Enable auto-start on login
klipdot service enable

# Disable auto-start
klipdot service disable

# Check auto-start status
klipdot service status --auto-start
```

### Service Scripts

For manual service management:

```bash
# Create systemd service (Linux)
klipdot service install --systemd

# Create launchd service (macOS)
klipdot service install --launchd

# Create Windows service
klipdot service install --windows
```

## Platform-Specific Setup

### macOS

```bash
# Install dependencies
brew install fswatch

# Grant accessibility permissions
# System Preferences → Security & Privacy → Privacy → Accessibility → Add Terminal

# Install KlipDot
cargo build --release && ./install.sh

# Start service
klipdot start --daemon
```

### Linux

```bash
# Install dependencies (Ubuntu/Debian)
sudo apt-get install inotify-tools xclip file

# Install dependencies (Red Hat/Fedora)
sudo yum install inotify-tools xclip file

# Install dependencies (Arch)
sudo pacman -S inotify-tools xclip file

# Build and install
cargo build --release && ./install.sh
```

### Windows

```bash
# Install via PowerShell
# Dependencies are built-in on Windows 10+

# Build and install
cargo build --release
.\install.ps1

# Or use pre-built binary
# Download from releases page
```

## Performance and Reliability

### Performance Metrics

- **Clipboard Monitoring**: 1000ms intervals (configurable)
- **File System Monitoring**: Real-time event-driven
- **Process Monitoring**: 5000ms intervals (configurable)
- **Image Processing**: ~50ms per image (varies by size)
- **Memory Usage**: <50MB steady state
- **CPU Usage**: <1% during idle monitoring

### Reliability Features

```bash
# Health checks
klipdot health check

# Performance monitoring
klipdot stats --live

# Resource usage
klipdot stats --resources

# Error recovery
klipdot recover --auto-restart
```

### Optimization Settings

```json
{
  "performance": {
    "clipboardPollInterval": 1000,
    "fileWatchInterval": 500,
    "processPollInterval": 5000,
    "maxConcurrentProcessing": 4,
    "imageCompressionLevel": 6,
    "cacheSize": "100MB",
    "enablePreemptiveProcessing": true
  }
}
```

## Troubleshooting

### Common Issues

#### Permission Denied

```bash
# Check permissions
klipdot doctor --permissions

# Fix common permission issues
klipdot fix --permissions

# Manual permission fixes
chmod 755 ~/.klipdot
chmod 644 ~/.klipdot/config.json
```

#### Missing Dependencies

```bash
# Check system dependencies
klipdot doctor --dependencies

# Install missing dependencies
klipdot install --dependencies

# Platform-specific fixes
# macOS: brew install fswatch
# Linux: sudo apt-get install inotify-tools xclip
# Windows: Usually no additional dependencies needed
```

#### Performance Issues

```bash
# Run performance diagnostics
klipdot doctor --performance

# Optimize settings
klipdot optimize --auto

# Manual performance tuning
klipdot config set performance.clipboardPollInterval 2000
klipdot config set performance.maxConcurrentProcessing 2
```

#### Service Not Starting

```bash
# Diagnose service issues
klipdot doctor --service

# Check logs
klipdot logs --tail 100

# Restart service
klipdot service restart

# Reset service configuration
klipdot service reset
```

### Debug Mode

```bash
# Enable debug logging
klipdot start --debug

# Enable verbose logging
klipdot start --verbose

# Enable trace logging
klipdot start --trace

# Log to file
klipdot start --log-file ~/.klipdot/debug.log
```

### Diagnostic Tools

```bash
# Run full system diagnostic
klipdot doctor

# Check specific components
klipdot doctor --clipboard
klipdot doctor --filesystem
klipdot doctor --shell-integration

# Generate diagnostic report
klipdot doctor --report > klipdot-diagnostics.txt
```

## Directory Structure

```
~/.klipdot/
├── screenshots/                     # Stored screenshots
│   ├── clipboard-2024-01-01-uuid.png
│   ├── terminal-2024-01-01-uuid.png
│   ├── dragdrop-2024-01-01-uuid.png
│   └── stdin-2024-01-01-uuid.png
├── hooks/                          # Shell integration files
│   ├── zsh-integration.zsh
│   ├── bash-integration.bash
│   └── common-functions.sh
├── temp/                           # Temporary processing files
├── logs/                           # Log files
│   ├── klipdot.log
│   └── error.log
├── config.json                     # Main configuration
├── klipdot.pid                     # Process ID file
└── service.json                    # Service configuration
```

## API Reference

### Command Line Interface

```bash
# Global options
klipdot [OPTIONS] <COMMAND>

Options:
  -c, --config <FILE>     Use custom config file
  -v, --verbose          Enable verbose output
  -q, --quiet            Suppress output
  -h, --help             Show help
  -V, --version          Show version

Commands:
  start                  Start image interceptor
  stop                   Stop image interceptor
  status                 Show status
  list                   List screenshots
  cleanup                Clean up old files
  config                 Configuration management
  service                Service management
  doctor                 Run diagnostics
  logs                   View logs
  help                   Show help
```

### Configuration API

```bash
# Configuration commands
klipdot config show                 # Show current config
klipdot config edit                 # Edit config file
klipdot config set <key> <value>    # Set config value
klipdot config get <key>            # Get config value
klipdot config reset                # Reset to defaults
klipdot config validate             # Validate config
```

### Service API

```bash
# Service commands
klipdot service start              # Start service
klipdot service stop               # Stop service
klipdot service restart            # Restart service
klipdot service status             # Show service status
klipdot service enable             # Enable auto-start
klipdot service disable            # Disable auto-start
klipdot service install            # Install system service
klipdot service uninstall          # Uninstall system service
```

## Security Considerations

### Data Privacy

- **Local Processing Only**: All image processing occurs locally
- **No Network Transmission**: No data sent to external servers
- **Secure Storage**: Images stored with restricted permissions
- **Automatic Cleanup**: Configurable retention policies

### File System Security

```bash
# Secure file permissions
chmod 700 ~/.klipdot/                    # Directory access restricted to user
chmod 600 ~/.klipdot/config.json         # Config file protected
chmod 644 ~/.klipdot/screenshots/*.png   # Screenshots readable by user
```

### Access Control

```json
{
  "security": {
    "allowExternalAccess": false,
    "restrictedPaths": [
      "/etc",
      "/var",
      "/tmp"
    ],
    "maxImageSize": "50MB",
    "allowedFormats": ["png", "jpg", "jpeg", "gif"],
    "enableFileValidation": true
  }
}
```

## Advanced Usage

### Custom Hooks

```bash
# Custom pre-processing hook
klipdot hooks add pre-process ~/.klipdot/hooks/custom-pre.sh

# Custom post-processing hook
klipdot hooks add post-process ~/.klipdot/hooks/custom-post.sh

# List hooks
klipdot hooks list

# Remove hooks
klipdot hooks remove pre-process
```

### Integration with Other Tools

```bash
# Integration with image optimization tools
klipdot config set processing.postProcessCommand "optipng -o7"

# Integration with cloud storage
klipdot config set storage.syncCommand "rsync -av ~/.klipdot/screenshots/ user@server:/backup/"

# Integration with notification systems
klipdot config set notifications.command "notify-send 'Image processed: %s'"
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests for new functionality
5. Run the test suite: `cargo test`
6. Submit a pull request

## License

MIT License - see LICENSE file for details.

## Support

- GitHub Issues: [Report bugs and request features](https://github.com/your-repo/klipdot/issues)
- Documentation: [Full documentation](https://github.com/your-repo/klipdot/wiki)
- Community: [Discussions](https://github.com/your-repo/klipdot/discussions)