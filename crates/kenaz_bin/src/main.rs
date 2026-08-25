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
mod file_manager;
mod log;
mod sync;

use clap::Parser;
use cli::Cli;

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

    // Run the application
    app::App::run(cli)?;

    Ok(())
}
