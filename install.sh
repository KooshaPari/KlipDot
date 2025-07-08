#!/bin/bash

# Claude-Code Clipboard Handler Installation Script
# Supports macOS and Linux with ZSH/Bash

set -e

CLAUDE_CODE_DIR="$HOME/.claude-code"
INSTALL_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

echo "🚀 Installing Claude-Code Clipboard Handler..."

# Detect OS
OS="$(uname -s)"
case "${OS}" in
    Linux*)     MACHINE=Linux;;
    Darwin*)    MACHINE=Mac;;
    *)          MACHINE="UNKNOWN:${OS}"
esac

echo "📋 Detected OS: $MACHINE"

# Detect Shell
SHELL_TYPE="$(basename "$SHELL")"
echo "🐚 Detected Shell: $SHELL_TYPE"

# Create directories
echo "📁 Creating directories..."
mkdir -p "$CLAUDE_CODE_DIR"
mkdir -p "$CLAUDE_CODE_DIR/hooks"
mkdir -p "$CLAUDE_CODE_DIR/temp"
mkdir -p "$CLAUDE_CODE_DIR/watch"
mkdir -p "$CLAUDE_CODE_DIR/stdin-buffer"
mkdir -p "$CLAUDE_CODE_DIR/clipboard-screenshots"

# Copy source files
echo "📄 Copying source files..."
cp -r "$INSTALL_DIR/src/"* "$CLAUDE_CODE_DIR/"
cp "$INSTALL_DIR/package.json" "$CLAUDE_CODE_DIR/"

# Install dependencies
echo "📦 Installing dependencies..."
cd "$CLAUDE_CODE_DIR"
npm install --production

# Make scripts executable
chmod +x "$CLAUDE_CODE_DIR/terminal-handler.js"
chmod +x "$CLAUDE_CODE_DIR/stdin-wrapper.js"
chmod +x "$CLAUDE_CODE_DIR/cli.js"

# Install shell hooks
echo "🔗 Installing shell hooks..."

if [[ "$SHELL_TYPE" == "zsh" ]]; then
    echo "Installing ZSH hooks..."
    
    # Create ZSH hook file
    cat > "$CLAUDE_CODE_DIR/hooks/zsh-hooks.zsh" << 'EOF'
# Claude-Code Terminal Interceptor ZSH Hooks
CLAUDE_CODE_DIR="$HOME/.claude-code"
CLAUDE_CODE_TEMP="$CLAUDE_CODE_DIR/temp"
CLAUDE_CODE_HANDLER="$CLAUDE_CODE_DIR/terminal-handler.js"

# Function to handle image files
claude_code_handle_image() {
  local file_path="$1"
  if [[ -f "$file_path" ]]; then
    local mime_type=$(file --mime-type -b "$file_path" 2>/dev/null)
    if [[ "$mime_type" =~ ^image/ ]]; then
      node "$CLAUDE_CODE_HANDLER" handle-image "$file_path" 2>/dev/null &
      return $?
    fi
  fi
  return 1
}

# Hook into command execution
preexec_claude_code() {
  local cmd="$1"
  
  # Check for image-related commands
  if [[ "$cmd" =~ (cp|mv|scp|rsync).*\.(png|jpg|jpeg|gif|bmp|webp|svg) ]]; then
    echo "[Claude-Code] Image operation detected"
  fi
  
  # Check for file arguments that might be images
  local args=("${(@s/ /)cmd}")
  for arg in "${args[@]}"; do
    if [[ -f "$arg" ]]; then
      claude_code_handle_image "$arg"
    fi
  done
}

# Hook into command completion
precmd_claude_code() {
  # Check for new files in current directory
  for file in *.{png,jpg,jpeg,gif,bmp,webp,svg}(N); do
    if [[ -f "$file" ]]; then
      claude_code_handle_image "$file"
    fi
  done
}

# Add hooks to ZSH
if [[ -n "$ZSH_VERSION" ]]; then
  autoload -Uz add-zsh-hook
  add-zsh-hook preexec preexec_claude_code
  add-zsh-hook precmd precmd_claude_code
fi

# Enhanced aliases
alias cp='claude_code_cp'
alias mv='claude_code_mv'
alias scp='claude_code_scp'

claude_code_cp() {
  local result
  command cp "$@"
  result=$?
  
  for arg in "$@"; do
    if [[ -f "$arg" ]]; then
      claude_code_handle_image "$arg"
    fi
  done
  
  return $result
}

claude_code_mv() {
  local result
  command mv "$@"
  result=$?
  
  for arg in "$@"; do
    if [[ -f "$arg" ]]; then
      claude_code_handle_image "$arg"
    fi
  done
  
  return $result
}

claude_code_scp() {
  local result
  command scp "$@"
  result=$?
  
  for arg in "$@"; do
    if [[ -f "$arg" ]]; then
      claude_code_handle_image "$arg"
    fi
  done
  
  return $result
}
EOF

    # Add to .zshrc
    if [[ -f "$HOME/.zshrc" ]]; then
        if ! grep -q "source.*claude-code.*zsh-hooks" "$HOME/.zshrc"; then
            echo "" >> "$HOME/.zshrc"
            echo "# Claude-Code Terminal Interceptor" >> "$HOME/.zshrc"
            echo "source \"$CLAUDE_CODE_DIR/hooks/zsh-hooks.zsh\"" >> "$HOME/.zshrc"
            echo "✅ Added hooks to ~/.zshrc"
        else
            echo "✅ Hooks already present in ~/.zshrc"
        fi
    fi

elif [[ "$SHELL_TYPE" == "bash" ]]; then
    echo "Installing Bash hooks..."
    
    # Create Bash hook file
    cat > "$CLAUDE_CODE_DIR/hooks/bash-hooks.bash" << 'EOF'
# Claude-Code Terminal Interceptor Bash Hooks
CLAUDE_CODE_DIR="$HOME/.claude-code"
CLAUDE_CODE_TEMP="$CLAUDE_CODE_DIR/temp"
CLAUDE_CODE_HANDLER="$CLAUDE_CODE_DIR/terminal-handler.js"

# Function to handle image files
claude_code_handle_image() {
  local file_path="$1"
  if [[ -f "$file_path" ]]; then
    local mime_type=$(file --mime-type -b "$file_path" 2>/dev/null)
    if [[ "$mime_type" =~ ^image/ ]]; then
      node "$CLAUDE_CODE_HANDLER" handle-image "$file_path" 2>/dev/null &
      return $?
    fi
  fi
  return 1
}

# Hook into command execution
claude_code_preexec() {
  local cmd="$BASH_COMMAND"
  
  # Check for image-related commands
  if [[ "$cmd" =~ (cp|mv|scp|rsync).*\.(png|jpg|jpeg|gif|bmp|webp|svg) ]]; then
    echo "[Claude-Code] Image operation detected"
  fi
  
  # Check for file arguments that might be images
  for arg in $cmd; do
    if [[ -f "$arg" ]]; then
      claude_code_handle_image "$arg"
    fi
  done
}

# Hook into prompt
claude_code_precmd() {
  # Check for new files in current directory
  for file in *.{png,jpg,jpeg,gif,bmp,webp,svg}; do
    if [[ -f "$file" ]] 2>/dev/null; then
      claude_code_handle_image "$file"
    fi
  done 2>/dev/null
}

# Set up command hooks
trap 'claude_code_preexec' DEBUG
if [[ -z "$PROMPT_COMMAND" ]]; then
  PROMPT_COMMAND="claude_code_precmd"
else
  PROMPT_COMMAND="claude_code_precmd;$PROMPT_COMMAND"
fi

# Enhanced aliases
alias cp='claude_code_cp'
alias mv='claude_code_mv'
alias scp='claude_code_scp'

claude_code_cp() {
  local result
  command cp "$@"
  result=$?
  
  for arg in "$@"; do
    if [[ -f "$arg" ]]; then
      claude_code_handle_image "$arg"
    fi
  done
  
  return $result
}

claude_code_mv() {
  local result
  command mv "$@"
  result=$?
  
  for arg in "$@"; do
    if [[ -f "$arg" ]]; then
      claude_code_handle_image "$arg"
    fi
  done
  
  return $result
}

claude_code_scp() {
  local result
  command scp "$@"
  result=$?
  
  for arg in "$@"; do
    if [[ -f "$arg" ]]; then
      claude_code_handle_image "$arg"
    fi
  done
  
  return $result
}
EOF

    # Add to .bashrc
    if [[ -f "$HOME/.bashrc" ]]; then
        if ! grep -q "source.*claude-code.*bash-hooks" "$HOME/.bashrc"; then
            echo "" >> "$HOME/.bashrc"
            echo "# Claude-Code Terminal Interceptor" >> "$HOME/.bashrc"
            echo "source \"$CLAUDE_CODE_DIR/hooks/bash-hooks.bash\"" >> "$HOME/.bashrc"
            echo "✅ Added hooks to ~/.bashrc"
        else
            echo "✅ Hooks already present in ~/.bashrc"
        fi
    fi
fi

# Install system-level dependencies
echo "🔧 Installing system dependencies..."

if [[ "$MACHINE" == "Mac" ]]; then
    echo "Installing macOS dependencies..."
    
    # Check for Homebrew
    if ! command -v brew &> /dev/null; then
        echo "⚠️  Homebrew not found. Some features may not work."
        echo "   Install Homebrew: /bin/bash -c \"\$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)\""
    else
        # Install fswatch for file monitoring
        if ! command -v fswatch &> /dev/null; then
            echo "Installing fswatch..."
            brew install fswatch
        fi
    fi
    
elif [[ "$MACHINE" == "Linux" ]]; then
    echo "Installing Linux dependencies..."
    
    # Detect package manager
    if command -v apt-get &> /dev/null; then
        echo "Installing dependencies with apt..."
        sudo apt-get update
        sudo apt-get install -y inotify-tools xclip file
    elif command -v yum &> /dev/null; then
        echo "Installing dependencies with yum..."
        sudo yum install -y inotify-tools xclip file
    elif command -v pacman &> /dev/null; then
        echo "Installing dependencies with pacman..."
        sudo pacman -S inotify-tools xclip file
    else
        echo "⚠️  Unknown package manager. Please install manually:"
        echo "   - inotify-tools (for file monitoring)"
        echo "   - xclip (for clipboard access)"
        echo "   - file (for MIME type detection)"
    fi
fi

# Create service script
echo "🛠️  Creating service script..."
cat > "$CLAUDE_CODE_DIR/service.sh" << 'EOF'
#!/bin/bash

# Claude-Code Clipboard Handler Service
CLAUDE_CODE_DIR="$HOME/.claude-code"
PID_FILE="$CLAUDE_CODE_DIR/clipboard-handler.pid"
LOG_FILE="$CLAUDE_CODE_DIR/clipboard-handler.log"

start() {
  if [[ -f "$PID_FILE" ]]; then
    PID=$(cat "$PID_FILE")
    if kill -0 "$PID" 2>/dev/null; then
      echo "Service already running (PID: $PID)"
      return 1
    fi
  fi
  
  echo "Starting Claude-Code Clipboard Handler..."
  cd "$CLAUDE_CODE_DIR"
  nohup node cli.js start > "$LOG_FILE" 2>&1 &
  echo $! > "$PID_FILE"
  echo "Service started (PID: $!)"
}

stop() {
  if [[ -f "$PID_FILE" ]]; then
    PID=$(cat "$PID_FILE")
    if kill -0 "$PID" 2>/dev/null; then
      echo "Stopping service (PID: $PID)..."
      kill "$PID"
      rm -f "$PID_FILE"
      echo "Service stopped"
    else
      echo "Service not running"
      rm -f "$PID_FILE"
    fi
  else
    echo "Service not running"
  fi
}

status() {
  if [[ -f "$PID_FILE" ]]; then
    PID=$(cat "$PID_FILE")
    if kill -0 "$PID" 2>/dev/null; then
      echo "Service running (PID: $PID)"
    else
      echo "Service not running (stale PID file)"
      rm -f "$PID_FILE"
    fi
  else
    echo "Service not running"
  fi
}

case "$1" in
  start)
    start
    ;;
  stop)
    stop
    ;;
  restart)
    stop
    sleep 2
    start
    ;;
  status)
    status
    ;;
  *)
    echo "Usage: $0 {start|stop|restart|status}"
    exit 1
esac
EOF

chmod +x "$CLAUDE_CODE_DIR/service.sh"

# Create CLI wrapper
echo "🔧 Creating CLI wrapper..."
cat > "$HOME/.local/bin/claude-code-clipboard" << EOF
#!/bin/bash
exec "$CLAUDE_CODE_DIR/cli.js" "\$@"
EOF

# Create ~/.local/bin if it doesn't exist
mkdir -p "$HOME/.local/bin"
chmod +x "$HOME/.local/bin/claude-code-clipboard"

# Add to PATH if not already there
if [[ ":$PATH:" != *":$HOME/.local/bin:"* ]]; then
    echo "Adding ~/.local/bin to PATH..."
    if [[ "$SHELL_TYPE" == "zsh" ]]; then
        echo 'export PATH="$HOME/.local/bin:$PATH"' >> "$HOME/.zshrc"
    elif [[ "$SHELL_TYPE" == "bash" ]]; then
        echo 'export PATH="$HOME/.local/bin:$PATH"' >> "$HOME/.bashrc"
    fi
fi

echo ""
echo "✅ Installation complete!"
echo ""
echo "📋 Next steps:"
echo "1. Restart your terminal or run: source ~/.${SHELL_TYPE}rc"
echo "2. Start the service: $CLAUDE_CODE_DIR/service.sh start"
echo "3. Check status: claude-code-clipboard status"
echo ""
echo "📁 Files installed in: $CLAUDE_CODE_DIR"
echo "🔗 Shell hooks installed for: $SHELL_TYPE"
echo "🛠️  Service script: $CLAUDE_CODE_DIR/service.sh"
echo ""
echo "🚀 The clipboard handler will now intercept ALL image interactions:"
echo "   • Clipboard pastes"
echo "   • File operations (cp, mv, scp)"
echo "   • Drag and drop"
echo "   • STDIN image data"
echo "   • Directory monitoring"
echo ""
echo "All images will be stored in: $CLAUDE_CODE_DIR/clipboard-screenshots/"
echo "And replaced with file paths for Claude-Code integration."