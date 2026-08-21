//! Developer tools for building and exporting the engram database.
//!
//! This module is only compiled with the `dev-tools` feature. It provides the
//! `EngramBuilder` which scrapes the Zed extension ecosystem (GitHub), downloads
//! theme JSON files, extracts their mathematical DNA (engrams), and populates
//! the local SQLite database.
//!
//! It also provides tools to export lightweight "Curated Packs" or "Full Packs"
//! as `.tar.gz` archives for GitHub releases.
//!
//! Because it makes heavy use of network requests, it uses multithreading with
//! `std::thread` and an `mpsc::channel` to limit concurrency and avoid hitting
//! GitHub's rate limits.

mod api_response;
mod consts;
mod engram_builder;
mod export;
mod fetcher;
mod guard;

pub mod prelude {
    pub use super::engram_builder::EngramBuilder;
    pub use super::export::export_pack;
}
