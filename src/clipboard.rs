use crate::{config::Config, error::Result, image_processor::ImageProcessor, Error};
use std::time::Duration;
use tokio::time::sleep;
use tracing::{debug, info, warn, error};

pub struct ClipboardMonitor {
    config: Config,
    image_processor: ImageProcessor,
    last_content: Option<String>,
    running: bool,
}

impl ClipboardMonitor {
    pub async fn new(config: Config) -> Result<Self> {
        let image_processor = ImageProcessor::new(config.clone()).await?;
        
        Ok(Self {
            config,
            image_processor,
            last_content: None,
            running: false,
        })
    }
    
    pub async fn run(&mut self) -> Result<()> {
        if !self.config.intercept_methods.clipboard {
            info!("Clipboard monitoring disabled in config");
            return Ok(());
        }
        
        info!("Starting clipboard monitor with {}ms interval", self.config.poll_interval);
        self.running = true;
        
        while self.running {
            if let Err(e) = self.poll_clipboard().await {
                if e.is_recoverable() {
                    warn!("Recoverable clipboard error: {}", e);
                    sleep(Duration::from_millis(self.config.poll_interval * 2)).await;
                } else {
                    error!("Fatal clipboard error: {}", e);
                    return Err(e);
                }
            }
            
            sleep(Duration::from_millis(self.config.poll_interval)).await;
        }
        
        Ok(())
    }
    
    pub fn stop(&mut self) {
        info!("Stopping clipboard monitor");
        self.running = false;
    }
    
    async fn poll_clipboard(&mut self) -> Result<()> {
        let content = self.get_clipboard_content().await?;
        
        if let Some(content) = content {
            if Some(&content) != self.last_content.as_ref() {
                self.handle_clipboard_change(&content).await?;
                self.last_content = Some(content);
            }
        }
        
        Ok(())
    }
    
    async fn handle_clipboard_change(&mut self, content: &str) -> Result<()> {
        debug!("Clipboard content changed");
        
        // Check if content is image data
        if self.is_image_data(content) {
            self.process_clipboard_image(content).await?;
        }
        
        Ok(())
    }
    
    async fn process_clipboard_image(&mut self, content: &str) -> Result<()> {
        info!("Processing clipboard image");
        
        // Convert clipboard content to image data
        let image_data = self.decode_clipboard_image(content)?;
        
        // Process the image
        let file_path = self.image_processor.process_image_data(
            &image_data,
            "clipboard"
        ).await?;
        
        // Replace clipboard content with file path
        self.set_clipboard_content(&file_path.to_string_lossy()).await?;
        
        info!("Clipboard image replaced with file path: {:?}", file_path);
        Ok(())
    }
    
    fn is_image_data(&self, content: &str) -> bool {
        // Check for base64 image data
        if content.starts_with("data:image/") {
            return true;
        }
        
        // Check for binary image signatures
        if let Ok(data) = base64::decode(content) {
            return self.has_image_signature(&data);
        }
        
        false
    }
    
    fn has_image_signature(&self, data: &[u8]) -> bool {
        if data.len() < 8 {
            return false;
        }
        
        // Check for common image signatures
        let signatures = [
            &[0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A], // PNG
            &[0xFF, 0xD8, 0xFF, 0xE0, 0x00, 0x10, 0x4A, 0x46], // JPEG
            &[0x47, 0x49, 0x46, 0x38, 0x39, 0x61, 0x00, 0x00], // GIF
            &[0x42, 0x4D, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], // BMP
            &[0x52, 0x49, 0x46, 0x46, 0x00, 0x00, 0x00, 0x00], // WEBP
        ];
        
        for sig in &signatures {
            if data.starts_with(*sig) {
                return true;
            }
        }
        
        false
    }
    
    fn decode_clipboard_image(&self, content: &str) -> Result<Vec<u8>> {
        if content.starts_with("data:image/") {
            // Handle data URL format
            if let Some(comma_pos) = content.find(',') {
                let base64_data = &content[comma_pos + 1..];
                return base64::decode(base64_data)
                    .map_err(|e| Error::Format(format!("Invalid base64 data: {}", e)));
            }
        }
        
        // Try direct base64 decode
        base64::decode(content)
            .map_err(|e| Error::Format(format!("Failed to decode image data: {}", e)))
    }
    
    // Platform-specific clipboard implementations
    
    #[cfg(target_os = "macos")]
    async fn get_clipboard_content(&self) -> Result<Option<String>> {
        use std::process::Command;
        
        // Try to get text content first
        let output = Command::new("pbpaste")
            .output()
            .map_err(|e| Error::Clipboard(format!("Failed to run pbpaste: {}", e)))?;
        
        if output.status.success() {
            let text = String::from_utf8_lossy(&output.stdout);
            if !text.is_empty() {
                return Ok(Some(text.to_string()));
            }
        }
        
        // Try to get image data
        let output = Command::new("osascript")
            .arg("-e")
            .arg("the clipboard as «class PNGf»")
            .output()
            .map_err(|e| Error::Clipboard(format!("Failed to get image from clipboard: {}", e)))?;
        
        if output.status.success() {
            let hex_data = String::from_utf8_lossy(&output.stdout);
            if !hex_data.is_empty() {
                // Convert hex to base64
                if let Ok(binary_data) = hex::decode(hex_data.trim()) {
                    return Ok(Some(base64::encode(&binary_data)));
                }
            }
        }
        
        Ok(None)
    }
    
    #[cfg(target_os = "macos")]
    async fn set_clipboard_content(&self, content: &str) -> Result<()> {
        use std::process::{Command, Stdio};
        use std::io::Write;
        
        let mut child = Command::new("pbcopy")
            .stdin(Stdio::piped())
            .spawn()
            .map_err(|e| Error::Clipboard(format!("Failed to start pbcopy: {}", e)))?;
        
        if let Some(stdin) = child.stdin.as_mut() {
            stdin.write_all(content.as_bytes())
                .map_err(|e| Error::Clipboard(format!("Failed to write to pbcopy: {}", e)))?;
        }
        
        let status = child.wait()
            .map_err(|e| Error::Clipboard(format!("Failed to wait for pbcopy: {}", e)))?;
        
        if !status.success() {
            return Err(Error::Clipboard("pbcopy failed".to_string()));
        }
        
        Ok(())
    }
    
    #[cfg(target_os = "linux")]
    async fn get_clipboard_content(&self) -> Result<Option<String>> {
        let available_tools = self.config.get_available_clipboard_tools();
        
        if available_tools.is_empty() {
            return Err(Error::Clipboard("No clipboard tools available".to_string()));
        }
        
        // Try each available tool
        for tool in &available_tools {
            if let Ok(content) = self.get_clipboard_with_tool(tool).await {
                return Ok(content);
            }
        }
        
        Ok(None)
    }
    
    #[cfg(target_os = "linux")]
    async fn get_clipboard_with_tool(&self, tool: &str) -> Result<Option<String>> {
        use std::process::Command;
        
        let output = match tool {
            "wl-paste" => {
                // Try text first
                let mut cmd = Command::new("wl-paste");
                cmd.arg("--type").arg("text/plain");
                let text_output = cmd.output().map_err(|e| Error::Clipboard(format!("Failed to run wl-paste: {}", e)))?;
                
                if text_output.status.success() {
                    let content = String::from_utf8_lossy(&text_output.stdout);
                    if !content.is_empty() {
                        return Ok(Some(content.to_string()));
                    }
                }
                
                // Try image data
                let mut cmd = Command::new("wl-paste");
                cmd.arg("--type").arg("image/png");
                cmd.output().map_err(|e| Error::Clipboard(format!("Failed to run wl-paste for image: {}", e)))?
            }
            "xclip" => {
                Command::new("xclip")
                    .arg("-selection")
                    .arg("clipboard")
                    .arg("-o")
                    .output()
                    .map_err(|e| Error::Clipboard(format!("Failed to run xclip: {}", e)))?
            }
            "xsel" => {
                Command::new("xsel")
                    .arg("--clipboard")
                    .arg("--output")
                    .output()
                    .map_err(|e| Error::Clipboard(format!("Failed to run xsel: {}", e)))?
            }
            _ => {
                return Err(Error::Clipboard(format!("Unsupported clipboard tool: {}", tool)));
            }
        };
        
        if output.status.success() {
            let content = String::from_utf8_lossy(&output.stdout);
            if !content.is_empty() {
                // For image data, encode as base64
                if tool == "wl-paste" && !content.starts_with("data:") && !content.chars().all(|c| c.is_ascii_graphic() || c.is_ascii_whitespace()) {
                    // This might be binary image data
                    let base64_content = base64::encode(output.stdout);
                    return Ok(Some(base64_content));
                }
                return Ok(Some(content.to_string()));
            }
        }
        
        Ok(None)
    }
    
    #[cfg(target_os = "linux")]
    async fn set_clipboard_content(&self, content: &str) -> Result<()> {
        let available_tools = self.config.get_available_clipboard_tools();
        
        if available_tools.is_empty() {
            return Err(Error::Clipboard("No clipboard tools available".to_string()));
        }
        
        // Try each available tool
        for tool in &available_tools {
            if let Ok(()) = self.set_clipboard_with_tool(tool, content).await {
                return Ok(());
            }
        }
        
        Err(Error::Clipboard("Failed to set clipboard content with any available tool".to_string()))
    }
    
    #[cfg(target_os = "linux")]
    async fn set_clipboard_with_tool(&self, tool: &str, content: &str) -> Result<()> {
        use std::process::{Command, Stdio};
        use std::io::Write;
        
        let mut child = match tool {
            "wl-copy" => {
                Command::new("wl-copy")
                    .arg("--type")
                    .arg("text/plain")
                    .stdin(Stdio::piped())
                    .spawn()
                    .map_err(|e| Error::Clipboard(format!("Failed to start wl-copy: {}", e)))?
            }
            "xclip" => {
                Command::new("xclip")
                    .arg("-selection")
                    .arg("clipboard")
                    .stdin(Stdio::piped())
                    .spawn()
                    .map_err(|e| Error::Clipboard(format!("Failed to start xclip: {}", e)))?
            }
            "xsel" => {
                Command::new("xsel")
                    .arg("--clipboard")
                    .arg("--input")
                    .stdin(Stdio::piped())
                    .spawn()
                    .map_err(|e| Error::Clipboard(format!("Failed to start xsel: {}", e)))?
            }
            _ => {
                return Err(Error::Clipboard(format!("Unsupported clipboard tool: {}", tool)));
            }
        };
        
        if let Some(stdin) = child.stdin.as_mut() {
            stdin.write_all(content.as_bytes())
                .map_err(|e| Error::Clipboard(format!("Failed to write to {}: {}", tool, e)))?;
        }
        
        let status = child.wait()
            .map_err(|e| Error::Clipboard(format!("Failed to wait for {}: {}", tool, e)))?;
        
        if !status.success() {
            return Err(Error::Clipboard(format!("{} failed", tool)));
        }
        
        Ok(())
    }
    
    #[cfg(target_os = "windows")]
    async fn get_clipboard_content(&self) -> Result<Option<String>> {
        use std::process::Command;
        
        let output = Command::new("powershell")
            .arg("-Command")
            .arg("Get-Clipboard")
            .output()
            .map_err(|e| Error::Clipboard(format!("Failed to run PowerShell: {}", e)))?;
        
        if output.status.success() {
            let content = String::from_utf8_lossy(&output.stdout);
            if !content.is_empty() {
                return Ok(Some(content.to_string()));
            }
        }
        
        Ok(None)
    }
    
    #[cfg(target_os = "windows")]
    async fn set_clipboard_content(&self, content: &str) -> Result<()> {
        use std::process::{Command, Stdio};
        use std::io::Write;
        
        let mut child = Command::new("clip")
            .stdin(Stdio::piped())
            .spawn()
            .map_err(|e| Error::Clipboard(format!("Failed to start clip: {}", e)))?;
        
        if let Some(stdin) = child.stdin.as_mut() {
            stdin.write_all(content.as_bytes())
                .map_err(|e| Error::Clipboard(format!("Failed to write to clip: {}", e)))?;
        }
        
        let status = child.wait()
            .map_err(|e| Error::Clipboard(format!("Failed to wait for clip: {}", e)))?;
        
        if !status.success() {
            return Err(Error::Clipboard("clip failed".to_string()));
        }
        
        Ok(())
    }
}

// Add base64 dependency to Cargo.toml
mod base64 {
    use base64::engine::general_purpose;
    use base64::Engine;
    
    pub fn encode(data: &[u8]) -> String {
        general_purpose::STANDARD.encode(data)
    }
    
    pub fn decode(data: &str) -> Result<Vec<u8>, base64::DecodeError> {
        general_purpose::STANDARD.decode(data)
    }
}

// Add hex dependency to Cargo.toml
mod hex {
    pub fn decode(data: &str) -> Result<Vec<u8>, hex::FromHexError> {
        hex::decode(data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    
    #[tokio::test]
    async fn test_clipboard_monitor_creation() {
        let temp_dir = TempDir::new().unwrap();
        let mut config = Config::default();
        config.screenshot_dir = temp_dir.path().to_path_buf();
        
        let monitor = ClipboardMonitor::new(config).await;
        assert!(monitor.is_ok());
    }
    
    #[tokio::test]
    async fn test_image_signature_detection() {
        let config = Config::default();
        let processor = ImageProcessor::new(config).await.unwrap();
        let monitor = ClipboardMonitor {
            config: Config::default(),
            image_processor: processor,
            last_content: None,
            running: false,
        };
        
        // PNG signature
        let png_data = vec![0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
        assert!(monitor.has_image_signature(&png_data));
        
        // JPEG signature (fixed - need proper JPEG header)
        let jpeg_data = vec![0xFF, 0xD8, 0xFF, 0xE0, 0x00, 0x10, 0x4A, 0x46];
        assert!(monitor.has_image_signature(&jpeg_data));
        
        // Not an image
        let text_data = b"Hello, world!";
        assert!(!monitor.has_image_signature(text_data));
    }
    
    #[tokio::test]
    async fn test_data_url_detection() {
        let config = Config::default();
        let processor = ImageProcessor::new(config).await.unwrap();
        let monitor = ClipboardMonitor {
            config: Config::default(),
            image_processor: processor,
            last_content: None,
            running: false,
        };
        
        let data_url = "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mNkYPhfDwAChAI9jU77UwAAAABJRU5ErkJggg==";
        assert!(monitor.is_image_data(data_url));
        
        let text = "Hello, world!";
        assert!(!monitor.is_image_data(text));
    }
}