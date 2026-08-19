//! Defines the `Colors` struct and its custom TOML deserialization.
//!
//! This module bridges the gap between human-readable hex strings (e.g., "#1e1e2e")
//! and the mathematically optimal `Oklab` color space used by the engine?
//! By deserializing directly to `Oklab`, we avoid runtime parsing overhead during
//! color calculations.

use palette::{IntoColor, Oklab, Srgb};
use serde::{Deserialize, Deserializer};

/// Custom Serde deserialized to convert a hex string into an `Oklab` color.
///
/// This allows the `Colors` struct to accept standard hex strings (like `#1e1e2e`)
/// directly from the TOML file and immediately convert them to the `Oklab` color space.
fn de<'de, D>(deserializer: D) -> Result<Oklab, D::Error>
where
    D: Deserializer<'de>,
{
    let hex = String::deserialize(deserializer)?;
    // The `palette` crate's parser natively handles standard RGB hex formats.
    let srgb: Srgb<u8> = hex.parse().map_err(serde::de::Error::custom)?;
    Ok(srgb.into_format::<f32>().into_color())
}

/// The 6 semantic anchor colors used as a base for the style transfer.
///
/// These colors represent the identity of the user's palette. Kenaz will mix and
/// shift these colors based on the extracted engrams to generate the final theme.
#[derive(Debug, Clone, Copy, Deserialize)]
pub struct Colors {
    /// The main background color.
    #[serde(deserialize_with = "de")]
    pub bg: Oklab,
    /// The main foreground (text) color.
    #[serde(deserialize_with = "de")]
    pub fg: Oklab,
    /// The main accent color (e.g., for highlights, links, etc.).
    #[serde(deserialize_with = "de")]
    pub accent: Oklab,
    /// The success color (e.g., for git additions, valid syntax).
    #[serde(deserialize_with = "de")]
    pub success: Oklab,
    /// The warning color (e.g., for cautionary syntax or UI elements).
    #[serde(deserialize_with = "de")]
    pub warning: Oklab,
    /// The erro color (e.g., for git deletions, invalid syntax)
    #[serde(deserialize_with = "de")]
    pub error: Oklab,
}

impl Colors {
    /// Returns the 6 colors as a flat array for easy iteration and mathematical operations.
    ///
    /// This is heavily used by the `fit_token` and `apply` functions to find the
    /// best matching anchor or to calculate weighted mixes.
    pub fn as_array(&self) -> [Oklab; 6] {
        [
            self.bg,
            self.fg,
            self.accent,
            self.success,
            self.warning,
            self.error,
        ]
    }
}
