//! Defines the `OpType` enum, representing the type of lightness operation
//! applied during the style transfer, and its SQLite integer mapping.

const CLASSIFY_THRESHOLD: f32 = 0.02;

/// Classifies the lightness operation required to reproduce a token's color.
///
/// This is determined by the `delta_l` (lightness difference) during the `fit_token`
/// phase. It is stored in the database as a small integer (TINYINT) for compactness.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OpType {
    /// The token is a direct match to an anchor or mix (no lightness shift)
    Direct = 0,
    /// The token is lighter than its base mix (`delta_l` > 0)
    Lighten = 1,
    /// The token is darker than its base mix (`delta_l` < 0)
    Darken = 2,
}

impl OpType {
    /// Classify a lightness delta (`delta_l`) into an `OpType`.
    ///
    /// Uses a small threshold (0.02) to determine if the difference is perceptible.
    pub fn classify_lightness_delta(delta_l: f32) -> OpType {
        if delta_l.abs() < CLASSIFY_THRESHOLD {
            return OpType::Direct;
        }
        if delta_l > 0.0 {
            OpType::Lighten
        } else {
            OpType::Darken
        }
    }
}

/// Safely converts a database integer back into an `OpType`
///
/// This is used when loading engrams from SQLite. If the database contains
/// an unknown integer (e.g., from a future version of Kenaz that added new types),
/// it safely defaults to `Direct` to prevent panics.
impl From<u8> for OpType {
    fn from(value: u8) -> Self {
        match value {
            0 => OpType::Direct,
            1 => OpType::Lighten,
            2 => OpType::Darken,
            _ => OpType::Direct,
        }
    }
}
