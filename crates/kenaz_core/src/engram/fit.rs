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
    error::{KenazError, Result},
    palette::Colors,
    schema,
    visitor::PathBuffer,
};
use palette::{IntoColor, Oklab};
use rayon::prelude::*;

const FIT_TOKENS_STEPS: usize = 20;

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
    pub fn parse_hex(hex: &str) -> Result<Self> {
        let hex = hex.trim_start_matches('#');

        // Expand shorthand (#RGB -> #RRGGBB)
        let expanded = match hex.len() {
            3 | 4 => hex
                .chars()
                .flat_map(|c| std::iter::repeat_n(c, 2))
                .collect::<String>(),
            6 | 8 => hex.to_string(),
            _ => return Err(KenazError::InvalidHexColor(hex.to_string())),
        };

        let parse_byte = |s: &str| -> Result<u8> {
            u8::from_str_radix(s, 16).map_err(|_| KenazError::InvalidHexColor(hex.to_string()))
        };

        let r = parse_byte(&expanded[0..2])? as f32 / 255.0;
        let g = parse_byte(&expanded[2..4])? as f32 / 255.0;
        let b = parse_byte(&expanded[4..6])? as f32 / 255.0;
        let a = if expanded.len() == 8 {
            parse_byte(&expanded[6..8])? as f32 / 255.0
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
pub fn extract_anchors(theme: &schema::ThemeContent) -> Result<Colors> {
    let style = serde_json::to_value(&theme.style)?;

    let get = |path: &str| -> Result<Oklab> {
        let raw_hex = style
            .get(path)
            .and_then(|v| v.as_str())
            .ok_or_else(|| KenazError::MissingAnchor(path.to_string()))?;

        let extracted = ExtractedColor::parse_hex(raw_hex)?;
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
            let mut path_buffer = String::with_capacity(64);
            extract_colors(&style_value, &mut path_buffer, &mut colors);
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
    current_path: &mut String,
    out: &mut Vec<(String, ExtractedColor)>,
) {
    match value {
        serde_json::Value::Object(map) => {
            for (k, v) in map {
                current_path.with_dynamic_segment(
                    |buf| {
                        for c in k.chars() {
                            buf.push(if c == '.' { '_' } else { c });
                        }
                    },
                    |path| extract_colors(v, path, out),
                );
            }
        }
        serde_json::Value::Array(arr) => {
            for (i, v) in arr.iter().enumerate() {
                current_path.with_index(i, |path| extract_colors(v, path, out));
            }
        }
        serde_json::Value::String(s) => {
            if let Ok(extracted) = ExtractedColor::parse_hex(s) {
                out.push((current_path.clone(), extracted));
            }
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_hex_standard() {
        let color = ExtractedColor::parse_hex("#1e1e2e").unwrap();
        assert_eq!(color.alpha, 1.0);
        assert!(color.oklab.l > 0.0 && color.oklab.l < 0.5);
    }

    #[test]
    fn test_parse_hex_with_alpha() {
        let color = ExtractedColor::parse_hex("#1e1e2eff").unwrap();
        assert_eq!(color.alpha, 1.0);

        let color_transparent = ExtractedColor::parse_hex("#1e1e2e80").unwrap();
        assert!((color_transparent.alpha - 0.5).abs() < 0.01); // 80 in hex = 128 in decimal = ~50% transparency
    }

    #[test]
    fn test_parse_hex_shorthand() {
        let color_long = ExtractedColor::parse_hex("#112233").unwrap();
        let color_short = ExtractedColor::parse_hex("#123").unwrap();
        assert_eq!(color_long.oklab.l, color_short.oklab.l);
    }

    #[test]
    fn test_parse_hex_invalid() {
        assert!(ExtractedColor::parse_hex("#12345").is_err());
        assert!(ExtractedColor::parse_hex("#zzz").is_err());
    }

    #[test]
    fn test_oklab_distance() {
        let c1 = ExtractedColor::parse_hex("#000").unwrap();
        let c2 = ExtractedColor::parse_hex("#fff").unwrap();
        let dist = oklab_distance(c1.oklab, c2.oklab);
        assert!(dist > 0.9 && dist < 1.1);
    }

    #[test]
    fn test_array_path_format_matches_between_extraction_and_application() {
        use crate::engram::Engram;
        use crate::visitor::ColorMutable;

        // 1. simulate a theme with an array field
        let json = serde_json::json!({
            "syntax": ["#1e1e2e", "#ff0000", "#00ff00"]
        });

        let mut extracted = Vec::new();
        let mut path_buffer = String::with_capacity(64);
        extract_colors(&json, &mut path_buffer, &mut extracted);
        assert_eq!(
            extracted.len(),
            3,
            "The 3 colors within the array must be extracted"
        );

        // 3. Construct engram from extracted paths (like the builder would)
        let anchors = test_anchors();
        let engram: Engram = extracted
            .iter()
            .map(|(path, color)| (path.clone(), fit_token(*color, &anchors)))
            .collect();

        // 4. App side: simulate corresponding rust field
        // with sentinel value that wouldn't survive if lookup succeeds
        let mut palette_field: Vec<String> = vec!["#000000".into(); 3];
        let mut current_path = String::from("syntax");
        palette_field.apply_colors(&mut current_path, &engram, &anchors);

        // 5. If both sides are using the same key format, each colors
        // has been found and replaced. If the format diverge, lookup fails quietly
        // and sentinel value remain unchanged.
        for (i, color) in palette_field.iter().enumerate() {
            assert_ne!(
                color, "#000000",
                "Element {i} unchanged: path generated by apply_colors do not match \
                the one generated by extract_colors (index format non synchronised)"
            );
        }

        // 6. Bonus: explicitly checks that current_path came back to its initial state
        // after the loop (push/truncate pattern should not leave traces)
        assert_eq!(current_path, "syntax");
    }

    fn test_anchors() -> Colors {
        Colors {
            bg: ExtractedColor::parse_hex("#1e1e2e").unwrap().oklab,
            fg: ExtractedColor::parse_hex("#cdd6f4").unwrap().oklab,
            accent: ExtractedColor::parse_hex("#89b4fa").unwrap().oklab,
            success: ExtractedColor::parse_hex("#a6e3a1").unwrap().oklab,
            warning: ExtractedColor::parse_hex("#f9e2af").unwrap().oklab,
            error: ExtractedColor::parse_hex("#f38ba8").unwrap().oklab,
        }
    }
}
