mod error;
mod reload;
mod server;
mod watcher;

pub use error::ServerError;

use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use tokio::sync::broadcast;

pub struct DevServerConfig {
    pub port: u16,
    pub project_root: PathBuf,
    pub open: bool,
}

impl Default for DevServerConfig {
    fn default() -> Self {
        Self {
            port: 3000,
            project_root: PathBuf::from("."),
            open: false,
        }
    }
}

/// Run the dev server with file watching and live reload.
pub async fn run_dev_server(config: DevServerConfig) -> Result<(), ServerError> {
    let project_root = if config.project_root == PathBuf::from(".") {
        std::env::current_dir()?
    } else {
        std::fs::canonicalize(&config.project_root)?
    };

    // Initial build with live reload JS
    eprintln!("Building site...");
    pyohwa_core::build::pipeline::build_dev(&project_root, config.port)?;
    eprintln!("Build complete.");

    // Broadcast channel for reload signals
    let (reload_tx, _) = broadcast::channel::<()>(16);

    // Shutdown flag for the file watcher
    let shutdown = Arc::new(AtomicBool::new(false));
    let shutdown_watcher = shutdown.clone();

    // Start file watcher in a blocking thread
    let watcher_root = project_root.clone();
    let watcher_tx = reload_tx.clone();
    let ws_port = config.port;
    let watcher_handle = tokio::task::spawn_blocking(move || {
        if let Err(e) = watcher::start_watcher(watcher_root, ws_port, watcher_tx, shutdown_watcher)
        {
            eprintln!("Watcher error: {e}");
        }
    });

    eprintln!("Dev server running at http://localhost:{}", config.port);

    if config.open {
        let url = format!("http://localhost:{}", config.port);
        let _ = open_browser(&url);
    }

    // Start HTTP + WebSocket server (blocks until Ctrl+C)
    server::start_server(&project_root, config.port, reload_tx).await?;

    // Signal the watcher to shut down and wait briefly
    shutdown.store(true, Ordering::Relaxed);
    let _ = tokio::time::timeout(std::time::Duration::from_millis(500), watcher_handle).await;

    Ok(())
}

fn open_browser(url: &str) -> Result<(), std::io::Error> {
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open").arg(url).spawn()?;
    }
    #[cfg(target_os = "linux")]
    {
        std::process::Command::new("xdg-open").arg(url).spawn()?;
    }
    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("cmd")
            .args(["/C", "start", url])
            .spawn()?;
    }
    Ok(())
}
