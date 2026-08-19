//! Manages the application state and orchestrate the theme generation process.
//!
//! This module ties everything together: it loads the user's palette, queries
//! the SQLite database for the requested style (engram), applies it to the Zed
//! theme schema using the `ColorMutable` trait, and writes the final JSON file
//! to disk, handling automatic backups of existing files.

use kenaz_core::{engram, palette::Palette, schema, util, visitor::ColorMutable};

use std::{
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

/// Holds the application state required to forge a new theme.
pub struct App {
    pub palette: Option<Palette>,
    pub style: String,
    pub output: PathBuf,
}

impl App {
    /// Create a new application state with the specified style and output path.
    pub fn new(style: String, output: PathBuf) -> Self {
        Self {
            palette: None,
            style,
            output,
        }
    }

    /// Parses and loads the user's color palette from a TOML file.
    pub fn parse_palette(&mut self, palette: PathBuf) -> anyhow::Result<()> {
        let toml = std::fs::read_to_string(palette)?;
        let palette = toml::from_str(&toml)?;
        self.palette = Some(palette);
        Ok(())
    }

    /// Builds the theme family by applying the engram to the palette
    ///
    /// This iterates through all variants (Dark/Light) defined in the palette.
    /// For each variant, it attempts to load the corresponding engram from the
    /// database. If the specific variant is not found, it falls back to the
    /// opposite variant to ensure generation does not fail.
    /// Finally, it writes the generated theme to the output path.
    pub fn build_theme(&self) -> anyhow::Result<()> {
        let palette = self.palette.as_ref().expect("Palette loaded successfully");
        let conn = rusqlite::Connection::open(util::engrams_db_path())?;

        let themes = palette
            .variants
            .iter()
            .map(|variant| {
                let engram = match engram::db::get_by_theme_name_and_variant(
                    &conn,
                    &self.style,
                    &variant.mode,
                ) {
                    Ok(e) => e,
                    Err(_) => {
                        let prev = variant.mode;
                        let fallback = !variant.mode;
                        tracing::warn!(
                            "Variant '{:?}' not found for {}, stricly using '{:?}' instead",
                            prev,
                            self.style,
                            fallback
                        );
                        engram::db::get_by_theme_name_and_variant(&conn, &self.style, &fallback)?
                    }
                };

                tracing::info!(
                    "Loaded engram for theme '{}' with {} tokens",
                    &self.style,
                    engram.len()
                );

                // TODO: handle engram inversion if wanted by the user

                // Find the original theme JSON file to use as a "canvas" for structure
                let base_style_value = util::find_base_style(&self.style)?;

                // Parse into the strongly-typed Zed schema
                let mut style: schema::ThemeStyleContent =
                    serde_json::from_value(base_style_value)?;

                // Apply the style transfer math
                style.apply_colors("", &engram, &variant.colors);

                Ok(schema::ThemeContent {
                    name: variant.name.clone(),
                    appearance: variant.mode.into(),
                    style,
                })
            })
            .collect::<anyhow::Result<Vec<_>>>()?;

        let family = schema::ThemeFamilyContent {
            name: palette.meta.name.clone(),
            author: "kenaz".to_string(),
            themes,
        };

        // Backup existing file before overwriting
        self.backup_file()?;

        std::fs::write(&self.output, serde_json::to_string_pretty(&family)?)?;
        tracing::info!("wrote theme in {:?}", self.output);

        Ok(())
    }

    /// Creates a timestamped backup of the output file if it already exists
    ///
    /// This prevents data loss when iterating on a theme. The backup is named
    /// using a Unix timestamp (e.g., `theme_1692198400.json.bak`).
    fn backup_file(&self) -> anyhow::Result<()> {
        if !self.output.exists() {
            return Ok(());
        }

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);

        let stem = self
            .output
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("theme");
        let extension = self
            .output
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("json");

        let backup_name = format!("{}_{}.{}", stem, timestamp, extension);

        let mut backup_path = self.output.clone();
        backup_path.set_file_name(backup_name);

        std::fs::rename(&self.output, &backup_path)?;
        tracing::info!("Backup create at: {:?}", backup_path);

        Ok(())
    }
}
