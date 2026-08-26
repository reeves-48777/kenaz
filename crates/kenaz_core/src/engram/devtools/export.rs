//! Logic for exporting the engram database and styles into distributable archives.

use crate::{
    engram::db::{ConnectionExt, prelude::EngramRecord},
    error::Result,
    util,
};
use std::path::Path;

/// The list of theme to include in the lightweight "Curated Pack".
/// These are the themes that will be downloaded by end-users.
const CURATED_REPOS: &[&str] = &[
    "zed_official",
    "catppuccin",
    "tokyo night themes",
    "nord",
    "ashen",
    "dracula",
];

/// Exports a curated subset (or full) of the database and its JSON canvas files.
///
/// This creates a lightweight `engrams.db` and a `styles/` folder containing
/// only the themes defined in `CURATED_REPOS` (unless `full` is true).
/// It then packs them into a `.tar.gz` archive for GitHub releases.
pub fn export_pack(source_db_path: &Path, output_archive: &Path, full: bool) -> Result<()> {
    use flate2::{Compression, write::GzEncoder};
    use tar::Builder;

    // 1. Create temporary directory to prepare file structure
    let temp_dir = std::env::temp_dir().join("kenaz_export_tmp");
    if temp_dir.exists() {
        std::fs::remove_dir_all(&temp_dir)?;
    }
    std::fs::create_dir_all(&temp_dir)?;

    let dest_db_path = temp_dir.join("engrams.db");
    let dest_styles_dir = temp_dir.join("styles");
    std::fs::create_dir_all(&dest_styles_dir)?;

    // 2. Copy the curated canvas folders and extract theme names
    let source_style_dir = util::styles_dir();
    let mut theme_names_to_export = Vec::new();

    // If `full` is true, we gather all cache subdirectories
    // Else we only browse CURATED_REPOS list (case-insensitive)
    let repos_to_export: Vec<String> = if full {
        std::fs::read_dir(&source_style_dir)?
            .filter_map(|e| {
                let path = e.ok()?.path();
                if path.is_dir() {
                    path.file_name()?.to_str().map(|s| s.to_string())
                } else {
                    None
                }
            })
            .collect()
    } else {
        CURATED_REPOS.iter().map(|r| r.to_string()).collect()
    };

    for repo_name in repos_to_export {
        // FIX: case-insensitive search directory
        let src_repo_dir = std::fs::read_dir(&source_style_dir)?
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .find(|path| {
                path.is_dir()
                    && path
                        .file_name()
                        .and_then(|n| n.to_str())
                        .map(|n| n.eq_ignore_ascii_case(&repo_name))
                        .unwrap_or(false)
            });

        let Some(src_repo_dir) = src_repo_dir else {
            tracing::warn!("Curated repo '{repo_name}' not found in cache, skipping.");
            continue;
        };

        let dest_repo_dir = dest_styles_dir.join(&repo_name);
        std::fs::create_dir_all(&dest_repo_dir)?;

        for file_entry in std::fs::read_dir(&src_repo_dir)? {
            let file_path = file_entry?.path();
            if !file_path.is_file() || file_path.extension().map_or(true, |ext| ext != "json") {
                continue;
            }

            let dest_file = dest_repo_dir.join(file_path.file_name().unwrap());
            std::fs::copy(&file_path, &dest_file)?;

            // Get theme name for database (curated mode only e.g., not full)
            if !full {
                if let Ok(content) = std::fs::read_to_string(&file_path) {
                    if let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) {
                        if let Some(themes) = json.get("themes").and_then(|t| t.as_array()) {
                            for theme in themes {
                                if let Some(name) = theme.get("name").and_then(|n| n.as_str()) {
                                    theme_names_to_export.push(util::normalize_theme_name(name));
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // 3. Export database subset
    let source_conn = rusqlite::Connection::open(source_db_path)?;

    if full {
        // FIX: in full mode, we copy database file, optimisation (instant and safe)
        tracing::info!("Copying full database...");
        source_conn.close_safely()?;
        std::fs::copy(source_db_path, &dest_db_path)?;
    } else if !theme_names_to_export.is_empty() {
        // FIX: avoid crash if empty
        let dest_conn = rusqlite::Connection::open(&dest_db_path)?;

        // Initialize schema in destination
        EngramRecord::create_table_if_not_exists(&dest_conn)?;

        let dest_db_path_str = dest_db_path.to_str().unwrap().replace("'", "''");

        // Attach destination DB to source to copy data easily
        source_conn.execute(
            &format!("ATTACH DATABASE '{}' AS dest", dest_db_path_str),
            [],
        )?;

        // Build the SQL IN clause: ('one dark', 'monokai', ...)
        let theme_names_sql = theme_names_to_export
            .iter()
            .map(|t| format!("'{}'", t.replace("'", "''")))
            .collect::<Vec<_>>()
            .join(", ");

        tracing::info!(
            "Exporting {} themes to curated database...",
            theme_names_to_export.len()
        );

        // Copy matching rows
        source_conn.execute(&format!("
        INSERT INTO dest.engrams (theme_name, variant, token_path, op_type, w_bg, w_fg, w_accent, w_success, w_warning, w_error, delta_l, alpha)
        SELECT theme_name, variant, token_path, op_type, w_bg, w_fg, w_accent, w_success, w_warning, w_error, delta_l, alpha
        FROM engrams
        WHERE LOWER(theme_name) IN ({})", theme_names_sql), [])?;

        source_conn.execute("DETACH DATABASE dest", [])?;
        dest_conn.close_safely()?;
        source_conn.close_safely()?;
    } else {
        tracing::warn!("No themes found to export to curated database");
    }

    // 4. Creating tar.gz archive
    tracing::info!("Packing into {output_archive:?}");
    let tar_gz = std::fs::File::create(output_archive)?;
    let enc = GzEncoder::new(tar_gz, Compression::default());
    let mut tar = Builder::new(enc);

    tar.append_dir_all("kenaz_pack", &temp_dir)?;
    tar.finish()?;

    std::fs::remove_dir_all(&temp_dir)?;

    tracing::info!("Pack successfully created at {output_archive:?}");

    Ok(())
}
