//! # Kenaz
//!
//! The executable entry point for the Kenaz theme engine.
//!
//! Kenaz is a style transfer tool for editor themes. It extracts the mathematical
//! DNA (engrams) of existing themes and applies them to custom, minimal color palettes.
//!
//! ## Commands
//! - `kenaz forge <my_palette.toml> <output.json> -e/--engram "One"`: Generates a theme based on One.
//! - `kenaz list`: Lists all available styles in the database.
//! - `kenaz dev build` (dev only): Scrapes GitHub and builds the database.
//! - `kenaz dev export` (dev only): Exports a `.tar.gz` pack for releases.
//!

mod app;
mod cli;
mod log;
mod sync;

use clap::Parser;
use cli::{Cli, Commands};
use kenaz_core::{engram, util};

/// The main entry point of the CLI.
///
/// Initialized logging, parses command-line arguments, and routes execution
/// to the appropriate subcommand (build database, list styles, or forge theme).
fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    // Initialize the logger with the verbosity level requested by the user
    let filter = tracing_subscriber::EnvFilter::builder()
        .with_default_directive(log::verbosity_to_level(cli.verbose).into())
        .from_env_lossy();
    tracing_subscriber::fmt().with_env_filter(filter).init();

    // 0. Zero friction setup: auto download the database and styles if missing
    if !cli.command.prevents_autosync() && !engram::db::path().exists() {
        sync::sync_repo(false)?;
    }

    // Route execution based on the parsed subcommand
    match cli.command {
        Commands::Forge {
            palette,
            output,
            engram,
            force,
        } => {
            let mut app = app::App::new();
            app.try_build_context(palette, output, engram, force)?;
            app.build_theme()?;
        }
        Commands::List => {
            let conn = rusqlite::Connection::open(engram::db::path())?;
            engram::db::list_engrams(&conn)?;
        }
        Commands::Sync { full } => {
            sync::sync_repo(full)?;
        }
        Commands::Clean => {
            let cache_dir = util::cache_dir();
            std::fs::remove_dir_all(&cache_dir)?;
            std::fs::create_dir_all(&cache_dir)?;

            tracing::info!("Cache cleaned successfully!");
        }
        Commands::Doc { action } => {
            use cli::DocActions;
            match action {
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
                }
            }
        }
        #[cfg(feature = "dev-tools")]
        Commands::Dev { action } => {
            use cli::DevActions;
            use engram::devtools::prelude::*;

            match action {
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
        }
    }

    Ok(())
}
