//! Manages the application state and orchestrate the theme generation process.
//!
//! This module ties everything together: it loads the user's palette, queries
//! the SQLite database for the requested style (engram), applies it to the Zed
//! theme schema using the `ColorMutable` trait, and writes the final JSON file
//! to disk, handling automatic backups of existing files.

mod context;

use kenaz_core::engram;

use crate::{app::context::*, cli::Commands};

/// Holds the application state required to forge a new theme.
pub struct App;

impl App {
    /// Run application with given arguments.
    pub fn run(cli: crate::cli::Cli) -> anyhow::Result<()> {
        // 0. Zero friction setup: auto download the database and styles if missing

        if !cli.command.prevents_autosync() && !engram::db::path().exists() {
            crate::sync::sync_repo(false)?;
        }

        match cli.command {
            Commands::Forge {
                palette,
                output,
                engram,
                force,
            } => {
                let ctx = ForgeContext::build(palette, engram, output, force)?;
                ctx.execute()
            }
            Commands::List => ListContext::execute(),
            Commands::Clean => CleanContext::execute(),
            Commands::Sync { full } => SyncContext { full }.execute(),
            Commands::Doc { action } => DocContext { action }.execute(),
            #[cfg(feature = "dev-tools")]
            Commands::Dev { action } => DevContext { action }.execute(),
        }
    }
}
