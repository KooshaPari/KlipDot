use anyhow::Result;
use clap::{Parser, Subcommand};
use klipdot::{
    clipboard::ClipboardMonitor,
    config::Config,
    interceptor::TerminalInterceptor,
    service::ServiceManager,
    image_preview::ImagePreviewManager,
};
use std::path::PathBuf;
use tracing::{info, error};
use tracing_subscriber::EnvFilter;

#[derive(Parser)]
#[command(
    name = "klipdot",
    about = "Universal terminal image interceptor",
    version,
    long_about = "KlipDot automatically intercepts image pastes and file operations, replacing them with file paths for any CLI/TUI application."
)]
struct Args {
    #[command(subcommand)]
    command: Commands,
    
    #[arg(short, long, global = true)]
    verbose: bool,
    
    #[arg(short, long, global = true)]
    config: Option<PathBuf>,
}

#[derive(Subcommand)]
enum Commands {
    /// Start the image interceptor service
    Start {
        #[arg(short, long)]
        daemon: bool,
    },
    /// Stop the running service
    Stop,
    /// Restart the service
    Restart,
    /// Show service status and statistics
    Status,
    /// Install shell hooks and system integration
    Install {
        #[arg(short, long)]
        shell: Option<String>,
    },
    /// Uninstall shell hooks and system integration
    Uninstall,
    /// Clean up old screenshots
    Cleanup {
        #[arg(short, long, default_value = "30")]
        days: u32,
    },
    /// Show configuration
    Config {
        #[command(subcommand)]
        action: Option<ConfigAction>,
    },
    /// Preview an image in the terminal
    Preview {
        /// Path to the image file
        image_path: PathBuf,
        /// Maximum width in characters/pixels
        #[arg(short, long)]
        width: Option<u32>,
        /// Maximum height in characters/pixels
        #[arg(short = 'H', long)]
        height: Option<u32>,
    },
}

#[derive(Subcommand)]
enum ConfigAction {
    /// Show current configuration
    Show,
    /// Edit configuration file
    Edit,
    /// Reset to default configuration
    Reset,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    
    // Initialize tracing
    let filter = if args.verbose {
        EnvFilter::new("klipdot=debug")
    } else {
        EnvFilter::new("klipdot=info")
    };
    
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .init();
    
    // Load configuration
    let config = if let Some(config_path) = args.config {
        Config::load_from_path(&config_path)?
    } else {
        Config::load_or_create_default()?
    };
    
    info!("KlipDot starting with config: {:?}", config);
    
    match args.command {
        Commands::Start { daemon } => {
            if daemon {
                start_daemon(&config).await?;
            } else {
                start_foreground(&config).await?;
            }
        }
        Commands::Stop => {
            ServiceManager::stop().await?;
        }
        Commands::Restart => {
            ServiceManager::restart().await?;
        }
        Commands::Status => {
            show_status(&config).await?;
        }
        Commands::Install { shell } => {
            install_hooks(shell).await?;
        }
        Commands::Uninstall => {
            uninstall_hooks().await?;
        }
        Commands::Cleanup { days } => {
            cleanup_screenshots(&config, days).await?;
        }
        Commands::Config { action } => {
            handle_config_command(action, &config).await?;
        }
        Commands::Preview { image_path, width, height } => {
            handle_preview_command(&config, &image_path, width, height).await?;
        }
    }
    
    Ok(())
}

async fn start_foreground(config: &Config) -> Result<()> {
    info!("Starting KlipDot in foreground mode");
    
    let mut interceptor = TerminalInterceptor::new(config.clone()).await?;
    let mut clipboard_monitor = ClipboardMonitor::new(config.clone()).await?;
    
    // Handle shutdown gracefully
    let shutdown_signal = async {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to install CTRL+C signal handler");
    };
    
    tokio::select! {
        result = interceptor.run() => {
            if let Err(e) = result {
                error!("Terminal interceptor error: {}", e);
            }
        }
        result = clipboard_monitor.run() => {
            if let Err(e) = result {
                error!("Clipboard monitor error: {}", e);
            }
        }
        _ = shutdown_signal => {
            info!("Received shutdown signal, stopping KlipDot");
        }
    }
    
    Ok(())
}

async fn start_daemon(config: &Config) -> Result<()> {
    info!("Starting KlipDot in daemon mode");
    ServiceManager::start_daemon(config.clone()).await
        .map_err(|e| anyhow::anyhow!("Failed to start daemon: {}", e))
}

async fn show_status(config: &Config) -> Result<()> {
    let service_manager = ServiceManager::new();
    let status = service_manager.status().await?;
    
    println!("=== KlipDot Status ===");
    println!("Service: {}", if status.running { "Running" } else { "Stopped" });
    
    if let Some(pid) = status.pid {
        println!("PID: {}", pid);
    }
    
    if let Some(uptime) = status.uptime {
        println!("Uptime: {}", klipdot::format_duration(uptime));
    }
    
    println!("Configuration: {:?}", config.screenshot_dir);
    
    // Show recent screenshots
    let screenshots = config.get_recent_screenshots(5).await?;
    println!("Recent screenshots: {}", screenshots.len());
    
    for (i, screenshot) in screenshots.iter().enumerate() {
        println!("  {}. {} ({})", i + 1, screenshot.filename, screenshot.size);
    }
    
    Ok(())
}

async fn install_hooks(shell: Option<String>) -> Result<()> {
    info!("Installing KlipDot shell hooks");
    
    let shell = shell.unwrap_or_else(|| {
        std::env::var("SHELL")
            .unwrap_or_else(|_| "/bin/bash".to_string())
            .split('/')
            .last()
            .unwrap_or("bash")
            .to_string()
    });
    
    let installer = klipdot::installer::ShellInstaller::new(&shell);
    installer.install().await?;
    
    println!("✅ Shell hooks installed for {}", shell);
    println!("Please restart your shell or run: source ~/.{}rc", shell);
    
    Ok(())
}

async fn uninstall_hooks() -> Result<()> {
    info!("Uninstalling KlipDot shell hooks");
    
    let installer = klipdot::installer::ShellInstaller::detect_shell();
    installer.uninstall().await?;
    
    println!("✅ Shell hooks uninstalled");
    println!("Please restart your shell to complete removal");
    
    Ok(())
}

async fn cleanup_screenshots(config: &Config, days: u32) -> Result<()> {
    info!("Cleaning up screenshots older than {} days", days);
    
    let count = config.cleanup_old_screenshots(days).await?;
    println!("✅ Cleaned up {} old screenshots", count);
    
    Ok(())
}

async fn handle_config_command(action: Option<ConfigAction>, config: &Config) -> Result<()> {
    match action.unwrap_or(ConfigAction::Show) {
        ConfigAction::Show => {
            println!("=== KlipDot Configuration ===");
            println!("{}", serde_json::to_string_pretty(config)?);
        }
        ConfigAction::Edit => {
            let editor = std::env::var("EDITOR").unwrap_or_else(|_| "nano".to_string());
            let config_path = config.get_config_path();
            
            std::process::Command::new(editor)
                .arg(&config_path)
                .status()?;
                
            println!("Configuration edited: {:?}", config_path);
        }
        ConfigAction::Reset => {
            Config::reset_to_default()?;
            println!("✅ Configuration reset to default");
        }
    }
    
    Ok(())
}

async fn handle_preview_command(config: &Config, image_path: &PathBuf, width: Option<u32>, height: Option<u32>) -> Result<()> {
    info!("Showing preview for image: {:?}", image_path);
    
    let preview_manager = ImagePreviewManager::new(config.clone()).await
        .map_err(|e| anyhow::anyhow!("Failed to create preview manager: {}", e))?;
    
    preview_manager.show_preview(image_path, width, height).await
        .map_err(|e| anyhow::anyhow!("Failed to show preview: {}", e))?;
    
    Ok(())
}