//! Defines the Command-Line Interface (CLI) for Kenaz using `clap`.
//!
//! This module parses command-line arguments and handles conditional requirements
//! based on the `dev-tools` feature flag. It ensures that mutually exclusive
//! actions (like building the database vs. generating a theme) cannot be run together.

use clap::Parser;
use std::path::PathBuf;

/// Generate themes from a semantic color palette
#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Palette file to compose theme with
    #[cfg_attr(feature = "dev-tools", arg(required_unless_present_any = ["list_engrams", "build_engrams"]))]
    #[cfg_attr(
        not(feature = "dev-tools"),
        arg(required_unless_present = "list_engrams")
    )]
    pub palette: Option<PathBuf>,

    /// Theme engram (use --list-engrams to see available ones)
    #[arg(short, long, default_value = "one")]
    pub engram: String,

    /// Output path to write to
    #[cfg_attr(feature = "dev-tools", arg(required_unless_present_any = ["list_engrams", "build_engrams"]))]
    #[cfg_attr(
        not(feature = "dev-tools"),
        arg(required_unless_present = "list_engrams")
    )]
    pub output: Option<PathBuf>,

    /// List available engrams
    #[cfg_attr(feature = "dev-tools", arg(long, conflicts_with_all = ["palette", "output", "build_engrams"]))]
    #[cfg_attr(not(feature = "dev-tools"), arg(long, conflicts_with_all = ["palette", "output"]))]
    pub list_engrams: bool,

    /// Build engrams from real Zed themes (dev only)
    #[cfg(feature = "dev-tools")]
    #[arg(long, conflicts_with_all = ["palette", "output", "list_engrams"])]
    pub build_engrams: bool,

    /// Skip re-fetching themes from repos, reuse the temp folder (requires --build-engrams)
    #[cfg(feature = "dev-tools")]
    #[arg(long, requires = "build_engrams")]
    pub skip_fetch: bool,

    /// Verbosity
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub verbose: u8,
}
