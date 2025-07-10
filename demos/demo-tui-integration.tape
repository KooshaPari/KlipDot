# VHS tape file for demonstrating KlipDot TUI integration
# Run with: vhs demo-tui-integration.tape

Output demos/tui-integration.gif
Set FontSize 14
Set Width 1200
Set Height 600
Set Theme "Catppuccin Mocha"

Type "# KlipDot - TUI Integration Demo"
Sleep 1s
Enter

Type "# Demo: TUI Applications with Image Monitoring"
Sleep 1s
Enter

Type "# 1. Run ls with TUI monitoring"
Sleep 1s
Enter

Type "klipdot tui ls ~/.klipdot/screenshots/"
Sleep 500ms
Enter
Sleep 3s

Type "# 2. Enhanced file manager aliases"
Sleep 1s
Enter

Type "source ~/.klipdot/zsh-preview-integration.zsh"
Sleep 500ms
Enter

Type "# Available TUI commands:"
Sleep 500ms
Enter

Type "echo 'tuiimg - Any TUI with monitoring'"
Sleep 200ms
Enter

Type "echo 'vimimg - Vim with image detection'"
Sleep 200ms
Enter

Type "echo 'rangerimg - Ranger with previews'"
Sleep 200ms
Enter

Type "# 3. Monitor file operations"
Sleep 1s
Enter

Type "echo 'cp image.png ~/Pictures/' | klipdot monitor-output"
Sleep 500ms
Enter
Sleep 3s

Type "# 4. Live preview mode demo"
Sleep 1s
Enter

Type "echo '~/.klipdot/screenshots/clipboard-2025-07-10T03-04-14.342Z-ee71d43d.png' | klipdot live-preview --auto-preview"
Sleep 500ms
Enter
Sleep 3s

Sleep 2s