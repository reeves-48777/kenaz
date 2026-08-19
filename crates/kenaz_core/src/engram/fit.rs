//! The mathematical core for extracting and applying theme engrams.
//!
//! This module handles the conversion between hex strings and the perceptually
//! uniform `Oklab` color space. It calculates the optimal "fit" (how a specific
//! theme token relates to the 6 anchor colors) by finding the best single anchor
//! or the best linear mix between two anchors, minimizing the perceptual distance.
//!
//! It also provides the recursive JSON traversal logic to flatten a nested theme
//! file into a list of token paths and their corresponding colors.

use crate::{
    engram::vector::{EngramVector, OpType},
    palette::Colors,
    schema,
};
use anyhow::anyhow;
use palette::{IntoColor, Oklab};
use rayon::prelude::*;

const FIT_TOKENS_STEPS: usize = 100;

/// Represents a parsed color, holding its `Oklab` value and original alpha channel.
///
/// `Oklab` is used for calculations as it is perceptually uniform, meaning
/// mathematical distances match human color perception.
#[derive(Clone, Copy)]
pub struct ExtractedColor {
    pub oklab: Oklab,
    pub alpha: f32,
}

impl ExtractedColor {
    /// Converts the `Oklab` color back into a Zed-compatible hex string.
    ///
    /// Automatically handles the alpha channel: returns `#RRGGBB` if opaque,
    /// or `#RRGGBBAA` if the color is semi-transparent.
    pub fn to_hex(&self) -> String {
        let srgb: palette::Srgb = self.oklab.into_color();
        let r = (srgb.red.clamp(0.0, 1.0) * 255.0).round() as u8;
        let g = (srgb.green.clamp(0.0, 1.0) * 255.0).round() as u8;
        let b = (srgb.blue.clamp(0.0, 1.0) * 255.0).round() as u8;

        if self.alpha < 1.0 {
            let a = (self.alpha.clamp(0.0, 1.0) * 255.0).round() as u8;
            format!("#{r:02x}{g:02x}{b:02x}{a:02x}")
        } else {
            format!("#{r:02x}{g:02x}{b:02x}")
        }
    }

    /// Parses a hex color string into an `ExtractedColor`.
    ///
    /// Supports CSS-style shorthand notations (`#RGB`, `#RGBA`) as well as standard
    /// 6-digit and 8-digit (with alpha) formats.
    fn try_parse_hex(hex: &str) -> anyhow::Result<Self> {
        let hex = hex.trim_start_matches('#');

        // Expand shorthand (#RGB -> #RRGGBB)
        let expanded = match hex.len() {
            3 | 4 => hex
                .chars()
                .flat_map(|c| std::iter::repeat(c).take(2))
                .collect::<String>(),
            6 | 8 => hex.to_string(),
            _ => return Err(anyhow!("Invalid hex length: {hex}")),
        };

        let r = u8::from_str_radix(&expanded[0..2], 16)? as f32 / 255.0;
        let g = u8::from_str_radix(&expanded[2..4], 16)? as f32 / 255.0;
        let b = u8::from_str_radix(&expanded[4..6], 16)? as f32 / 255.0;
        let a = if expanded.len() == 8 {
            u8::from_str_radix(&expanded[6..8], 16)? as f32 / 255.0
        } else {
            1.0
        };

        let srgb = palette::Srgb::new(r, g, b);
        let oklab: Oklab = srgb.into_color();

        Ok(ExtractedColor { oklab, alpha: a })
    }
}

/// Calculates the best engram vector to reproduce a target color using the given anchors.
///
/// This function tries all single anchors and all pairwise linear mixes (in `FIT_TOKENS_STEPS` steps)
/// to find the combination that minimizes the perceptual distance in `Oklab` space.
/// It then calculates the `delta_l` (lightness difference) between this optimal mix
/// and the target color.
pub fn fit_token(target: ExtractedColor, anchors: &Colors) -> EngramVector {
    let anchor_arr = anchors.as_array();
    let mut best_dist = f32::MAX;
    let mut best_weights = [0.0f32; 6];

    // 1. Try matching against a single anchor
    for i in 0..6 {
        let dist = oklab_distance(target.oklab, anchor_arr[i]);
        if dist < best_dist {
            best_dist = dist;
            best_weights = [0.0; 6];
            best_weights[i] = 1.0;
        }
    }

    // 2. Try matching against a linear mix of two anchors
    for i in 0..6 {
        for j in (i + 1)..6 {
            for step in 0..=FIT_TOKENS_STEPS {
                let t = step as f32 / FIT_TOKENS_STEPS as f32;
                let mixed = mix_oklab(anchor_arr[i], anchor_arr[j], t);
                let dist = oklab_distance(target.oklab, mixed);
                if dist < best_dist {
                    best_dist = dist;
                    best_weights = [0.0; 6];
                    best_weights[i] = 1.0 - t;
                    best_weights[j] = t;
                }
            }
        }
    }

    // 3. Calculate the final base color and the lightness delta
    let base = weighted_mix_oklab(&anchor_arr, &best_weights);
    let delta_l = target.oklab.l - base.l;

    EngramVector {
        op_type: OpType::classify_lightness_delta(delta_l),
        weights: best_weights,
        delta_l,
        alpha: target.alpha,
    }
}

/// Extracts the 6 semantic anchor colors (bg, fg, etc.) from a Zed theme.
///
/// These anchors define the palette of the source theme, which `fit_token`
/// will use as a reference to calculate all other token vectors.
pub fn extract_anchors(theme: &schema::ThemeContent) -> anyhow::Result<Colors> {
    let style = serde_json::to_value(&theme.style)?;

    let get = |path: &str| -> anyhow::Result<Oklab> {
        let raw_hex = style.get(path).and_then(|v| v.as_str()).ok_or_else(|| {
            anyhow::anyhow!("missing anchor token '{path}' in theme {}", theme.name)
        })?;

        let extracted = ExtractedColor::try_parse_hex(raw_hex)
            .map_err(|_| anyhow::anyhow!("invalid hex color for '{path}': {raw_hex}"))?;
        Ok(extracted.oklab)
    };

    Ok(Colors {
        bg: get("background")?,
        fg: get("text")?,
        accent: get("text.accent")?,
        success: get("success")?,
        warning: get("warning")?,
        error: get("error")?,
    })
}

/// Recursively traverses a theme's style object, extracting all colors and their flat paths.
///
/// This parallelizes the `fit_token` calculation using `rayon` for massive performance
/// gains when processing themes with hundreds of syntax tokens.
pub fn fit_theme(
    themes: &[&schema::ThemeContent],
    anchors: &Colors,
) -> Vec<(String, EngramVector)> {
    themes
        .iter()
        .flat_map(|t| {
            let style_value = serde_json::to_value(&t.style).unwrap_or(serde_json::Value::Null);
            let mut colors = Vec::new();
            extract_colors(&style_value, String::new(), &mut colors);
            colors
        })
        .collect::<Vec<_>>()
        .par_iter()
        .map(|(name, color)| (name.clone(), fit_token(*color, anchors)))
        .collect()
}

/// Calculates a weighted average of multiple anchor colors in `Oklab` space.
pub(crate) fn weighted_mix_oklab(anchors: &[Oklab; 6], weights: &[f32; 6]) -> Oklab {
    let mut l = 0.0;
    let mut a = 0.0;
    let mut b = 0.0;

    for i in 0..6 {
        if weights[i] == 0.0 {
            continue;
        }
        let lab: Oklab = anchors[i].into_color();
        l += lab.l * weights[i];
        a += lab.a * weights[i];
        b += lab.b * weights[i];
    }

    let mixed_lab = Oklab::new(l, a, b);
    mixed_lab.into_color()
}

/// Computes the perceptual Euclidean distance between two `Oklab` colors.
fn oklab_distance(a: Oklab, b: Oklab) -> f32 {
    let dl = a.l - b.l;
    let da = a.a - b.a;
    let db = a.b - b.b;
    (dl * dl + da * da + db * db).sqrt()
}

/// Performs linear interpolation between two `Oklab` colors.
fn mix_oklab(a: Oklab, b: Oklab, t: f32) -> Oklab {
    let a_lab: Oklab = a.into_color();
    let b_lab: Oklab = b.into_color();
    let mixed = Oklab::new(
        a_lab.l + (b_lab.l - a_lab.l) * t,
        a_lab.a + (b_lab.a - a_lab.a) * t,
        a_lab.b + (b_lab.b - a_lab.b) * t,
    );
    mixed.into_color()
}

/// Recursively extracts colors from a JSON value, building flat, dot-separated paths.
///
/// Keys containing dots (e.g., "variable.member") have their dots replaced with
/// underscores to prevent path conflicts with nested structs, ensuring 1:1 mapping
/// with the Rust schema.
fn extract_colors(
    value: &serde_json::Value,
    prefix: String,
    out: &mut Vec<(String, ExtractedColor)>,
) {
    match value {
        serde_json::Value::Object(map) => {
            for (k, v) in map {
                let clean_k = k.replace('.', "_");
                let path = if prefix.is_empty() {
                    clean_k
                } else {
                    format!("{prefix}_{clean_k}")
                };

                extract_colors(v, path, out);
            }
        }
        serde_json::Value::Array(arr) => {
            for (i, v) in arr.iter().enumerate() {
                extract_colors(v, format!("{prefix}[{i}]"), out);
            }
        }
        serde_json::Value::String(s) => {
            if let Ok(extracted) = ExtractedColor::try_parse_hex(s) {
                out.push((prefix, extracted));
            }
        }
        _ => {}
    }
}
