// src/watch.rs

use crate::build::{build_fresh, get_paths_to_watch};
use crate::config::Config;
use crate::error::RunError;
use tracing::{debug, error, info};

/// Watch for file changes and rebuild automatically (macOS only)
#[cfg(target_os = "macos")]
pub(crate) fn watch(config_file: &str, include_drafts: bool) -> Result<(), RunError> {
    use std::sync::mpsc::channel;
    use std::thread;
    use std::time::{Duration, Instant};

    // Load config to get directories to watch
    let config = Config::load_from_file(config_file)?;

    let paths_to_watch = get_paths_to_watch(config_file, &config);

    info!("watch::start {:?}", paths_to_watch);
    info!("watch::info press Ctrl+C to stop");
    if include_drafts {
        info!("watch::drafts including draft content");
    }

    // Initial build (use fresh environment from the start)
    if let Err(e) = build_fresh(config_file, include_drafts) {
        error!("Initial build failed: {:?}", e);
    }

    let (sender, receiver) = channel();

    let _watcher_thread = thread::spawn(move || {
        let fsevent = fsevent::FsEvent::new(paths_to_watch);
        fsevent.observe(sender);
    });

    // Debounce: track last build time
    let mut last_build = Instant::now();
    let debounce_duration = Duration::from_millis(500);

    loop {
        match receiver.recv() {
            Ok(events) => {
                // Check debounce
                if last_build.elapsed() < debounce_duration {
                    debug!("watch::debounce skipping rebuild");
                    continue;
                }

                // Log event_id at INFO, full details at DEBUG
                info!("watch::change event_id: {}", events.event_id);
                debug!("watch::change {:?}", events);
                last_build = Instant::now();

                if let Err(e) = build_fresh(config_file, include_drafts) {
                    error!("Build failed: {:?}", e);
                }
            }
            Err(e) => {
                error!("Watch error: {:?}", e);
                break;
            }
        }
    }

    Ok(())
}

#[cfg(not(target_os = "macos"))]
pub(crate) fn watch(_config_file: &str, _include_drafts: bool) -> Result<(), RunError> {
    eprintln!("Watch mode is only supported on macOS");
    std::process::exit(1);
}
