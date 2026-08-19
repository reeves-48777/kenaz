//! # Kenaz
//!
//! The executable entry point for the Kenaz theme engine.
//!
//! Kenaz is a style transfer tool for editor themes. It extracts the mathematical
//! DNA (engrams) of existing themes and applies them to custom, minimal color palettes.
//!
//! ## Commands
//! - `kenaz my_palette.toml output.json -e/--engram "One"`: Generates a theme based on One.
//! - `kenaz --list-engrams`: Lists all available styles in the database.
//! - `kenaz --build-engrams` (dev only): Scrapes GitHub and builds the database.
//!

mod app;
mod cli;
mod log;

use app::App;
use clap::Parser;
use cli::Cli;
use kenaz_core::util;

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

    // 1.Developer Tools: Build the engram database from Zed's ecosystem
    #[cfg(feature = "dev-tools")]
    if cli.build_engrams {
        let rt = tokio::runtime::Runtime::new()?;
        rt.block_on(async {
            use kenaz_core::engram::devtools::EngramBuilder;
            EngramBuilder::ensure_dot_env();
            let mut eb = EngramBuilder::new()
                .skip_fetch(cli.skip_fetch)
                .try_init_client()?;
            eb.build_engrams().await
        })?;
        return Ok(());
    }

    // 2. Utility: List available engrams directly from the SQLite databse
    if cli.list_engrams {
        use kenaz_core::engram::prelude::list_engrams;
        let conn = rusqlite::Connection::open(util::engrams_db_path())?;
        list_engrams(&conn)?;
        return Ok(());
    }

    // 3. Default Action: Forge a new theme using the provided palette, style and output path
    let mut app = App::new(
        cli.engram,
        cli.output.expect("output path passed as argument"),
    );
    app.parse_palette(cli.palette.expect("palette passed as argument"))?;
    app.build_theme()?;

    Ok(())
}
