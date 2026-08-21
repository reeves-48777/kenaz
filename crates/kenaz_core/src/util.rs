//! Utility functions for path resolutions, theme name normalization and cache management.
//!
//! This module handles cross-platform path resolution (finding where to store the SQLite
//! database and temporary files) and provides helpers to locate the raw JSON structure
//! of a theme to use as a "canvas" for color replacement.

use std::path::PathBuf;

/// Returns the root cache directory
///
/// This directory is the parent of the downloaded pack
/// holding the engrams.db SQLite database and styles directory
pub fn cache_dir() -> PathBuf {
    if let Ok(test_dir) = std::env::var("KENAZ_TEST_CACHE_DIR") {
        return PathBuf::from(test_dir);
    }

    let mut path = dirs::cache_dir().expect("Successfully got cache directory");
    path.push("kenaz");
    path
}

/// Returns the directory used to cache raw JSON themes fetched from Github.
///
/// This is primarily used by the `dev-tools` feature to store downloaded repositories
/// before extracting their engrams.
pub fn styles_dir() -> PathBuf {
    let mut path = cache_dir();
    path.push("styles");
    path
}

/// Normalizes a theme name for consistent database lookup and comparison
///
/// Converts the name to lowercase and replaces any non-alphanumerci characters
/// with hyphens, removing consecutive or leading/trailing hyphens.
/// (e.g., "Gruvbox Material Light" becomes "gruvbox-material-light").
pub fn normalize_theme_name(name: &str) -> String {
    name.to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

/// Searches the temporary styles directory for a specific theme's raw `style` JSON.
///
/// This acts as a "canvas" for engram application. Instead of building a theme
/// from scratch, Kenaz find the original theme file, loads its `style` object,
/// and recursively replaces its colors.
///
/// # Arguments
/// * `style_name` - The name of the theme to find (will be normalized).
///
/// # Errors
/// Returns an error if the directory cannot be read or if the theme is not found.
pub fn find_base_style(style_name: &str) -> anyhow::Result<serde_json::Value> {
    let dir = styles_dir();

    let normalized_target = normalize_theme_name(style_name);

    // 1. Iterate through all repository folders in the temp directory
    for repo_entry in std::fs::read_dir(&dir)? {
        let repo_dir = repo_entry?.path();

        if !repo_dir.is_dir() {
            continue;
        }

        // 2. Iterate through all JSON files inside the repository folder
        for file_entry in std::fs::read_dir(&repo_dir)? {
            let file_path = file_entry?.path();

            if !file_path.is_file() || file_path.extension().map_or(true, |ext| ext != "json") {
                continue;
            }

            // 3. Parse the JSON file and look for the requested theme
            if let Ok(content) = std::fs::read_to_string(&file_path) {
                if let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) {
                    if let Some(themes) = json.get("themes").and_then(|t| t.as_array()) {
                        for theme in themes {
                            if let Some(name) = theme.get("name").and_then(|n| n.as_str()) {
                                if normalize_theme_name(name) == normalized_target {
                                    if let Some(style) = theme.get("style") {
                                        tracing::debug!(
                                            "Found base style for {style_name} in {file_path:?}"
                                        );
                                        return Ok(style.clone());
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    Err(anyhow::anyhow!(
        "Base theme style not found for {style_name}"
    ))
}
