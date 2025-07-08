use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    
    #[error("Image processing error: {0}")]
    Image(#[from] image::ImageError),
    
    #[error("File watcher error: {0}")]
    FileWatcher(#[from] notify::Error),
    
    #[error("Configuration error: {0}")]
    Config(String),
    
    #[error("Clipboard error: {0}")]
    Clipboard(String),
    
    #[error("Service error: {0}")]
    Service(String),
    
    #[error("Shell integration error: {0}")]
    Shell(String),
    
    #[error("Process error: {0}")]
    Process(String),
    
    #[error("Network error: {0}")]
    Network(String),
    
    #[error("Permission error: {0}")]
    Permission(String),
    
    #[error("Timeout error: {0}")]
    Timeout(String),
    
    #[error("Validation error: {0}")]
    Validation(String),
    
    #[error("Not found: {0}")]
    NotFound(String),
    
    #[error("Already exists: {0}")]
    AlreadyExists(String),
    
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    
    #[error("Unsupported operation: {0}")]
    Unsupported(String),
    
    #[error("Internal error: {0}")]
    Internal(String),
    
    #[error("Parse error: {0}")]
    Parse(String),
    
    #[error("Format error: {0}")]
    Format(String),
    
    #[error("Cancelled")]
    Cancelled,
    
    #[error("Unknown error: {0}")]
    Unknown(String),
}

pub type Result<T> = std::result::Result<T, Error>;

impl Error {
    pub fn is_recoverable(&self) -> bool {
        match self {
            Error::Io(_) => true,
            Error::Clipboard(_) => true,
            Error::Network(_) => true,
            Error::Timeout(_) => true,
            Error::Process(_) => true,
            Error::Cancelled => true,
            _ => false,
        }
    }
    
    pub fn is_fatal(&self) -> bool {
        match self {
            Error::Config(_) => true,
            Error::Permission(_) => true,
            Error::Unsupported(_) => true,
            Error::Internal(_) => true,
            _ => false,
        }
    }
    
    pub fn error_code(&self) -> &'static str {
        match self {
            Error::Io(_) => "IO",
            Error::Serialization(_) => "SERIALIZATION",
            Error::Image(_) => "IMAGE",
            Error::FileWatcher(_) => "FILE_WATCHER",
            Error::Config(_) => "CONFIG",
            Error::Clipboard(_) => "CLIPBOARD",
            Error::Service(_) => "SERVICE",
            Error::Shell(_) => "SHELL",
            Error::Process(_) => "PROCESS",
            Error::Network(_) => "NETWORK",
            Error::Permission(_) => "PERMISSION",
            Error::Timeout(_) => "TIMEOUT",
            Error::Validation(_) => "VALIDATION",
            Error::NotFound(_) => "NOT_FOUND",
            Error::AlreadyExists(_) => "ALREADY_EXISTS",
            Error::InvalidInput(_) => "INVALID_INPUT",
            Error::Unsupported(_) => "UNSUPPORTED",
            Error::Internal(_) => "INTERNAL",
            Error::Parse(_) => "PARSE",
            Error::Format(_) => "FORMAT",
            Error::Cancelled => "CANCELLED",
            Error::Unknown(_) => "UNKNOWN",
        }
    }
}

impl From<Box<dyn std::error::Error + Send + Sync>> for Error {
    fn from(err: Box<dyn std::error::Error + Send + Sync>) -> Self {
        Error::Unknown(err.to_string())
    }
}

impl From<&str> for Error {
    fn from(err: &str) -> Self {
        Error::Unknown(err.to_string())
    }
}

impl From<String> for Error {
    fn from(err: String) -> Self {
        Error::Unknown(err)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_error_properties() {
        let io_error = Error::Io(std::io::Error::new(std::io::ErrorKind::NotFound, "file not found"));
        assert!(io_error.is_recoverable());
        assert!(!io_error.is_fatal());
        assert_eq!(io_error.error_code(), "IO");
        
        let config_error = Error::Config("invalid config".to_string());
        assert!(!config_error.is_recoverable());
        assert!(config_error.is_fatal());
        assert_eq!(config_error.error_code(), "CONFIG");
    }
    
    #[test]
    fn test_error_from_string() {
        let error = Error::from("test error");
        assert_eq!(error.error_code(), "UNKNOWN");
        assert!(error.to_string().contains("test error"));
    }
}