//! # Kenaz Core
//!
//! The core library for the Kenaz theme engine.
//!
//! Kenaz Performs "Style Transfer" for editor themes. It extracts the mathematical
//! DNA (luminosity and opacity in the Oklab color space) of existing themes into "engrams",
//! and applies them to custom user palettes.
//!
//! ## Key Modules
//!
//! - [`palette`]: Defines the user's color palette and parses it from TOML.
//! - [`engram`]: Handles the extract, persistence (SQLite in dev mode) and mathematical application of style vectors.
//! - [`visitor`]: The `ColorMutable` trait used to recursively traverse data structures.
//! - [`schema`]: The official Zed theme schema, auto-generated at build time by `typify`
//! - [`util`]: Small utilitary module

/// Handles the extraction, persistence, and application of style vectors (engrams).
pub mod engram;

/// Defines the user-facing palette structure and TOML deserialization.
pub mod palette;

/// Utility functions for database access, path manipulation and JSON traversal.
pub mod util;

/// Provides the `ColorMutable` trait and implementation for standard library types.
pub mod visitor;

pub mod error;

/// The auto-generated Zed theme schema
///
/// This module is populated at compile time by `build.rs` using `typify`.
/// It contains strongly-typed structs and enums representing the `theme.json` specification.
#[allow(clippy::all, clippy::pedantic, dead_code)]
pub mod schema {
    include!(concat!(env!("OUT_DIR"), "/schema.rs"));
}

/// A re-export of the `Engram` type, representing a loaded style mapping( token path to vector).
pub use engram::Engram;
pub use error::{KenazError, Result};
