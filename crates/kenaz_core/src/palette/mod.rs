//! Defines the user-facing color palette structures and TOML deserialization.
//!
//! This module represents the "Canvas" for the Kenaz style transfer. Instead of
//! hardcoding colors, users define a set of semantic anchor colors (`bg`, `fg`, etc.)
//! which Kenaz then distorts using the extracted engrams to produce a final theme.

mod color;
pub use color::Colors;

use crate::engram::variant::EngramVariant;
use serde::Deserialize;

/// Metadata for the theme family.
#[derive(Debug, Deserialize)]
pub struct Meta {
    /// The name of the theme family (e.g., "MyTheme").
    pub name: String,
}

/// A specific variant of the palette (e.g., Dark or Light)
///
/// A single palette file can contain multiple variants, allowing Kenaz to
/// generate a complete theme family in one run
#[derive(Debug, Deserialize)]
pub struct Variant {
    /// The display name for this specific variant (e.g., "MyTheme Dark")
    pub name: String,
    /// The mode of this variant, determining which engrams to apply.
    pub mode: EngramVariant,
    /// The 6 semantic anchor colors for this variant.
    pub colors: Colors,
    // NOTE: should add a style attribute
}

/// The root structure representing a `palette.toml` file.
#[derive(Debug, Deserialize)]
pub struct Palette {
    /// Global metadata for the theme
    pub meta: Meta,
    /// A list of light/dark variants to generate
    // NOTE: could be possible to generate multiple dark and light variants based on differents input styles
    // Variant struct should have a style attribute
    pub variants: Vec<Variant>,
}
