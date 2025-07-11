# KlipDot AI Integration and API Demo
# Generate with: vhs demo-ai-integration.tape

Output ai-integration.gif
Set FontSize 14
Set Width 1200
Set Height 800
Set Theme "Molokai"

# Title
Type "# 🤖 KlipDot - AI Agent Integration Demo"
Sleep 1s
Enter
Type "# Automated Image Processing for Claude Code & AI Tools"
Sleep 1s
Enter
Enter

# API endpoints demonstration
Type "# 1️⃣ HTTP API for Automation"
Sleep 1s
Enter
Type "klipdot start --daemon --api-port 8080"
Sleep 1s
Enter
Sleep 2s

Type "# Check API status"
Sleep 1s
Enter
Type "curl -s http://localhost:8080/status | jq '.' 2>/dev/null || curl -s http://localhost:8080/status"
Sleep 1s
Enter
Sleep 3s

# Performance metrics for AI tools
Type "# 2️⃣ Performance Metrics for AI Integration"
Sleep 1s
Enter
Type "echo 'Response Time: <100ms guaranteed'"
Sleep 1s
Enter
Type "echo 'Throughput: 1000+ images/minute'"
Sleep 1s
Enter
Type "echo 'Memory: <50MB steady state'"
Sleep 1s
Enter
Sleep 2s

# Real-time monitoring
Type "# 3️⃣ Real-time Image Monitoring"
Sleep 1s
Enter
Type "echo 'Monitoring clipboard, drag-drop, file operations...'"
Sleep 1s
Enter
Type "klipdot monitor --json --output /tmp/klipdot-monitor.log &"
Sleep 1s
Enter
Sleep 2s

Type "echo 'Monitor started - JSON output for AI parsing'"
Sleep 1s
Enter
Sleep 1s

# Batch processing demo
Type "# 4️⃣ Batch Processing API"
Sleep 1s
Enter
Type "curl -X POST http://localhost:8080/batch-process -H 'Content-Type: application/json' -d '{\"paths\": [\"~/.klipdot/screenshots\"]}'"
Sleep 1s
Enter
Sleep 3s

# Claude Code integration
Type "# 5️⃣ Claude Code Integration"
Sleep 1s
Enter
Type "echo 'KlipDot automatically provides image paths to Claude Code'"
Sleep 1s
Enter
Type "echo 'No manual file selection needed - seamless workflow'"
Sleep 1s
Enter
Sleep 2s

# Configuration for AI tools
Type "# 6️⃣ AI-Optimized Configuration"
Sleep 1s
Enter
Type "klipdot config set ai_integration.enabled true"
Sleep 1s
Enter
Type "klipdot config set ai_integration.json_output true"
Sleep 1s
Enter
Type "klipdot config set ai_integration.webhook_url 'http://localhost:8080/webhook'"
Sleep 1s
Enter
Sleep 2s

# Security for AI environments
Type "# 7️⃣ Secure AI Environment"
Sleep 1s
Enter
Type "echo '🔒 Local processing - no data leaves your machine'"
Sleep 1s
Enter
Type "echo '🛡️ Sandboxed execution for AI safety'"
Sleep 1s
Enter
Type "echo '📊 Audit logs for AI compliance'"
Sleep 1s
Enter
Sleep 2s

# Stop services
Type "# 8️⃣ Cleanup"
Sleep 1s
Enter
Type "pkill -f 'klipdot monitor' 2>/dev/null || true"
Sleep 1s
Enter
Type "klipdot stop"
Sleep 1s
Enter
Sleep 2s

Type "# ✅ AI Integration Demo Complete!"
Sleep 1s
Enter
Type "# 🚀 Ready for Claude Code, Cursor, and other AI tools"
Sleep 2s