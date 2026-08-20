//! Defines the Command-Line Interface (CLI) for Kenaz using `clap`.
//!
//! This module parses command-line arguments and handles conditional requirements
//! based on the `dev-tools` feature flag. It ensures that mutually exclusive
//! actions (like building the database vs. generating a theme) cannot be run together.

use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// Generate themes from a semantic color palette
#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Verbosity
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub verbose: u8,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Generates a theme from a palette and an engram
    Forge {
        /// Palette file to compose theme with
        palette: PathBuf,

        /// Output path to write to
        output: PathBuf,

        /// Theme engram (use `kenaz list` to see available ones)
        #[arg(short, long, default_value = "one dark")]
        engram: String,

        /// Force engram usage whether it is relevant for the palette we use
        #[arg(short, long, action = clap::ArgAction::SetTrue)]
        force: bool,
    },

    /// List all available engrams in the database
    List,

    /// Clean cache
    Clean,

    Sync {
        /// Full sync channel
        #[arg(long, action = clap::ArgAction::SetTrue)]
        full: bool,
    },

    /// Doctor tools, for now it just have the show path command
    Doc {
        #[command(subcommand)]
        action: DocActions,
    },

    /// Developer tools for building and managing the engram database (requires `dev-tools` feature)
    #[cfg(feature = "dev-tools")]
    Dev {
        #[command(subcommand)]
        action: DevActions,
    },
}

#[derive(Debug, Subcommand)]
#[cfg(feature = "dev-tools")]
pub enum DevActions {
    /// Scrape GitHub and build full engram database
    Build {
        /// Skip re-fetching themes from repos, reuse the local cache
        #[arg(long, action = clap::ArgAction::SetTrue)]
        skip_fetch: bool,
    },

    /// Export `.tar.gz` pack for GitHub releases
    Export {
        #[arg(long, action = clap::ArgAction::SetTrue)]
        full: bool,
    },
}

#[derive(Debug, Subcommand)]
pub enum DocActions {
    /// Show paths used by kenaz
    ShowPath,
}
