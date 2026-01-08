// src/asset_hash.rs

use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};
use tracing::{debug, info};
use walkdir::WalkDir;

use crate::error::StaticError;

/// Maps original asset paths to their hashed versions.
/// Key: relative path from static dir (e.g., "css/style.css")
/// Value: hashed path (e.g., "/static/css/style.a1b2c3d4.css")
pub(crate) type AssetManifest = HashMap<String, String>;

/// Regex pattern for detecting previously hashed files: name.XXXXXXXX.ext
/// where XXXXXXXX is exactly 8 hex characters
fn is_hashed_filename(filename: &str) -> bool {
    let parts: Vec<&str> = filename.rsplitn(2, '.').collect();
    if parts.len() != 2 {
        return false;
    }
    let ext = parts[0];
    let name_with_hash = parts[1];

    // Must be .css or .js
    if ext != "css" && ext != "js" {
        return false;
    }

    // Check for hash pattern: name.XXXXXXXX
    let hash_parts: Vec<&str> = name_with_hash.rsplitn(2, '.').collect();
    if hash_parts.len() != 2 {
        return false;
    }

    let potential_hash = hash_parts[0];
    potential_hash.len() == 8 && potential_hash.chars().all(|c| c.is_ascii_hexdigit())
}

/// Computes an 8-character BLAKE3 hash from file content.
fn compute_file_hash(path: &Path) -> Result<String, StaticError> {
    let content = fs::read(path).map_err(|e| StaticError::Io {
        path: path.to_path_buf(),
        source: e,
    })?;

    let hash = blake3::hash(&content);
    // Take first 8 hex characters (4 bytes)
    Ok(hash.to_hex()[..8].to_string())
}

/// Generates a hashed filename: name.XXXXXXXX.ext
fn hashed_filename(original: &str, hash: &str) -> String {
    if let Some(dot_pos) = original.rfind('.') {
        let (name, ext) = original.split_at(dot_pos);
        format!("{}.{}{}", name, hash, ext)
    } else {
        format!("{}.{}", original, hash)
    }
}

/// Cleans up old hashed files from the output static directory.
/// Removes files matching the pattern: name.XXXXXXXX.css/js
pub(crate) fn cleanup_old_hashed_files(output_static_dir: &Path) -> Result<usize, StaticError> {
    if !output_static_dir.exists() {
        return Ok(0);
    }

    let mut removed_count = 0;

    for entry in WalkDir::new(output_static_dir)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
    {
        let path = entry.path();
        if let Some(filename) = path.file_name().and_then(|n| n.to_str())
            && is_hashed_filename(filename)
        {
            debug!("asset_hash::cleanup {:?}", path);
            fs::remove_file(path).map_err(|e| StaticError::Io {
                path: path.to_path_buf(),
                source: e,
            })?;
            removed_count += 1;
        }
    }

    if removed_count > 0 {
        info!("asset_hash::cleanup {} old hashed files", removed_count);
    }

    Ok(removed_count)
}

/// Hashes CSS and JS files in the static directory and copies them to output.
/// Returns a manifest mapping original paths to hashed URLs.
pub(crate) fn hash_static_assets(
    static_dir: &str,
    output_dir: &str,
) -> Result<AssetManifest, StaticError> {
    let static_path = PathBuf::from(static_dir);
    let output_static_path = PathBuf::from(output_dir).join("static");

    if !static_path.exists() {
        debug!("asset_hash::scan no static directory");
        return Ok(HashMap::new());
    }

    // Clean up old hashed files first
    cleanup_old_hashed_files(&output_static_path)?;

    // Ensure output static directory exists
    fs::create_dir_all(&output_static_path).map_err(|e| StaticError::Io {
        path: output_static_path.clone(),
        source: e,
    })?;

    let mut manifest = HashMap::new();
    let mut hashed_count = 0;

    for entry in WalkDir::new(&static_path)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
    {
        let source_path = entry.path();
        let relative_path =
            source_path
                .strip_prefix(&static_path)
                .map_err(|e| StaticError::Io {
                    path: source_path.to_path_buf(),
                    source: std::io::Error::other(e),
                })?;

        // Only hash .css and .js files
        let extension = source_path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("");

        if extension != "css" && extension != "js" {
            continue;
        }

        // Skip already-hashed files (from previous builds that weren't cleaned)
        if let Some(filename) = source_path.file_name().and_then(|n| n.to_str())
            && is_hashed_filename(filename)
        {
            continue;
        }

        // Compute hash
        let hash = compute_file_hash(source_path)?;

        // Generate hashed filename
        let original_filename = source_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("");
        let hashed_name = hashed_filename(original_filename, &hash);

        // Build destination path with hashed filename
        let dest_relative = relative_path.parent().map_or_else(
            || PathBuf::from(&hashed_name),
            |parent| parent.join(&hashed_name),
        );
        let dest_path = output_static_path.join(&dest_relative);

        // Create parent directories
        if let Some(parent) = dest_path.parent() {
            fs::create_dir_all(parent).map_err(|e| StaticError::Io {
                path: parent.to_path_buf(),
                source: e,
            })?;
        }

        // Copy file with hashed name
        fs::copy(source_path, &dest_path).map_err(|e| StaticError::Io {
            path: dest_path.clone(),
            source: e,
        })?;

        debug!("asset_hash::copy {:?} → {:?}", relative_path, dest_relative);

        // Add to manifest: "css/style.css" -> "/static/css/style.a1b2c3d4.css"
        let original_key = relative_path.to_string_lossy().replace('\\', "/");
        let hashed_url = format!(
            "/static/{}",
            dest_relative.to_string_lossy().replace('\\', "/")
        );
        manifest.insert(original_key, hashed_url);

        hashed_count += 1;
    }

    if hashed_count > 0 {
        info!("asset_hash::hash {} files", hashed_count);
    }

    Ok(manifest)
}

/// Resolves an asset path using the manifest.
/// If the path is in the manifest, returns the hashed URL.
/// Otherwise, returns the original path with a leading slash.
#[cfg(test)]
fn resolve_asset_path(manifest: &AssetManifest, path: &str) -> String {
    // Normalize the path: remove leading "static/" or "/static/" if present
    let normalized = path.trim_start_matches('/').trim_start_matches("static/");

    if let Some(hashed) = manifest.get(normalized) {
        hashed.clone()
    } else {
        // Return original path, ensuring it starts with /static/
        if path.starts_with("/static/") {
            path.to_string()
        } else if path.starts_with("static/") {
            format!("/{}", path)
        } else {
            format!("/static/{}", path)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_is_hashed_filename() {
        // Valid hashed filenames
        assert!(is_hashed_filename("style.a1b2c3d4.css"));
        assert!(is_hashed_filename("app.12345678.js"));
        assert!(is_hashed_filename("main.abcdef12.css"));

        // Invalid: wrong extension
        assert!(!is_hashed_filename("image.a1b2c3d4.png"));
        assert!(!is_hashed_filename("font.12345678.woff"));

        // Invalid: wrong hash length
        assert!(!is_hashed_filename("style.a1b2c3.css")); // 6 chars
        assert!(!is_hashed_filename("style.a1b2c3d4e5.css")); // 10 chars

        // Invalid: non-hex characters
        assert!(!is_hashed_filename("style.ghijklmn.css"));

        // Invalid: no hash
        assert!(!is_hashed_filename("style.css"));
        assert!(!is_hashed_filename("app.js"));

        // Invalid: no extension
        assert!(!is_hashed_filename("style.a1b2c3d4"));
    }

    #[test]
    fn test_compute_file_hash() {
        let temp_dir = tempdir().unwrap();
        let file_path = temp_dir.path().join("test.css");

        let mut file = fs::File::create(&file_path).unwrap();
        file.write_all(b"body { color: red; }").unwrap();

        let hash = compute_file_hash(&file_path).unwrap();

        assert_eq!(hash.len(), 8);
        assert!(hash.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn test_compute_file_hash_deterministic() {
        let temp_dir = tempdir().unwrap();
        let file1 = temp_dir.path().join("file1.css");
        let file2 = temp_dir.path().join("file2.css");

        // Same content should produce same hash
        fs::write(&file1, "body { color: blue; }").unwrap();
        fs::write(&file2, "body { color: blue; }").unwrap();

        let hash1 = compute_file_hash(&file1).unwrap();
        let hash2 = compute_file_hash(&file2).unwrap();

        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_compute_file_hash_different_content() {
        let temp_dir = tempdir().unwrap();
        let file1 = temp_dir.path().join("file1.css");
        let file2 = temp_dir.path().join("file2.css");

        fs::write(&file1, "body { color: blue; }").unwrap();
        fs::write(&file2, "body { color: red; }").unwrap();

        let hash1 = compute_file_hash(&file1).unwrap();
        let hash2 = compute_file_hash(&file2).unwrap();

        assert_ne!(hash1, hash2);
    }

    #[test]
    fn test_hashed_filename() {
        assert_eq!(
            hashed_filename("style.css", "a1b2c3d4"),
            "style.a1b2c3d4.css"
        );
        assert_eq!(hashed_filename("app.js", "12345678"), "app.12345678.js");
        assert_eq!(
            hashed_filename("my.component.css", "abcdef12"),
            "my.component.abcdef12.css"
        );
    }

    #[test]
    fn test_hash_static_assets() {
        let temp_dir = tempdir().unwrap();
        let static_dir = temp_dir.path().join("static");
        let output_dir = temp_dir.path().join("output");

        // Create static directory structure
        fs::create_dir_all(static_dir.join("css")).unwrap();
        fs::create_dir_all(static_dir.join("js")).unwrap();

        // Create test files
        fs::write(static_dir.join("css/style.css"), "body { margin: 0; }").unwrap();
        fs::write(static_dir.join("js/app.js"), "console.log('hello');").unwrap();
        fs::write(static_dir.join("image.png"), "not css or js").unwrap();

        let manifest =
            hash_static_assets(static_dir.to_str().unwrap(), output_dir.to_str().unwrap()).unwrap();

        // Should have entries for CSS and JS
        assert_eq!(manifest.len(), 2);
        assert!(manifest.contains_key("css/style.css"));
        assert!(manifest.contains_key("js/app.js"));

        // Should not have entry for PNG
        assert!(!manifest.contains_key("image.png"));

        // Verify hashed files were created
        let css_hashed = manifest.get("css/style.css").unwrap();
        let js_hashed = manifest.get("js/app.js").unwrap();

        // Extract filename from path and check it exists
        let css_filename = css_hashed.trim_start_matches("/static/");
        let js_filename = js_hashed.trim_start_matches("/static/");

        assert!(output_dir.join("static").join(css_filename).exists());
        assert!(output_dir.join("static").join(js_filename).exists());
    }

    #[test]
    fn test_cleanup_old_hashed_files() {
        let temp_dir = tempdir().unwrap();
        let output_static = temp_dir.path().join("static");
        fs::create_dir_all(&output_static).unwrap();

        // Create some hashed files
        fs::write(output_static.join("style.a1b2c3d4.css"), "old").unwrap();
        fs::write(output_static.join("app.12345678.js"), "old").unwrap();
        // Create a regular file (should not be removed)
        fs::write(output_static.join("style.css"), "keep").unwrap();

        let removed = cleanup_old_hashed_files(&output_static).unwrap();

        assert_eq!(removed, 2);
        assert!(!output_static.join("style.a1b2c3d4.css").exists());
        assert!(!output_static.join("app.12345678.js").exists());
        assert!(output_static.join("style.css").exists());
    }

    #[test]
    fn test_resolve_asset_path_with_manifest() {
        let mut manifest = HashMap::new();
        manifest.insert(
            "css/style.css".to_string(),
            "/static/css/style.a1b2c3d4.css".to_string(),
        );

        // Various input formats should resolve correctly
        assert_eq!(
            resolve_asset_path(&manifest, "css/style.css"),
            "/static/css/style.a1b2c3d4.css"
        );
        assert_eq!(
            resolve_asset_path(&manifest, "static/css/style.css"),
            "/static/css/style.a1b2c3d4.css"
        );
        assert_eq!(
            resolve_asset_path(&manifest, "/static/css/style.css"),
            "/static/css/style.a1b2c3d4.css"
        );
    }

    #[test]
    fn test_resolve_asset_path_without_manifest() {
        let manifest = HashMap::new();

        // Should return normalized path when not in manifest
        assert_eq!(
            resolve_asset_path(&manifest, "css/style.css"),
            "/static/css/style.css"
        );
        assert_eq!(
            resolve_asset_path(&manifest, "static/css/style.css"),
            "/static/css/style.css"
        );
        assert_eq!(
            resolve_asset_path(&manifest, "/static/css/style.css"),
            "/static/css/style.css"
        );
    }
}
