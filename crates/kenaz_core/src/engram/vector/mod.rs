//! Defines the `EngramVector` struct and its mathematical operations.
//!
//! An `EngramVector` represents the "DNA" of a single theme token. It defines
//! how to reconstruct a specific color by mixing the 6 semantic anchors and
//! shifting the lightness. This module provides the logic to apply this vector
//! to a user's palette, or to invert it for cross-mode (Dark<->Light) transfers.

mod types;
// Re-export of the OpType enum for convenience
pub use types::OpType;

use super::fit::weighted_mix_oklab;
use crate::{engram::fit::ExtractedColor, palette::Colors};
use palette::Oklab;

/// Represents the mathematical DNA of a single theme token.
///
/// It contains the optimal mix of the 6 anchor colors (`weights`), the lightness
/// difference (`delta_l`), and the original transparency (`alpha`) required to
/// reproduce the target color.
#[derive(Debug, Clone, Copy)]
pub struct EngramVector {
    /// The classification of the lightness operation (Direct, Lighten, Darken).
    pub op_type: types::OpType,
    /// The 6 weights defining how to mix the anchor colors (sums to 1.0).
    pub weights: [f32; 6],
    /// The lightness difference in `Oklab` space to apply after the mix.
    pub delta_l: f32,
    /// The original alpha (transparency) channel of the source token.
    pub alpha: f32,
}

impl EngramVector {
    /// Inverts the vector for cross-mode style transfer.
    ///
    /// This is used when applying an engram extracted from a Dark theme to a Light
    /// palette (or vice versa). It swaps `Lighten`/`Darken` operations and negates
    /// the `delta_l`, ensuring the contrasts remains logicial in the opposite mode.
    pub fn invert(&self) -> Self {
        let inverted_op = match self.op_type {
            OpType::Darken => OpType::Lighten,
            OpType::Lighten => OpType::Darken,
            _ => self.op_type,
        };

        Self {
            op_type: inverted_op,
            weights: self.weights,
            delta_l: -self.delta_l,
            alpha: self.alpha,
        }
    }

    /// Applies the engram vector to the user's palette anchors to produce the final color.
    ///
    /// This performs the weighted mix of the anchors, then applies the `delta_l`
    /// shift. Crucially, it only modifies the `L` (lightness) channel, preserving
    /// the `A` and `B` (chroma/hue) of the mixed palette. This guarentees that the
    /// generated colors stays true to the user's palette identity.
    pub fn apply(&self, anchors: &Colors) -> ExtractedColor {
        let anchor_arr = anchors.as_array();

        // Calculate the base color by mixing the user's anchors
        let base = weighted_mix_oklab(&anchor_arr, &self.weights);

        // Apply the lightness delta, stricly preserving the `a` and `b` channels
        let final_oklab = Oklab::new((base.l + self.delta_l).clamp(0.0, 1.0), base.a, base.b);

        ExtractedColor {
            oklab: final_oklab,
            alpha: self.alpha,
        }
    }
}
