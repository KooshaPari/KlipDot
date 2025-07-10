# KlipDot Demonstrations

This directory contains demonstration materials for KlipDot's features.

## VHS Tape Files

Use [VHS](https://github.com/charmbracelet/vhs) to generate GIF demonstrations:

```bash
# Install VHS
brew install vhs

# Generate basic preview demo
vhs demo-basic-preview.tape

# Generate TUI integration demo  
vhs demo-tui-integration.tape

# Generate live preview demo
vhs demo-live-preview.tape
```

## Demo Scripts

### 1. Basic Preview (`demo-basic-preview.tape`)
- Image preview with file info and dimensions
- Image detection in command output
- ZSH integration functions
- Quick preview functionality

### 2. TUI Integration (`demo-tui-integration.tape`)
- TUI application monitoring
- File manager integration
- Enhanced command aliases
- Live preview mode

### 3. Live Preview (`demo-live-preview.tape`)
- LSP-style real-time previews
- Stdin image data handling
- Auto-detection features
- Keybinding demonstrations

## Running Live Demos

### Basic Features
```bash
# Quick image preview
klipdot preview [image_path]

# Monitor command output
ls *.png | klipdot monitor-output

# ZSH integration
source ~/.klipdot/zsh-preview-integration.zsh
klipdot_quick_preview [image_path]
```

### TUI Integration
```bash
# Run TUI with monitoring
klipdot tui ls ~/Pictures/
klipdot tui ranger ~/Downloads/

# Enhanced aliases
tuiimg vim notes.md
rangerimg ~/Pictures/
```

### Advanced Features
```bash
# Live preview mode
klipdot live-preview --auto-preview

# Stdin data preview
cat image.png | klipdot preview-stdin

# Alt+I keybinding for cursor preview
# (Type image path and press Alt+I)
```

## Feature Highlights

- ✅ **Apple Terminal Support**: Native qlmanage integration
- ✅ **Non-blocking Previews**: Quick info + background app launch
- ✅ **TUI-Aware Monitoring**: 15+ applications supported
- ✅ **Real-time Detection**: Stdout/stdin monitoring
- ✅ **Smart Fallbacks**: Always shows useful information
- ✅ **macOS Integration**: Native tools (sips, stat, qlmanage)

## Demo Requirements

- macOS (tested on macOS 14+)
- Terminal.app or compatible terminal
- KlipDot installed in PATH
- ZSH shell (for integration features)
- Optional: VHS for generating GIFs