use crate::{config::Config, error::Result, Error};
use std::collections::HashMap;
use std::process::Stdio;
use std::time::Duration;
use tokio::process::Command;
use tokio::time::sleep;
use tracing::{debug, info, warn};

pub struct TerminalInterceptor {
    config: Config,
    running: bool,
    process_monitors: HashMap<String, ProcessMonitor>,
}

#[derive(Debug, Clone)]
struct ProcessMonitor {
    name: String,
    pid: Option<u32>,
    last_seen: std::time::SystemTime,
}

impl TerminalInterceptor {
    pub async fn new(config: Config) -> Result<Self> {
        Ok(Self {
            config,
            running: false,
            process_monitors: HashMap::new(),
        })
    }
    
    pub async fn run(&mut self) -> Result<()> {
        if !self.config.intercept_methods.process_monitor {
            info!("Process monitoring disabled in config");
            return Ok(());
        }
        
        info!("Starting terminal interceptor");
        self.running = true;
        
        let mut interval = tokio::time::interval(Duration::from_millis(self.config.poll_interval));
        
        while self.running {
            interval.tick().await;
            
            if let Err(e) = self.monitor_processes().await {
                if e.is_recoverable() {
                    warn!("Recoverable process monitoring error: {}", e);
                } else {
                    return Err(e);
                }
            }
        }
        
        Ok(())
    }
    
    pub fn stop(&mut self) {
        info!("Stopping terminal interceptor");
        self.running = false;
    }
    
    async fn monitor_processes(&mut self) -> Result<()> {
        debug!("Monitoring processes for image operations");
        
        let processes = self.get_running_processes().await?;
        
        for process in processes {
            if self.is_image_process(&process.name) {
                self.handle_image_process(&process).await?;
            }
        }
        
        Ok(())
    }
    
    async fn get_running_processes(&self) -> Result<Vec<Process>> {
        let mut processes = Vec::new();
        
        #[cfg(unix)]
        {
            let output = Command::new("ps")
                .arg("-eo")
                .arg("pid,comm,args")
                .output()
                .await
                .map_err(|e| Error::Process(format!("Failed to run ps: {}", e)))?;
            
            if output.status.success() {
                let output_str = String::from_utf8_lossy(&output.stdout);
                for line in output_str.lines().skip(1) {
                    if let Some(process) = self.parse_ps_line(line) {
                        processes.push(process);
                    }
                }
            }
        }
        
        #[cfg(windows)]
        {
            let output = Command::new("wmic")
                .arg("process")
                .arg("get")
                .arg("ProcessId,Name,CommandLine")
                .arg("/format:csv")
                .output()
                .await
                .map_err(|e| Error::Process(format!("Failed to run wmic: {}", e)))?;
            
            if output.status.success() {
                let output_str = String::from_utf8_lossy(&output.stdout);
                for line in output_str.lines().skip(1) {
                    if let Some(process) = self.parse_wmic_line(line) {
                        processes.push(process);
                    }
                }
            }
        }
        
        Ok(processes)
    }
    
    #[cfg(unix)]
    fn parse_ps_line(&self, line: &str) -> Option<Process> {
        let parts: Vec<&str> = line.trim().split_whitespace().collect();
        if parts.len() >= 3 {
            if let Ok(pid) = parts[0].parse::<u32>() {
                let name = parts[1].to_string();
                let command = parts[2..].join(" ");
                
                return Some(Process {
                    pid,
                    name,
                    command,
                });
            }
        }
        None
    }
    
    #[cfg(windows)]
    fn parse_wmic_line(&self, line: &str) -> Option<Process> {
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() >= 3 {
            if let Ok(pid) = parts[2].parse::<u32>() {
                let name = parts[1].to_string();
                let command = parts[0].to_string();
                
                return Some(Process {
                    pid,
                    name,
                    command,
                });
            }
        }
        None
    }
    
    fn is_image_process(&self, name: &str) -> bool {
        let name_lower = name.to_lowercase();
        
        for process_name in crate::IMAGE_PROCESS_NAMES {
            if name_lower.contains(&process_name.to_lowercase()) {
                return true;
            }
        }
        
        false
    }
    
    async fn handle_image_process(&mut self, process: &Process) -> Result<()> {
        debug!("Detected image process: {} (PID: {})", process.name, process.pid);
        
        // Check if this is a screenshot process
        if self.is_screenshot_process(&process.name) {
            self.handle_screenshot_process(process).await?;
        }
        
        // Update process monitor
        self.process_monitors.insert(
            process.name.clone(),
            ProcessMonitor {
                name: process.name.clone(),
                pid: Some(process.pid),
                last_seen: std::time::SystemTime::now(),
            },
        );
        
        Ok(())
    }
    
    fn is_screenshot_process(&self, name: &str) -> bool {
        let screenshot_processes = [
            "screencapture",
            "screenshot",
            "scrot",
            "gnome-screenshot",
            "spectacle",
            "flameshot",
        ];
        
        let name_lower = name.to_lowercase();
        for proc in &screenshot_processes {
            if name_lower.contains(proc) {
                return true;
            }
        }
        
        false
    }
    
    async fn handle_screenshot_process(&self, process: &Process) -> Result<()> {
        info!("Screenshot process detected: {} (PID: {})", process.name, process.pid);
        
        // Wait for the process to complete
        self.wait_for_process_completion(process.pid).await?;
        
        // Look for recently created image files
        self.scan_for_new_images().await?;
        
        Ok(())
    }
    
    async fn wait_for_process_completion(&self, pid: u32) -> Result<()> {
        let mut attempts = 0;
        const MAX_ATTEMPTS: u32 = 50; // 5 seconds with 100ms intervals
        
        while attempts < MAX_ATTEMPTS {
            if !self.is_process_running(pid).await? {
                break;
            }
            
            sleep(Duration::from_millis(100)).await;
            attempts += 1;
        }
        
        Ok(())
    }
    
    async fn is_process_running(&self, pid: u32) -> Result<bool> {
        #[cfg(unix)]
        {
            match Command::new("kill")
                .arg("-0")
                .arg(&pid.to_string())
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .status()
                .await
            {
                Ok(status) => Ok(status.success()),
                Err(_) => Ok(false),
            }
        }
        
        #[cfg(windows)]
        {
            match Command::new("tasklist")
                .arg("/FI")
                .arg(&format!("PID eq {}", pid))
                .stdout(Stdio::piped())
                .stderr(Stdio::null())
                .output()
                .await
            {
                Ok(output) => {
                    let output_str = String::from_utf8_lossy(&output.stdout);
                    Ok(output_str.contains(&pid.to_string()))
                },
                Err(_) => Ok(false),
            }
        }
    }
    
    async fn scan_for_new_images(&self) -> Result<()> {
        let scan_dirs = [
            dirs::desktop_dir(),
            dirs::download_dir(),
            dirs::picture_dir(),
            Some(std::env::current_dir().unwrap_or_else(|_| "/tmp".into())),
        ];
        
        for dir_option in &scan_dirs {
            if let Some(dir) = dir_option {
                if dir.exists() {
                    self.scan_directory_for_images(dir).await?;
                }
            }
        }
        
        Ok(())
    }
    
    async fn scan_directory_for_images(&self, dir: &std::path::Path) -> Result<()> {
        let mut entries = tokio::fs::read_dir(dir).await?;
        let now = std::time::SystemTime::now();
        
        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            
            if path.is_file() && crate::is_image_file(&path) {
                if let Ok(metadata) = entry.metadata().await {
                    if let Ok(created) = metadata.created() {
                        // Check if file was created in the last 30 seconds
                        if let Ok(elapsed) = now.duration_since(created) {
                            if elapsed.as_secs() < 30 {
                                self.process_new_image(&path).await?;
                            }
                        }
                    }
                }
            }
        }
        
        Ok(())
    }
    
    async fn process_new_image(&self, path: &std::path::Path) -> Result<()> {
        info!("Processing new image: {:?}", path);
        
        // Use the image processor to handle the file
        let image_processor = crate::image_processor::ImageProcessor::new(self.config.clone()).await?;
        let processed_path = image_processor.process_image_file(&path.to_path_buf(), "screenshot").await?;
        
        // Replace the original file reference with the processed path
        // This would typically involve shell integration
        debug!("Processed screenshot: {:?} -> {:?}", path, processed_path);
        
        Ok(())
    }
    
    pub async fn cleanup_old_monitors(&mut self) -> Result<()> {
        let now = std::time::SystemTime::now();
        let mut to_remove = Vec::new();
        
        for (name, monitor) in &self.process_monitors {
            if let Ok(elapsed) = now.duration_since(monitor.last_seen) {
                if elapsed.as_secs() > 300 { // 5 minutes
                    to_remove.push(name.clone());
                }
            }
        }
        
        for name in to_remove {
            self.process_monitors.remove(&name);
        }
        
        Ok(())
    }
}

#[derive(Debug, Clone)]
struct Process {
    pid: u32,
    name: String,
    command: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    
    #[tokio::test]
    async fn test_terminal_interceptor_creation() {
        let temp_dir = TempDir::new().unwrap();
        let mut config = Config::default();
        config.screenshot_dir = temp_dir.path().to_path_buf();
        
        let interceptor = TerminalInterceptor::new(config).await;
        assert!(interceptor.is_ok());
    }
    
    #[test]
    fn test_image_process_detection() {
        let config = Config::default();
        let interceptor = TerminalInterceptor {
            config,
            running: false,
            process_monitors: HashMap::new(),
        };
        
        assert!(interceptor.is_image_process("screencapture"));
        assert!(interceptor.is_image_process("screenshot"));
        assert!(interceptor.is_image_process("scrot"));
        assert!(interceptor.is_image_process("convert"));
        assert!(!interceptor.is_image_process("bash"));
        assert!(!interceptor.is_image_process("vim"));
    }
    
    #[test]
    fn test_screenshot_process_detection() {
        let config = Config::default();
        let interceptor = TerminalInterceptor {
            config,
            running: false,
            process_monitors: HashMap::new(),
        };
        
        assert!(interceptor.is_screenshot_process("screencapture"));
        assert!(interceptor.is_screenshot_process("gnome-screenshot"));
        assert!(interceptor.is_screenshot_process("flameshot"));
        assert!(!interceptor.is_screenshot_process("convert"));
        assert!(!interceptor.is_screenshot_process("gimp"));
    }
    
    #[tokio::test]
    async fn test_cleanup_old_monitors() {
        let temp_dir = TempDir::new().unwrap();
        let mut config = Config::default();
        config.screenshot_dir = temp_dir.path().to_path_buf();
        
        let mut interceptor = TerminalInterceptor::new(config).await.unwrap();
        
        // Add an old monitor
        let old_time = std::time::SystemTime::now() - Duration::from_secs(400);
        interceptor.process_monitors.insert(
            "old_process".to_string(),
            ProcessMonitor {
                name: "old_process".to_string(),
                pid: Some(12345),
                last_seen: old_time,
            },
        );
        
        // Add a recent monitor
        interceptor.process_monitors.insert(
            "recent_process".to_string(),
            ProcessMonitor {
                name: "recent_process".to_string(),
                pid: Some(67890),
                last_seen: std::time::SystemTime::now(),
            },
        );
        
        assert_eq!(interceptor.process_monitors.len(), 2);
        
        interceptor.cleanup_old_monitors().await.unwrap();
        
        assert_eq!(interceptor.process_monitors.len(), 1);
        assert!(interceptor.process_monitors.contains_key("recent_process"));
        assert!(!interceptor.process_monitors.contains_key("old_process"));
    }
}