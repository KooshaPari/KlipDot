# VHS tape file for demonstrating KlipDot basic preview functionality
# Run with: vhs demo-basic-preview.tape

Output demos/basic-preview.gif
Set FontSize 14
Set Width 1200
Set Height 600
Set Theme "Catppuccin Mocha"

Type "# KlipDot - Universal Terminal Image Interceptor"
Sleep 1s
Enter

Type "# Demo: Basic Image Preview"
Sleep 1s
Enter

Type "# 1. Quick preview with file info"
Sleep 1s
Enter

Type "klipdot preview ~/.klipdot/screenshots/clipboard-2025-07-10T03-04-14.342Z-ee71d43d.png"
Sleep 500ms
Enter
Sleep 3s

Type "# 2. Image detection in ls output"
Sleep 1s
Enter

Type "ls ~/.klipdot/screenshots/ | head -3"
Sleep 500ms
Enter
Sleep 2s

Type "# 3. Monitor command output for images"
Sleep 1s
Enter

Type "echo 'Found image: ~/.klipdot/screenshots/test.png' | klipdot monitor-output"
Sleep 500ms
Enter
Sleep 3s

Type "# 4. ZSH integration - quick preview"
Sleep 1s
Enter

Type "source ~/.klipdot/zsh-preview-integration.zsh"
Sleep 500ms
Enter

Type "klipdot_quick_preview ~/.klipdot/screenshots/clipboard-2025-07-10T03-04-14.342Z-ee71d43d.png"
Sleep 500ms
Enter
Sleep 3s

Sleep 2s