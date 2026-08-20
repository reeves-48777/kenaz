//! Manages the application state and orchestrate the theme generation process.
//!
//! This module ties everything together: it loads the user's palette, queries
//! the SQLite database for the requested style (engram), applies it to the Zed
//! theme schema using the `ColorMutable` trait, and writes the final JSON file
//! to disk, handling automatic backups of existing files.

use kenaz_core::{engram, palette::Palette, schema, util, visitor::ColorMutable};

use std::{
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

/// Execution context of all user configurations
pub struct Context {
    pub palette: Palette,
    pub engram_name: String,
    pub output: PathBuf,
    pub force_engram_use: bool,
}

/// Holds the application state required to forge a new theme.
pub struct App {
    ctx: Option<Context>,
}

impl App {
    /// Create a new application state with the specified style and output path.
    pub fn new() -> Self {
        Self { ctx: None }
    }

    pub fn try_build_context(
        &mut self,
        palette_path: PathBuf,
        output: PathBuf,
        engram: String,
        force: bool,
    ) -> anyhow::Result<()> {
        let toml = std::fs::read_to_string(palette_path)?;
        let palette = toml::from_str(&toml)?;

        self.ctx = Some(Context {
            palette,
            engram_name: util::normalize_theme_name(&engram),
            output,
            force_engram_use: force,
        });
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
        let ctx = self.ctx.as_ref().expect("Context has been built");

        let conn = rusqlite::Connection::open(engram::db::path())?;

        let themes = ctx
            .palette
            .variants
            .iter()
            .map(|variant| {
                let engram = match engram::db::get_by_theme_name_and_variant(
                    &conn,
                    &ctx.engram_name,
                    &variant.mode,
                ) {
                    Ok(e) => e,
                    Err(_) => {
                        let requested_mode = variant.mode;
                        let fallback_mode = !variant.mode;

                        let mut fallback_engram = engram::db::get_by_theme_name_and_variant(
                            &conn,
                            &ctx.engram_name,
                            &fallback_mode,
                        )?;

                        if ctx.force_engram_use {
                            // User asked explicitly to invert engram
                            tracing::info!(
                                "--force used: applying inversion on {:?} engram to fit {:?} theme",
                                fallback_mode,
                                requested_mode
                            );

                            for vector in fallback_engram.values_mut() {
                                *vector = vector.invert();
                            }
                        } else {
                            // We use the engram as it is (default behaviour)
                            tracing::info!(
                                "Variant '{:?}' not found for {}, stricly using '{:?}' instead",
                                requested_mode,
                                ctx.engram_name,
                                fallback_mode
                            );
                        }

                        fallback_engram
                    }
                };

                tracing::info!(
                    "Loaded engram for theme '{}' with {} tokens",
                    &ctx.engram_name,
                    engram.len()
                );

                // Find the original theme JSON file to use as a "canvas" for structure
                let base_style_value = util::find_base_style(&ctx.engram_name)?;

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
            name: ctx.palette.meta.name.clone(),
            author: "kenaz".to_string(),
            themes,
        };

        // Backup existing file before overwriting
        self.backup_file(&ctx.output)?;

        std::fs::write(&ctx.output, serde_json::to_string_pretty(&family)?)?;
        tracing::info!("wrote theme in {:?}", ctx.output);

        Ok(())
    }

    /// Creates a timestamped backup of the output file if it already exists
    ///
    /// This prevents data loss when iterating on a theme. The backup is named
    /// using a Unix timestamp (e.g., `theme_1692198400.json.bak`).
    fn backup_file(&self, output: &Path) -> anyhow::Result<()> {
        if !output.exists() {
            return Ok(());
        }

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);

        let stem = output
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("theme");
        let extension = output
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("json");

        let backup_name = format!("{}_{}.{}", stem, timestamp, extension);

        let mut backup_path = output.to_path_buf();
        backup_path.set_file_name(backup_name);

        std::fs::rename(&output, &backup_path)?;
        tracing::info!("Backup create at: {:?}", backup_path);

        Ok(())
    }
}
