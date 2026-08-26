//! Defines the core error types for the Kenaz engine.
//!
//! Using `thiserror` allows us to provide strong, typed errors instead of
//! opaque `anyhow` strings, making the library logic easier to debug and handle.

use thiserror::Error;

/// The result type used throughout `kenaz_core`.
pub type Result<T> = std::result::Result<T, KenazError>;

/// The error type for the Kenaz core engine
#[derive(Debug, Error)]
pub enum KenazError {
    #[error("Theme style not found in database for '{theme_name}' ({variant:?})")]
    StyleNotFound {
        theme_name: String,
        variant: crate::engram::variant::EngramVariant,
    },

    #[error("Missing required anchor token '{0}' in theme")]
    MissingAnchor(String),

    #[error("Invalid hex color provided: {0}")]
    InvalidHexColor(String),

    #[error("Unexpected repository URL format: {0}")]
    InvalidRepoURLFormat(String),

    #[error("Base theme style not found for '{0}'")]
    BaseStyleNotFound(String),

    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Thread pool initialization error: {0}")]
    ThreadPool(#[from] rayon::ThreadPoolBuildError),

    #[error("JSON serialization/deserialization error: {0}")]
    Json(#[from] serde_json::Error),

    #[cfg(feature = "dev-tools")]
    #[error("Network error: {0}")]
    Network(#[from] ureq::Error),
}
