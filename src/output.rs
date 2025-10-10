// src/output.rs

use std::{
    fs,
    io::ErrorKind,
    path::{Path, PathBuf},
};
use thiserror::Error;
use tracing::{debug, info};
use walkdir::WalkDir;

use crate::{config::Config, error::StaticError};

#[derive(Error, Debug)]
pub(crate) enum WriteError {
    #[error("I/O error processing static file {path:?}: {source}")]
    Io {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
}

pub(crate) fn copy_static_files(config: &Config) -> Result<(), StaticError> {
    let static_dir = &config.static_dir;
    info!("Processing static directory: {}", static_dir);

    // Check if static directory exists
    if !PathBuf::from(static_dir).exists() {
        info!("No static directory found at: {}", static_dir);
        return Ok(());
    }

    // Create the static directory in the output folder
    let output_static_dir = PathBuf::from(&config.output_dir).join("static");
    fs::create_dir_all(&output_static_dir).map_err(|e| StaticError::Io {
        path: output_static_dir.clone(),
        source: e,
    })?;

    // Copy all files recursively
    for entry in WalkDir::new(static_dir)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
    {
        let source_path = entry.path();
        let relative_path = source_path
            .strip_prefix(static_dir)
            .map_err(|e| StaticError::Io {
                path: source_path.to_path_buf(),
                source: std::io::Error::new(ErrorKind::Other, e),
            })?;

        let dest_path = output_static_dir.join(relative_path);

        // Create parent directories if needed
        if let Some(parent) = dest_path.parent() {
            fs::create_dir_all(parent).map_err(|e| StaticError::Io {
                path: parent.to_path_buf(),
                source: e,
            })?;
        }

        fs::copy(source_path, &dest_path).map_err(|e| StaticError::Io {
            path: dest_path.clone(),
            source: e,
        })?;

        debug!("Copied static file: {:?} -> {:?}", source_path, dest_path);
    }

    Ok(())
}

pub(crate) fn write_output_file(output_path: &Path, content: &str) -> Result<(), WriteError> {
    // Create parent directories if they don't exist
    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent).map_err(|e| WriteError::Io {
            path: parent.to_path_buf(),
            source: e,
        })?;
    }

    // Write the content to the file
    fs::write(output_path, content).map_err(|e| WriteError::Io {
        path: output_path.to_path_buf(),
        source: e,
    })?;

    info!("Wrote output file: {:?}", output_path);
    Ok(())
}
