use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

use notify_debouncer_mini::{new_debouncer, DebouncedEventKind};
use tokio::sync::broadcast;

/// Directories to watch for changes.
const WATCH_DIRS: &[&str] = &["content", "static", "themes"];
/// Also watch the config file.
const WATCH_FILES: &[&str] = &["pyohwa.toml"];
/// Directories to exclude (checked as path prefixes).
const EXCLUDE_DIRS: &[&str] = &["dist", ".pyohwa", "target"];

/// Start watching for file changes. Runs in a blocking thread.
/// On change, triggers an incremental rebuild and sends a reload signal.
/// The `shutdown` flag is checked periodically to allow graceful termination.
pub fn start_watcher(
    project_root: PathBuf,
    ws_port: u16,
    reload_tx: broadcast::Sender<()>,
    shutdown: Arc<AtomicBool>,
) -> Result<(), crate::error::ServerError> {
    let (tx, rx) = std::sync::mpsc::channel();

    let mut debouncer = new_debouncer(Duration::from_millis(100), tx)
        .map_err(|e| crate::error::ServerError::Watcher(e.to_string()))?;

    // Watch directories
    for dir_name in WATCH_DIRS {
        let dir = project_root.join(dir_name);
        if dir.exists() {
            debouncer
                .watcher()
                .watch(&dir, notify::RecursiveMode::Recursive)
                .map_err(|e| crate::error::ServerError::Watcher(e.to_string()))?;
        }
    }

    // Watch config file
    for file_name in WATCH_FILES {
        let file = project_root.join(file_name);
        if file.exists() {
            debouncer
                .watcher()
                .watch(&file, notify::RecursiveMode::NonRecursive)
                .map_err(|e| crate::error::ServerError::Watcher(e.to_string()))?;
        }
    }

    eprintln!("Watching for changes...");

    loop {
        if shutdown.load(Ordering::Relaxed) {
            break;
        }
        match rx.recv_timeout(Duration::from_millis(200)) {
            Ok(Ok(events)) => {
                let has_relevant_change = events.iter().any(|event| {
                    event.kind == DebouncedEventKind::Any
                        && !is_excluded(&event.path, &project_root)
                });

                if !has_relevant_change {
                    continue;
                }

                let start = Instant::now();

                match pyohwa_core::build::pipeline::build_dev_incremental(&project_root, ws_port) {
                    Ok(true) => {
                        let elapsed = start.elapsed();
                        eprintln!("Rebuilt in {}ms", elapsed.as_millis());
                        let _ = reload_tx.send(());
                    }
                    Ok(false) => {
                        // No actual content changes detected by manifest
                    }
                    Err(e) => {
                        eprintln!("Build error: {e}");
                    }
                }
            }
            Ok(Err(e)) => {
                eprintln!("Watcher error: {e:?}");
            }
            Err(std::sync::mpsc::RecvTimeoutError::Timeout) => continue,
            Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => break,
        }
    }

    Ok(())
}

fn is_excluded(path: &Path, project_root: &Path) -> bool {
    let relative = path.strip_prefix(project_root).unwrap_or(path);

    for exclude in EXCLUDE_DIRS {
        if relative.starts_with(exclude) {
            return true;
        }
    }

    false
}
