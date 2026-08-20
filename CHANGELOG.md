# Changelog

All notable changes to this project will be documented in this file.

## [1.0.0] - 2026-08-20

### Added
- **Style Transfer Engine**: Kenaz now performs Luminance-only style transfer in the Oklab color space, modifying only the L channel to preserve palette's identity.
- **SQLite Persistence**: Engrams (style vectors) are stored in a local SQLite databse for fast querying.
- **Zero-Friction Setup**: First run automatically downloads a lightweight "Curated Pack" containing the database and canvas JSON files.
- **Subcommands**: Restructured the CLI to use clear subcommands (forge, list, sync, clean, doc, dev).
- **Compile-Time Code Generation**: Added `build.rs` to auto-generate Zed schema structs via `typify` and inject a custom `ColorMutable` proc-macro.
- **Dev Tools**: Added an asynchronous Github scraper (`kenaz dev build`) to extract the mathematical DNA of 1300+ Zed themes.
- **Curated Export**: Dev tool to export a lightweight tarball pack of selected styles (`kenaz dev export`).
- **Cross-Mode Transfer**: Added `--force` flag to invert lightness deltas when applying dark engrams to light palettes.
- **Timestamped Backups**: Automatically creates a timestamped backup of the output file before overwriting.

### Changed
- **Renamed Project**: Migrated from ZTF (Zed Theme Fixer) to Kenaz.
- **Workspace Architecture**: Refactured the monolithic crate into a Cargo workspace (kenaz_bin, kenaz_core, kenaz_macros).
- **Color Math**: Moved from RGB/Oklch to Oklab to ensure perceptually uniform calculations.
- **Cache Management**: Moved the engram database and raw styles to the persistent OS cache directory.
- **Async Isolation**: Scoped the `tokio` async runtime exclusively behind the `dev-tools` feature, keeping the main binary lightweight and synchronous.

### Removed
- Legacy flags `--list-engrams`, `--build-engrams`, etc. gone. Replaced by explicit commands `kenaz forge`, `kenaz list`, `kenaz dev build`.
