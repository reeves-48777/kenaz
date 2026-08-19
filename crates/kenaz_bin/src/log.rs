//! Logging configuraition utilities/
//!
//! This module translates the CLI verbosity flags (e.g., `-v`, `-vv) into
//! `tracing_subscriber` log level filters, allowing users to control the
//! amount of diagnostics output generated during theme generation.

use tracing_subscriber::filter::LevelFilter;

/// Maps a verbosity count (from clap's `ArgAction::Count`) to a log `LevelFilter`.
///
/// -  `0` (default):   Only show warning and errors (`WARN`).
/// -  `1` (`-v`):      Show informational messages (`INFO`).
/// -  `2` (`-vv`):     Show debug messages (`DEBUG`).
/// -  `3+` (`-vvv`):   Show everything, including highly detailed traces (`TRACE`).
pub fn verbosity_to_level(count: u8) -> LevelFilter {
    match count {
        0 => LevelFilter::WARN,
        1 => LevelFilter::INFO,
        2 => LevelFilter::DEBUG,
        _ => LevelFilter::TRACE,
    }
}
