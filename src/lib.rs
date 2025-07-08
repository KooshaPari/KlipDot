pub mod clipboard;
pub mod config;
pub mod error;
pub mod interceptor;
pub mod service;
pub mod installer;
pub mod image_processor;
pub mod shell_hooks;

pub use error::{Error, Result};

/// KlipDot version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Default configuration directory name
pub const APP_NAME: &str = "klipdot";

/// Default screenshot directory name
pub const SCREENSHOT_DIR: &str = "screenshots";

/// Configuration file name
pub const CONFIG_FILE: &str = "config.json";

/// Service PID file name
pub const PID_FILE: &str = "klipdot.pid";

/// Service log file name
pub const LOG_FILE: &str = "klipdot.log";

/// Shell hooks directory name
pub const HOOKS_DIR: &str = "hooks";

/// Temporary files directory name
pub const TEMP_DIR: &str = "temp";

/// Default polling interval in milliseconds
pub const DEFAULT_POLL_INTERVAL: u64 = 1000;

/// Default cleanup age in days
pub const DEFAULT_CLEANUP_DAYS: u32 = 30;

/// Maximum file size for image processing (10MB)
pub const MAX_FILE_SIZE: u64 = 10 * 1024 * 1024;

/// Supported image formats
pub const SUPPORTED_FORMATS: &[&str] = &["png", "jpg", "jpeg", "gif", "bmp", "webp", "svg"];

/// Image quality for compression
pub const IMAGE_QUALITY: u8 = 90;

/// Maximum number of recent screenshots to display
pub const MAX_RECENT_SCREENSHOTS: usize = 10;

/// Service status check interval in milliseconds
pub const SERVICE_CHECK_INTERVAL: u64 = 5000;

/// Shell hook patterns to detect image operations
pub const IMAGE_COMMAND_PATTERNS: &[&str] = &[
    r"cp.*\.(png|jpg|jpeg|gif|bmp|webp|svg)",
    r"mv.*\.(png|jpg|jpeg|gif|bmp|webp|svg)",
    r"scp.*\.(png|jpg|jpeg|gif|bmp|webp|svg)",
    r"rsync.*\.(png|jpg|jpeg|gif|bmp|webp|svg)",
    r"screencapture",
    r"screenshot",
    r"scrot",
    r"gnome-screenshot",
    r"import.*\.(png|jpg|jpeg|gif|bmp|webp|svg)",
    r"convert.*\.(png|jpg|jpeg|gif|bmp|webp|svg)",
];

/// Process names to monitor for image operations
pub const IMAGE_PROCESS_NAMES: &[&str] = &[
    "screencapture",
    "screenshot",
    "scrot",
    "gnome-screenshot",
    "import",
    "convert",
    "ffmpeg",
    "imagemagick",
    "gimp",
    "inkscape",
];

/// Initialize tracing for the application
pub fn init_tracing(verbose: bool) {
    use tracing_subscriber::EnvFilter;
    
    let filter = if verbose {
        EnvFilter::new("klipdot=debug")
    } else {
        EnvFilter::new("klipdot=info")
    };
    
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(false)
        .with_thread_ids(true)
        .init();
}

/// Get the application data directory
pub fn get_app_dir() -> Result<std::path::PathBuf> {
    let app_dir = dirs::data_dir()
        .ok_or_else(|| Error::Config("Failed to get data directory".to_string()))?
        .join(APP_NAME);
    
    std::fs::create_dir_all(&app_dir)?;
    Ok(app_dir)
}

/// Get the application config directory
pub fn get_config_dir() -> Result<std::path::PathBuf> {
    let config_dir = dirs::config_dir()
        .ok_or_else(|| Error::Config("Failed to get config directory".to_string()))?
        .join(APP_NAME);
    
    std::fs::create_dir_all(&config_dir)?;
    Ok(config_dir)
}

/// Get the application home directory
pub fn get_home_dir() -> Result<std::path::PathBuf> {
    let home_dir = dirs::home_dir()
        .ok_or_else(|| Error::Config("Failed to get home directory".to_string()))?
        .join(format!(".{}", APP_NAME));
    
    std::fs::create_dir_all(&home_dir)?;
    Ok(home_dir)
}

/// Check if a file is an image based on extension
pub fn is_image_file(path: &std::path::Path) -> bool {
    if let Some(ext) = path.extension() {
        if let Some(ext_str) = ext.to_str() {
            return SUPPORTED_FORMATS.contains(&ext_str.to_lowercase().as_str());
        }
    }
    false
}

/// Generate a unique filename for a screenshot
pub fn generate_screenshot_filename(source: &str) -> String {
    let timestamp = chrono::Utc::now().format("%Y-%m-%dT%H-%M-%S%.3fZ");
    let id = uuid::Uuid::new_v4().to_string()[..8].to_string();
    format!("{}-{}-{}.png", source, timestamp, id)
}

/// Format file size for display
pub fn format_file_size(size: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    let mut size = size as f64;
    let mut unit_index = 0;
    
    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }
    
    format!("{:.1} {}", size, UNITS[unit_index])
}

/// Format duration for display
pub fn format_duration(duration: std::time::Duration) -> String {
    let total_seconds = duration.as_secs();
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;
    
    if hours > 0 {
        format!("{}h {}m {}s", hours, minutes, seconds)
    } else if minutes > 0 {
        format!("{}m {}s", minutes, seconds)
    } else {
        format!("{}s", seconds)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_is_image_file() {
        assert!(is_image_file(&std::path::Path::new("test.png")));
        assert!(is_image_file(&std::path::Path::new("test.jpg")));
        assert!(is_image_file(&std::path::Path::new("test.PNG")));
        assert!(!is_image_file(&std::path::Path::new("test.txt")));
        assert!(!is_image_file(&std::path::Path::new("test")));
    }
    
    #[test]
    fn test_generate_screenshot_filename() {
        let filename = generate_screenshot_filename("clipboard");
        assert!(filename.starts_with("clipboard-"));
        assert!(filename.ends_with(".png"));
        assert!(filename.len() > 20);
    }
    
    #[test]
    fn test_format_file_size() {
        assert_eq!(format_file_size(512), "512.0 B");
        assert_eq!(format_file_size(1024), "1.0 KB");
        assert_eq!(format_file_size(1536), "1.5 KB");
        assert_eq!(format_file_size(1024 * 1024), "1.0 MB");
    }
    
    #[test]
    fn test_format_duration() {
        assert_eq!(format_duration(std::time::Duration::from_secs(30)), "30s");
        assert_eq!(format_duration(std::time::Duration::from_secs(90)), "1m 30s");
        assert_eq!(format_duration(std::time::Duration::from_secs(3665)), "1h 1m 5s");
    }
}