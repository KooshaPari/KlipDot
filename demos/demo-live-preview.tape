# VHS tape file for demonstrating KlipDot live preview functionality
# Run with: vhs demo-live-preview.tape

Output demos/live-preview.gif
Set FontSize 14
Set Width 1200
Set Height 600
Set Theme "Catppuccin Mocha"

Type "# KlipDot - Live Preview Demo"
Sleep 1s
Enter

Type "# Demo: LSP-style Real-time Image Detection"
Sleep 1s
Enter

Type "# 1. Load ZSH integration"
Sleep 1s
Enter

Type "source ~/.klipdot/zsh-preview-integration.zsh"
Sleep 500ms
Enter

Type "# 2. Alt+I keybinding demo"
Sleep 1s
Enter

Type "# Type an image path and press Alt+I to preview"
Sleep 1s
Enter

Type "~/.klipdot/screenshots/clipboard-2025-07-10T03-04-14.342Z-ee71d43d.png"
Sleep 1s
# Note: Alt+I would be pressed here in real usage
Type " # <-- Press Alt+I here for instant preview"
Sleep 1s
Enter

Type "# 3. Live preview mode"
Sleep 1s
Enter

Type "echo 'Starting live preview mode...'"
Sleep 500ms
Enter

Type "echo '~/.klipdot/screenshots/test.png' | klipdot live-preview --auto-preview"
Sleep 500ms
Enter
Sleep 3s

Type "# 4. Stdin image data preview"
Sleep 1s
Enter

Type "# Preview image data from stdin"
Sleep 500ms
Enter

Type "echo 'cat image.png | klipdot preview-stdin'"
Sleep 500ms
Enter

Type "# 5. Auto-detection in file operations"
Sleep 1s
Enter

Type "cp ~/.klipdot/screenshots/clipboard-2025-07-10T03-04-14.342Z-ee71d43d.png ~/Pictures/demo.png"
Sleep 500ms
Enter
Sleep 2s

Type "# Image operation detected and previewed!"
Sleep 1s
Enter

Sleep 3s