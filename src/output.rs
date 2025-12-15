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
    let static_dir = &config.site.static_dir;
    info!("Processing static directory: {}", static_dir);

    // Check if static directory exists
    if !PathBuf::from(static_dir).exists() {
        info!("No static directory found at: {}", static_dir);
        return Ok(());
    }

    // Create the static directory in the output folder
    let output_static_dir = PathBuf::from(&config.site.output_dir).join("static");
    fs::create_dir_all(&output_static_dir).map_err(|e| StaticError::Io {
        path: output_static_dir.clone(),
        source: e,
    })?;

    // Copy all files recursively, excluding root_static files
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
                source: std::io::Error::other(e),
            })?;

        // Skip files that are configured as root_static
        let relative_path_str = relative_path.to_string_lossy();
        if config
            .site
            .root_static
            .values()
            .any(|src| src == &*relative_path_str)
        {
            debug!("Skipping root static file: {:?}", source_path);
            continue;
        }

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

    // Copy root-level static files to output root
    copy_root_static_files(config)?;

    Ok(())
}

/// Copies configured root static files to the output directory root.
fn copy_root_static_files(config: &Config) -> Result<(), StaticError> {
    if config.site.root_static.is_empty() {
        debug!("No root static files configured.");
        return Ok(());
    }

    let static_dir = PathBuf::from(&config.site.static_dir);
    let output_dir = PathBuf::from(&config.site.output_dir);

    for (output_filename, source_relative_path) in &config.site.root_static {
        let source_path = static_dir.join(source_relative_path);

        if !source_path.exists() {
            return Err(StaticError::Io {
                path: source_path.clone(),
                source: std::io::Error::new(
                    ErrorKind::NotFound,
                    format!("Root static file not found: {:?}", source_path),
                ),
            });
        }

        let dest_path = output_dir.join(output_filename);

        // Create parent directories if needed (though typically not needed for root files)
        if let Some(parent) = dest_path.parent() {
            fs::create_dir_all(parent).map_err(|e| StaticError::Io {
                path: parent.to_path_buf(),
                source: e,
            })?;
        }

        fs::copy(&source_path, &dest_path).map_err(|e| StaticError::Io {
            path: dest_path.clone(),
            source: e,
        })?;

        info!(
            "Copied root static file: {:?} -> {:?}",
            source_path, dest_path
        );
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::fs::{self, File};
    use std::io::Write;
    use tempfile::tempdir;

    fn create_test_config_with_root_static() -> Config {
        let mut root_static = HashMap::new();
        root_static.insert("favicon.ico".to_string(), "favicon.ico".to_string());
        root_static.insert("robots.txt".to_string(), "seo/robots.txt".to_string());

        Config {
            site: crate::config::SiteConfig {
                title: "Test Site".to_string(),
                tagline: "Hello world".to_string(),
                domain: "test.com".to_string(),
                author: "Test Author".to_string(),
                output_dir: "out".to_string(),
                content_dir: "src/content".to_string(),
                template_dir: "templates".to_string(),
                static_dir: "static".to_string(),
                site_index_template: "site_index.html".to_string(),
                content_types: HashMap::new(),
                root_static,
            },
            dynamic: HashMap::new(),
        }
    }

    #[test]
    fn test_copy_root_static_files() {
        let temp_dir = tempdir().unwrap();
        let static_dir = temp_dir.path().join("static");
        let output_dir = temp_dir.path().join("out");

        // Create static directory
        fs::create_dir_all(&static_dir).unwrap();
        fs::create_dir_all(static_dir.join("seo")).unwrap();
        fs::create_dir_all(&output_dir).unwrap();

        // Create test files
        let favicon_path = static_dir.join("favicon.ico");
        let robots_path = static_dir.join("seo/robots.txt");
        let css_path = static_dir.join("style.css");

        File::create(&favicon_path)
            .unwrap()
            .write_all(b"favicon data")
            .unwrap();
        File::create(&robots_path)
            .unwrap()
            .write_all(b"robots content")
            .unwrap();
        File::create(&css_path)
            .unwrap()
            .write_all(b"body { color: red; }")
            .unwrap();

        let mut config = create_test_config_with_root_static();
        config.site.static_dir = static_dir.to_string_lossy().to_string();
        config.site.output_dir = output_dir.to_string_lossy().to_string();

        // Test copying root static files
        copy_static_files(&config).unwrap();

        // Verify root static files are copied to output root
        assert!(output_dir.join("favicon.ico").exists());
        assert!(output_dir.join("robots.txt").exists());

        // Verify regular static files are copied to static subdirectory
        assert!(output_dir.join("static/style.css").exists());

        // Verify content is correct
        let favicon_content = fs::read_to_string(output_dir.join("favicon.ico")).unwrap();
        assert_eq!(favicon_content, "favicon data");

        let robots_content = fs::read_to_string(output_dir.join("robots.txt")).unwrap();
        assert_eq!(robots_content, "robots content");
    }

    #[test]
    fn test_copy_root_static_files_missing_source() {
        let temp_dir = tempdir().unwrap();
        let static_dir = temp_dir.path().join("static");
        let output_dir = temp_dir.path().join("out");

        fs::create_dir_all(&static_dir).unwrap();
        fs::create_dir_all(&output_dir).unwrap();

        let mut config = create_test_config_with_root_static();
        config.site.static_dir = static_dir.to_string_lossy().to_string();
        config.site.output_dir = output_dir.to_string_lossy().to_string();

        // Don't create the source files - should error
        let result = copy_static_files(&config);
        assert!(result.is_err());

        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("Root static file not found"));
    }

    #[test]
    fn test_copy_static_files_no_root_static() {
        let temp_dir = tempdir().unwrap();
        let static_dir = temp_dir.path().join("static");
        let output_dir = temp_dir.path().join("out");

        fs::create_dir_all(&static_dir).unwrap();
        fs::create_dir_all(&output_dir).unwrap();

        // Create a regular static file
        let css_path = static_dir.join("style.css");
        File::create(&css_path)
            .unwrap()
            .write_all(b"body { color: red; }")
            .unwrap();

        let mut config = create_test_config_with_root_static();
        config.site.static_dir = static_dir.to_string_lossy().to_string();
        config.site.output_dir = output_dir.to_string_lossy().to_string();
        config.site.root_static.clear(); // No root static files

        copy_static_files(&config).unwrap();

        // Verify regular static file is copied to static subdirectory
        assert!(output_dir.join("static/style.css").exists());

        // Verify no root files were created
        assert!(!output_dir.join("favicon.ico").exists());
        assert!(!output_dir.join("robots.txt").exists());
    }
}
