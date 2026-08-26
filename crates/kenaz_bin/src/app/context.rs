use kenaz_core::{
    engram,
    palette::Palette,
    schema::{ThemeContent, ThemeFamilyContent, ThemeStyleContent},
    util,
    visitor::ColorMutable,
};
use std::path::PathBuf;

#[cfg(feature = "dev-tools")]
use crate::cli::DevActions;
use crate::{cli::DocActions, file_manager::FileManager};

pub struct ForgeContext {
    palette: Palette,
    output: PathBuf,
    engram_name: String,
    force: bool,
}

impl ForgeContext {
    pub fn build(
        palette_path: PathBuf,
        engram_name: String,
        output: PathBuf,
        force: bool,
    ) -> anyhow::Result<Self> {
        let toml = std::fs::read_to_string(palette_path)?;
        let palette = toml::from_str(&toml)?;

        Ok(Self {
            palette,
            engram_name,
            output,
            force,
        })
    }

    /// Builds the theme family by applying the engram to the palette
    ///
    /// This iterates through all variants (Dark/Light) defined in the palette.
    /// For each variant, it attempts to load the corresponding engram from the
    /// database. If the specific variant is not found, it falls back to the
    /// opposite variant to ensure generation does not fail.
    /// Finally, it writes the generated theme to the output path.
    pub fn execute(&self) -> anyhow::Result<()> {
        let conn = rusqlite::Connection::open(engram::db::path())?;

        let themes = self
            .palette
            .variants
            .iter()
            .map(|variant| {
                let engram = match engram::db::get_by_theme_name_and_variant(
                    &conn,
                    &self.engram_name,
                    &variant.mode,
                ) {
                    Ok(e) => e,
                    Err(_) => {
                        let requested_mode = variant.mode;
                        let fallback_mode = !variant.mode;

                        let mut fallback_engram = engram::db::get_by_theme_name_and_variant(
                            &conn,
                            &self.engram_name,
                            &fallback_mode,
                        )?;

                        if self.force {
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
                                self.engram_name,
                                fallback_mode
                            );
                        }

                        fallback_engram
                    }
                };

                tracing::info!(
                    "Loaded engram for theme '{}' with {} tokens",
                    &self.engram_name,
                    engram.len()
                );

                // Find the original theme JSON file to use as a "canvas" for structure
                let base_style_path = util::find_base_style(&self.engram_name)?;

                let file_bytes = std::fs::read(&base_style_path)?;

                // Parse into the strongly-typed Zed schema
                let mut style: ThemeStyleContent = serde_json::from_slice(&file_bytes)?;

                // Apply the style transfer math
                let mut path_buffer = String::with_capacity(64);
                style.apply_colors(&mut path_buffer, &engram, &variant.colors);

                Ok(ThemeContent {
                    name: variant.name.clone(),
                    appearance: variant.mode.into(),
                    style,
                })
            })
            .collect::<anyhow::Result<Vec<_>>>()?;

        let family = ThemeFamilyContent {
            name: self.palette.meta.name.clone(),
            author: "kenaz".to_string(),
            themes,
        };

        let file_manager = FileManager::new(&self.output);
        file_manager.backup()?;
        file_manager.save(serde_json::to_string_pretty(&family)?)?;
        Ok(())
    }
}

pub struct ListContext;
impl ListContext {
    pub fn execute() -> anyhow::Result<()> {
        let conn = rusqlite::Connection::open(engram::db::path())?;
        engram::db::list_engrams(&conn)?;
        Ok(())
    }
}

pub struct SyncContext {
    pub full: bool,
}

impl SyncContext {
    pub fn execute(self) -> anyhow::Result<()> {
        crate::sync::sync_repo(self.full)
    }
}

pub struct CleanContext;
impl CleanContext {
    pub fn execute() -> anyhow::Result<()> {
        let cache_dir = util::cache_dir();
        std::fs::remove_dir_all(&cache_dir)?;
        std::fs::create_dir_all(&cache_dir)?;
        tracing::info!("Cache cleaned successfully");
        Ok(())
    }
}

pub struct DocContext {
    pub action: DocActions,
}
impl DocContext {
    pub fn execute(self) -> anyhow::Result<()> {
        match self.action {
            DocActions::ShowPath => {
                let mut print_help = false;
                let mut missing = Vec::new();
                let cache_dir = util::cache_dir();
                if cache_dir.exists() && cache_dir.is_dir() {
                    println!("Cache directory at: {:?}", cache_dir);
                } else {
                    print_help = true;
                    missing.push(cache_dir);
                }

                let engrams_db_path = engram::db::path();
                if engrams_db_path.exists() && engrams_db_path.is_file() {
                    println!("Engrams database at: {:?}", engrams_db_path);
                } else {
                    print_help = true;
                    missing.push(engrams_db_path);
                }

                let styles_dir = util::styles_dir();
                if styles_dir.exists() && styles_dir.is_dir() {
                    println!("Styles directory at: {:?}", styles_dir);
                } else {
                    print_help = true;
                    missing.push(styles_dir);
                }

                if print_help {
                    println!("Kenaz cache missing :");
                    for path in missing {
                        println!("\t- {}", path.to_string_lossy());
                    }
                    println!("Try `kenaz sync` to build them");
                }

                Ok(())
            }
        }
    }
}

#[cfg(feature = "dev-tools")]
pub struct DevContext {
    pub action: DevActions,
}

#[cfg(feature = "dev-tools")]
impl DevContext {
    pub fn execute(self) -> anyhow::Result<()> {
        use engram::devtools::prelude::*;
        match self.action {
            DevActions::Build { skip_fetch } => {
                EngramBuilder::ensure_dot_env();
                let mut eb = EngramBuilder::new().skip_fetch(skip_fetch);
                eb.build_engrams()?;
            }
            DevActions::Export { full } => {
                let source_db = engram::db::path();
                let output_archive = if full {
                    std::path::PathBuf::from("./kenaz_full_pack.tar.gz")
                } else {
                    std::path::PathBuf::from("./kenaz_curated_pack.tar.gz")
                };
                tracing::info!("Exporting to {output_archive:?}");
                export_pack(&source_db, &output_archive, full)?;
            }
        }
        Ok(())
    }
}
