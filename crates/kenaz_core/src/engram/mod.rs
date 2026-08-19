//! Provides the extraction, persistence, and application of style vectors (engrams).
//!
//! An "engram" is a memory trace. In Kenaz, it represents the mathematical DNA
//! of a theme: how each token's color relates to the 6 semantic anchor colors
//! (using weights and lightness deltas in the Oklab color space).
//!
//! This module coordinates:
//! - [`fit`](crate::engram::fit): The math to extract vectors from a source theme or apply them to a palette.
//! - [`vector`](crate::engram::vector): The `EngramVector` data structure.
//! - [`db`](crate::engram::db): SQLite persistence and retrieval.
//! - [`variant`](crate::engram::variant): The `EngramVariant` (Dark/Light) enum.
//! - `devtools`: The asynchronous GitHub scraper to build the database (feature-gated).

pub mod db;
pub mod fit;
pub mod variant;
pub mod vector;

/// A convenient prelude for users of the `kenaz_core` API.
///
/// This module re-exports the most commonly used types and functions,
/// allowing users to simply `use kenaz_core::engram::prelude::*`.
pub mod prelude {
    pub use super::Engram;
    pub use super::db::list_engrams;
    pub use super::fit::*;
    pub use super::variant::EngramVariant;
    pub use super::vector::{EngramVector, OpType};
}

/// Developer tools for scraping and building the engram database.
///
/// This module is only compiled when the `dev-tools` feature is enabled,
/// keeping the release binary lightweight and free of network dependencies.
#[cfg(feature = "dev-tools")]
pub mod devtools;

/// An in-memory representation of a theme's style mapping.
///
/// It maps a flat, dot-separated token path (e.g., `editor_background`)
/// to its corresponding `EngramVector`.
pub type Engram = std::collections::HashMap<String, vector::EngramVector>;
