# KlipDot - Universal Terminal Image Interceptor

A universal terminal image interceptor that automatically intercepts image pastes and file operations, replacing them with file paths for any CLI/TUI application.

## Features

- **Automatic Image Interception**: Monitors clipboard for image pastes
- **File Path Replacement**: Replaces clipboard image with file path
- **User-Level Storage**: Creates `~/.klipdot/screenshots/` directory
- **Cross-Platform Support**: Works on macOS, Windows, and Linux
- **Configurable**: User-configurable settings via JSON config
- **History Management**: Track and cleanup old screenshots
- **CLI Interface**: Easy command-line management
- **Universal Compatibility**: Works with any CLI/TUI application

## Installation

```bash
npm install
./install.sh
```

## Usage

### CLI Commands

```bash
# Start the image interceptor
node src/cli.js start

# Check status and recent screenshots
node src/cli.js status

# Clean up old screenshots (30+ days)
node src/cli.js cleanup

# Show help
node src/cli.js help
```

### Programmatic Usage

```javascript
const KlipDotIntegration = require('./src/klipdot-integration');

const integration = new KlipDotIntegration();
await integration.initialize();

// Handler will now monitor clipboard and process image pastes
```

## How It Works

1. **Monitoring**: Continuously polls the system clipboard for changes
2. **Detection**: Identifies when an image is pasted to the clipboard
3. **Processing**: Saves the image to `~/.klipdot/screenshots/`
4. **Replacement**: Replaces clipboard content with the file path
5. **Integration**: Any CLI/TUI application receives the file path instead of raw image data

## Directory Structure

```
~/.klipdot/
├── screenshots/                     # Stored screenshots
│   ├── screenshot-2024-01-01T10-00-00-abc123.png
│   └── screenshot-2024-01-01T10-05-00-def456.png
└── config.json                     # Configuration file
```

## Configuration

The handler creates a configuration file at `~/.klipdot/config.json`:

```json
{
  "enabled": true,
  "autoStart": false,
  "imageFormats": ["png", "jpg", "jpeg", "gif", "bmp"],
  "maxFileSize": "10MB",
  "compressionQuality": 90,
  "createdAt": "2024-01-01T10:00:00.000Z"
}
```

## Platform-Specific Clipboard Access

### macOS
- Uses `pbpaste` and `pbcopy` commands
- Detects images via `osascript` AppleScript

### Windows
- Uses PowerShell `Get-Clipboard` and `clip` commands
- Handles both text and image clipboard content

### Linux
- Uses `xclip` for clipboard operations
- Requires `xclip` to be installed

## Testing

```bash
npm test
```

## Development

```bash
# Start in development mode with auto-restart
npm run dev
```

## API Reference

### ClipboardHandler

Main class that handles clipboard monitoring and image processing.

#### Constructor Options
- `screenshotDir` - Directory to store screenshots (default: `./screenshots`)
- `enableLogging` - Enable console logging (default: `false`)
- `pollInterval` - Clipboard polling interval in ms (default: `1000`)

#### Methods
- `start()` - Start clipboard monitoring
- `stop()` - Stop clipboard monitoring
- `processImagePaste(imageData)` - Process clipboard image data

### ClaudeCodeClipboardIntegration

High-level integration class for Claude-Code.

#### Methods
- `initialize()` - Initialize directories and start handler
- `createUserDirectories()` - Create required user directories
- `loadConfig()` - Load or create configuration
- `updateConfig(newConfig)` - Update configuration
- `getScreenshotHistory()` - Get list of stored screenshots
- `cleanupOldScreenshots(daysOld)` - Remove old screenshots

## Security Considerations

- Screenshots are stored locally in user's home directory
- No network transmission of clipboard data
- Automatic cleanup of old screenshots
- Configurable file size limits

## Troubleshooting

### Common Issues

1. **Permission denied**: Ensure proper clipboard access permissions
2. **Missing dependencies**: Run `npm install` to install required packages
3. **Platform compatibility**: Check platform-specific clipboard tools are installed

### Debug Mode

Enable logging for troubleshooting:

```javascript
const handler = new ClipboardHandler({
  enableLogging: true
});
```

## License

MIT License - see LICENSE file for details.