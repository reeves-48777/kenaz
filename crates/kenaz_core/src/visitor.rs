//! Provides the `ColorMutable` trait and its implementations for standard types.
//!
//! This module is the engine of the recursive traversal. When the `ColorMutable`
//! derive macro generates code for a struct, it blindy class `apply_colors` on
//! its fields. The implementation here decide what to do based on the type:
//! - String/Cow are treated as colors and replaced if they match the engram vector.
//! - Collections (Option, Vec, HashMap, Result) propagate the call down the tree.
//! - Primitives (bool, f64, etc.) are ignored (no-ops), preserving metadata like `font_weight`.

use crate::engram::Engram;
use crate::palette::Colors;
use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt::Write;

pub trait PathBuffer {
    fn with_segment<R>(&mut self, segment: &str, f: impl FnOnce(&mut Self) -> R) -> R;
    fn with_dynamic_segment<R>(
        &mut self,
        write: impl FnOnce(&mut Self),
        f: impl FnOnce(&mut Self) -> R,
    ) -> R;
    fn with_index<R>(&mut self, index: usize, f: impl FnOnce(&mut Self) -> R) -> R;
}

impl PathBuffer for String {
    fn with_segment<R>(&mut self, segment: &str, f: impl FnOnce(&mut Self) -> R) -> R {
        let prev_len = self.len();
        if !self.is_empty() {
            self.push('_');
        }

        self.push_str(segment);

        let result = f(self);

        self.truncate(prev_len);
        result
    }

    fn with_dynamic_segment<R>(
        &mut self,
        write: impl FnOnce(&mut Self),
        f: impl FnOnce(&mut Self) -> R,
    ) -> R {
        let prev_len = self.len();

        if !self.is_empty() {
            self.push('_');
        }

        write(self);

        let result = f(self);

        self.truncate(prev_len);
        result
    }

    fn with_index<R>(&mut self, index: usize, f: impl FnOnce(&mut Self) -> R) -> R {
        let prev_len = self.len();
        let _ = write!(self, "[{index}]");

        let result = f(self);

        self.truncate(prev_len);
        result
    }
}

/// A trait for recursively applying engram vectors to the data structures.
///
/// This is automatically derived from Zed schema types via the `kenaz_macros::ColorMutable` macro.
pub trait ColorMutable {
    /// Recursively applies colors to this item, building the `current_path` as it descends?
    /// Uses a mutable buffer to achieve Zero-Allocation path building
    fn apply_colors(&mut self, current_path: &mut String, engram: &Engram, anchors: &Colors);
}

// --- Color Leaves ---
// These types represent the actual color strings in the JSON schema

impl ColorMutable for Cow<'static, str> {
    fn apply_colors(&mut self, current_path: &mut String, engram: &Engram, anchors: &Colors) {
        if let Some(vector) = engram.get(current_path) {
            let new_color = vector.apply(anchors);
            *self = Cow::Owned(new_color.to_hex());
        }
    }
}

impl ColorMutable for String {
    fn apply_colors(&mut self, current_path: &mut String, engram: &Engram, anchors: &Colors) {
        if let Some(vector) = engram.get(current_path) {
            let new_color = vector.apply(anchors);
            *self = new_color.to_hex();
        }
    }
}

// --- Branches / Propagators ---
// These types just pass the call down to their inner values, adjusting the path.
impl<T: ColorMutable> ColorMutable for Option<T> {
    fn apply_colors(&mut self, current_path: &mut String, engram: &Engram, anchors: &Colors) {
        if let Some(inner) = self {
            inner.apply_colors(current_path, engram, anchors);
        }
    }
}

impl<T: ColorMutable> ColorMutable for Vec<T> {
    fn apply_colors(&mut self, current_path: &mut String, engram: &Engram, anchors: &Colors) {
        for (i, item) in self.iter_mut().enumerate() {
            current_path.with_index(i, |path| {
                item.apply_colors(path, engram, anchors);
            });
        }
    }
}

impl<V: ColorMutable> ColorMutable for HashMap<String, V> {
    fn apply_colors(&mut self, current_path: &mut String, engram: &Engram, anchors: &Colors) {
        for (key, value) in self.iter_mut() {
            current_path.with_dynamic_segment(
                |buf| {
                    for c in key.chars() {
                        buf.push(if c == '.' { '_' } else { c });
                    }
                },
                |path| {
                    value.apply_colors(path, engram, anchors);
                },
            );
        }
    }
}

impl<T: ColorMutable, E: ColorMutable> ColorMutable for Result<T, E> {
    fn apply_colors(&mut self, current_path: &mut String, engram: &Engram, anchors: &Colors) {
        match self {
            Ok(t) => t.apply_colors(current_path, engram, anchors),
            Err(e) => e.apply_colors(current_path, engram, anchors),
        }
    }
}

// --- No-Ops (metadata) ---
// These types are not colors (e.g., `font_weight`, `font_style`). We implement
// the trait as a no-op so the recursive macro doesn't fail when encountering them.
impl ColorMutable for u32 {
    fn apply_colors(&mut self, _: &mut String, _: &Engram, _: &Colors) {}
}
impl ColorMutable for i32 {
    fn apply_colors(&mut self, _: &mut String, _: &Engram, _: &Colors) {}
}
impl ColorMutable for f32 {
    fn apply_colors(&mut self, _: &mut String, _: &Engram, _: &Colors) {}
}
impl ColorMutable for u64 {
    fn apply_colors(&mut self, _: &mut String, _: &Engram, _: &Colors) {}
}
impl ColorMutable for i64 {
    fn apply_colors(&mut self, _: &mut String, _: &Engram, _: &Colors) {}
}
impl ColorMutable for f64 {
    fn apply_colors(&mut self, _: &mut String, _: &Engram, _: &Colors) {}
}
impl ColorMutable for bool {
    fn apply_colors(&mut self, _: &mut String, _: &Engram, _: &Colors) {}
}
impl ColorMutable for () {
    fn apply_colors(&mut self, _: &mut String, _: &Engram, _: &Colors) {}
}
