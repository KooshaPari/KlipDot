use crate::{config::Config, error::Result, Error, image_preview::ImagePreviewManager};
use regex::Regex;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use tokio::sync::mpsc;
use tracing::{debug, info, warn};

/// Monitors stdout/stderr for image paths and automatically shows previews
pub struct StdoutMonitor {
    config: Config,
    preview_manager: ImagePreviewManager,
    image_path_regex: Regex,
    url_regex: Regex,
    base64_regex: Regex,
}

#[derive(Debug, Clone)]
pub struct DetectedImage {
    pub path: PathBuf,
    pub source: ImageSource,
    pub context: String,
    pub line_number: usize,
}

#[derive(Debug, Clone)]
pub enum ImageSource {
    FilePath,
    Url,
    Base64Data,
    StdinPipe,
}

impl StdoutMonitor {
    pub async fn new(config: Config) -> Result<Self> {
        let preview_manager = ImagePreviewManager::new(config.clone()).await?;
        
        // Regex patterns for detecting image references
        let image_path_regex = Regex::new(
            r#"(?:^|\s|["'])((?:[~/.]|[A-Za-z]:|\\\\)[^"'\s]*\.(?:png|jpe?g|gif|bmp|webp|svg|tiff?|ico))(?:["']|\s|$)"#
        ).map_err(|e| Error::Config(format!("Failed to compile image path regex: {}", e)))?;
        
        let url_regex = Regex::new(
            r#"https?://[^\s"']+\.(?:png|jpe?g|gif|bmp|webp|svg|tiff?|ico)(?:\?[^\s"']*)?(?:["']|\s|$)"#
        ).map_err(|e| Error::Config(format!("Failed to compile URL regex: {}", e)))?;
        
        let base64_regex = Regex::new(
            r"data:image/(?:png|jpe?g|gif|bmp|webp|svg\+xml);base64,([A-Za-z0-9+/=]+)"
        ).map_err(|e| Error::Config(format!("Failed to compile base64 regex: {}", e)))?;
        
        Ok(Self {
            config,
            preview_manager,
            image_path_regex,
            url_regex,
            base64_regex,
        })
    }
    
    /// Monitor a command's output for image paths
    pub async fn monitor_command(&self, command_args: Vec<String>) -> Result<()> {
        if command_args.is_empty() {
            return Err(Error::InvalidInput("No command provided".to_string()));
        }
        
        info!("Monitoring command output: {:?}", command_args);
        
        let mut cmd = Command::new(&command_args[0]);
        if command_args.len() > 1 {
            cmd.args(&command_args[1..]);
        }
        
        cmd.stdout(Stdio::piped())
           .stderr(Stdio::piped());
        
        let mut child = cmd.spawn()
            .map_err(|e| Error::Process(format!("Failed to spawn command: {}", e)))?;
        
        let (tx, mut rx) = mpsc::channel::<DetectedImage>(100);
        
        // Monitor stdout
        if let Some(stdout) = child.stdout.take() {
            let tx_stdout = tx.clone();
            let monitor = self.clone();
            tokio::spawn(async move {
                if let Err(e) = monitor.monitor_stream(stdout, tx_stdout, "stdout").await {
                    warn!("Error monitoring stdout: {}", e);
                }
            });
        }
        
        // Monitor stderr
        if let Some(stderr) = child.stderr.take() {
            let tx_stderr = tx.clone();
            let monitor = self.clone();
            tokio::spawn(async move {
                if let Err(e) = monitor.monitor_stream(stderr, tx_stderr, "stderr").await {
                    warn!("Error monitoring stderr: {}", e);
                }
            });
        }
        
        // Handle detected images
        tokio::spawn(async move {
            while let Some(detected_image) = rx.recv().await {
                info!("Detected image: {:?}", detected_image);
                // Auto-preview detected images (could be configurable)
                // self.preview_manager.show_preview(&detected_image.path, None, None).await;
            }
        });
        
        // Wait for command to complete
        let status = child.wait()
            .map_err(|e| Error::Process(format!("Failed to wait for command: {}", e)))?;
        
        if !status.success() {
            warn!("Command exited with non-zero status: {}", status);
        }
        
        Ok(())
    }
    
    async fn monitor_stream<R: std::io::Read + Send + 'static>(
        &self,
        stream: R,
        tx: mpsc::Sender<DetectedImage>,
        stream_name: &str,
    ) -> Result<()> {
        let reader = BufReader::new(stream);
        let mut line_number = 0;
        
        for line in reader.lines() {
            line_number += 1;
            let line = line.map_err(Error::Io)?;
            
            // Print the line to maintain normal output
            println!("{}", line);
            
            // Detect images in this line
            let detected = self.detect_images_in_line(&line, line_number);
            
            for image in detected {
                if tx.send(image).await.is_err() {
                    debug!("Receiver dropped, stopping {} monitoring", stream_name);
                    break;
                }
            }
        }
        
        Ok(())
    }
    
    /// Detect image references in a single line
    pub fn detect_images_in_line(&self, line: &str, line_number: usize) -> Vec<DetectedImage> {
        let mut detected = Vec::new();
        
        // Detect file paths
        for cap in self.image_path_regex.captures_iter(line) {
            if let Some(path_match) = cap.get(1) {
                let path_str = path_match.as_str();
                let path = PathBuf::from(self.expand_path(path_str));
                
                if path.exists() && self.is_image_file(&path) {
                    detected.push(DetectedImage {
                        path,
                        source: ImageSource::FilePath,
                        context: line.to_string(),
                        line_number,
                    });
                }
            }
        }
        
        // Detect URLs
        for cap in self.url_regex.captures_iter(line) {
            if let Some(url_match) = cap.get(0) {
                let url = url_match.as_str().trim_end_matches(&['"', '\'', ' ', '\n', '\r']);
                // For URLs, we could download and create a temp file
                // For now, just log the detection
                debug!("Detected image URL: {}", url);
            }
        }
        
        // Detect base64 images
        for cap in self.base64_regex.captures_iter(line) {
            if let Some(base64_match) = cap.get(1) {
                let base64_data = base64_match.as_str();
                // Could decode and create temp file for preview
                debug!("Detected base64 image data: {} bytes", base64_data.len());
            }
        }
        
        detected
    }
    
    fn expand_path(&self, path: &str) -> String {
        if path.starts_with('~') {
            if let Some(home) = dirs::home_dir() {
                return path.replacen('~', &home.to_string_lossy(), 1);
            }
        }
        path.to_string()
    }
    
    fn is_image_file(&self, path: &Path) -> bool {
        if let Some(ext) = path.extension() {
            if let Some(ext_str) = ext.to_str() {
                let ext_lower = ext_str.to_lowercase();
                return matches!(ext_lower.as_str(), 
                    "png" | "jpg" | "jpeg" | "gif" | "bmp" | "webp" | "svg" | "tiff" | "tif" | "ico"
                );
            }
        }
        false
    }
    
    /// Create a wrapper command that monitors the original command's output
    pub fn create_monitoring_wrapper(&self, original_command: &str) -> String {
        format!(
            "({}) 2>&1 | klipdot monitor-output",
            original_command
        )
    }
}

impl Clone for StdoutMonitor {
    fn clone(&self) -> Self {
        // Note: This is a simplified clone that recreates the regexes
        // In practice, you might want to use Arc<Regex> for better performance
        Self {
            config: self.config.clone(),
            preview_manager: self.preview_manager.clone(),
            image_path_regex: self.image_path_regex.clone(),
            url_regex: self.url_regex.clone(),
            base64_regex: self.base64_regex.clone(),
        }
    }
}

/// LSP-style live preview system for real-time image detection
pub struct LivePreviewSystem {
    config: Config,
    preview_manager: ImagePreviewManager,
    current_preview: Option<PathBuf>,
}

impl LivePreviewSystem {
    pub async fn new(config: Config) -> Result<Self> {
        let preview_manager = ImagePreviewManager::new(config.clone()).await?;
        
        Ok(Self {
            config,
            preview_manager,
            current_preview: None,
        })
    }
    
    /// Show live preview as user types (like LSP hover)
    pub async fn show_live_preview(&mut self, text: &str, cursor_position: usize) -> Result<bool> {
        let detected_path = self.extract_image_path_at_cursor(text, cursor_position);
        
        match detected_path {
            Some(path) if Some(&path) != self.current_preview.as_ref() => {
                // New image detected, show preview
                self.show_floating_preview(&path).await?;
                self.current_preview = Some(path);
                Ok(true)
            }
            None if self.current_preview.is_some() => {
                // No image at cursor, hide preview
                self.hide_floating_preview().await?;
                self.current_preview = None;
                Ok(true)
            }
            _ => Ok(false), // No change needed
        }
    }
    
    fn extract_image_path_at_cursor(&self, text: &str, cursor_position: usize) -> Option<PathBuf> {
        // Find word boundaries around cursor
        let before_cursor = &text[..cursor_position.min(text.len())];
        let after_cursor = &text[cursor_position.min(text.len())..];
        
        // Find start of current word
        let word_start = before_cursor.rfind(|c: char| c.is_whitespace() || c == '"' || c == '\'')
            .map(|i| i + 1)
            .unwrap_or(0);
        
        // Find end of current word
        let word_end = after_cursor.find(|c: char| c.is_whitespace() || c == '"' || c == '\'')
            .map(|i| cursor_position + i)
            .unwrap_or(text.len());
        
        if word_start < word_end {
            let word = &text[word_start..word_end];
            let path = PathBuf::from(self.expand_path(word));
            
            if path.exists() && self.is_image_file(&path) {
                return Some(path);
            }
        }
        
        None
    }
    
    async fn show_floating_preview(&self, path: &Path) -> Result<()> {
        // In a real implementation, this would show a floating window or modal
        // For now, we'll show a compact preview with escape sequences for positioning
        
        print!("\x1b[s"); // Save cursor position
        print!("\x1b[H"); // Move to top-left
        print!("\x1b[2K"); // Clear line
        print!("🖼️  Live Preview: {}", path.file_name().unwrap_or_default().to_string_lossy());
        
        // Show small preview
        self.preview_manager.show_preview(path, Some(40), Some(10)).await?;
        
        print!("\x1b[u"); // Restore cursor position
        
        Ok(())
    }
    
    async fn hide_floating_preview(&self) -> Result<()> {
        // Clear the preview area
        print!("\x1b[s"); // Save cursor position
        print!("\x1b[H"); // Move to top-left
        print!("\x1b[K"); // Clear line
        print!("\x1b[u"); // Restore cursor position
        
        Ok(())
    }
    
    fn expand_path(&self, path: &str) -> String {
        if path.starts_with('~') {
            if let Some(home) = dirs::home_dir() {
                return path.replacen('~', &home.to_string_lossy(), 1);
            }
        }
        path.to_string()
    }
    
    fn is_image_file(&self, path: &Path) -> bool {
        if let Some(ext) = path.extension() {
            if let Some(ext_str) = ext.to_str() {
                let ext_lower = ext_str.to_lowercase();
                return matches!(ext_lower.as_str(), 
                    "png" | "jpg" | "jpeg" | "gif" | "bmp" | "webp" | "svg" | "tiff" | "tif" | "ico"
                );
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use std::fs;
    
    #[tokio::test]
    async fn test_detect_images_in_line() {
        let config = Config::default();
        let monitor = StdoutMonitor::new(config).await.unwrap();
        
        // Create a temporary image file for testing
        let temp_dir = tempdir().unwrap();
        let image_path = temp_dir.path().join("test.png");
        fs::write(&image_path, b"fake image data").unwrap();
        
        let line = format!("Found image at: {}", image_path.display());
        let detected = monitor.detect_images_in_line(&line, 1);
        
        assert_eq!(detected.len(), 1);
        assert_eq!(detected[0].path, image_path);
        assert!(matches!(detected[0].source, ImageSource::FilePath));
    }
    
    #[tokio::test]
    async fn test_live_preview_path_extraction() {
        let config = Config::default();
        let mut system = LivePreviewSystem::new(config).await.unwrap();
        
        // Create a temporary image file
        let temp_dir = tempdir().unwrap();
        let image_path = temp_dir.path().join("test.png");
        fs::write(&image_path, b"fake image data").unwrap();
        
        let text = format!("vim {}", image_path.display());
        let cursor_pos = text.len() - 4; // Position in the middle of the filename
        
        let detected = system.extract_image_path_at_cursor(&text, cursor_pos);
        assert_eq!(detected, Some(image_path));
    }
}