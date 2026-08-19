# Kenaz Architecture

This document outlines the technical decisions and architectural patterns used in Kenaz.

## 1. Compile-Time Code Generation (`typify` + `build.rs`)
Instead of manually maintaining Rust structs to match the Zed `theme.json` schema, Kenaz uses a `build.rs` script. 
- It downloads the official Zed schema at compile time.
- It uses the `typify` crate to generate strongly-typed Rust structs.
- It uses the `syn` crate to inject a custom `#[derive(ColorMutable)]` macro into every generated struct and enum.
- **Why?** This guarantees zero-friction maintenance. If Zed adds 50 new tokens tomorrow, Kenaz automatically supports them in the next `cargo build` without a single line of code changed.

## 2. The Visitor Pattern via Proc-Macros (`kenaz_macros`)
Because the generated schema has over 6000 lines of nested structs, dynamically traversing them to replace colors is impossible without runtime reflection (which Rust doesn't have).
- The `ColorMutable` derive macro generates a recursive visitor at compile time.
- It blindly calls `apply_colors` on every field.
- Standard library types (`String`, `Option`, `Vec`, `HashMap`) implement the trait to either replace colors or propagate the call. Primitives (`bool`, `i32`) implement it as no-ops, preserving metadata like `font_weight`.

## 3. Luminance-Only Style Transfer (Oklab)
Early iterations used `Oklch` (Hue, Chroma, Lightness) to transfer the complete identity of a source theme. This caused issues: applying a warm theme (Gruvbox) to a cold palette (Grim) resulted in muddy, off-hue colors.
- **The Pivot:** Kenaz now operates exclusively in the `Oklab` color space, modifying **only the `L` (Lightness) channel**.
- The `weights` array defines how to mix the user's 6 anchors. The `delta_l` defines the lightness shift.
- **Why?** This guarantees that the generated theme respects 100% of the user's palette identity (Hue/Chroma), while perfectly adopting the structural depth (shadows, popups, syntax contrasts) of the source theme.

## 4. Data Persistence & Performance (SQLite)
- Extracting the "DNA" of 1300+ themes requires parsing thousands of JSON files and running expensive Oklab distance calculations.
- Kenaz uses `rayon` for parallel extraction and stores the resulting 244,000+ vectors in a local SQLite database (`engrams.db`).
- Database writes are wrapped in unchecked transactions for massive performance gains.
- The runtime (theme generation) only performs a lightweight `SELECT` query and in-memory math, taking milliseconds to generate a theme.
