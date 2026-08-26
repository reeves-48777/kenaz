# Changelog

All notable changes to this project will be documented in this file.

## [1.1.0] - 2026-08-26
### Performance
- Zero-Allocation Extraction: Updated extract_colors to use the PathBuffer trait (with_dynamic_segment, with_index). This unifies the path-building logic with the application phase and eliminates all remaining format! allocations during the engram extraction process.

## [1.0.5] - 2026-08-26
### Added
- Typed Errors: Introduced KenazError using thiserror for robust, typed error handling across kenaz_core.
- Zero-Allocation Traversal: Implemented a PathBuffer trait with closures (with_segment, with_index) to eliminate String allocations during recursive schema traversal.
- Database Utilities: Added ConnectionExt trait for safe SQLite closure and EngramRecord::delete_by_theme_name for future edit commands.
- Benchmarks: Added criterion benchmarks for fit_token and apply_colors to measure engine performance.

### Changed
- Concurrency: Replaced the manual mpsc semaphore and PermitGuard with a standard rayon::ThreadPool for fetching themes.
- Database API: Replaced EngramRecordBuilder with a simpler, infallible EngramRecord::new() constructor.
- Math Performance: Reduced FIT_TOKENS_STEPS from 100 to 20 for faster engram extraction with imperceptible visual difference.
- JSON Parsing: Switched to serde_json::from_slice for slightly faster, zero-copy reading of base theme files.

### Removed
- Removed anyhow from kenaz_core in favor of the custom KenazError enum.
- Removed tokio and reqwest completely from the workspace, resulting in significantly faster compile times.

## [1.0.4] - 2026-08-20
### Fixed
- Curated Export Bug: Fixed an issue where a missing ! in the is_file() check caused the export process to silently skip all JSON files, resulting in an empty curated database.
- Case-Insensitive Folders: Curated repository folder names are now matched case-insensitively to ensure all repos (e.g., "Catppuccin" vs "catppuccin") are properly included in the export.

### Changed
- Global Verbosity Flag: The --verbose (-v) CLI flag is now global = true, allowing users to place it anywhere in the command (e.g., kenaz dev export -vvv).

## [1.0.2] - 2026-08-20
### Fixed
- Auto-sync bug: Diagnostic and maintenance commands (kenaz clean, kenaz doc, kenaz sync) no longer trigger the automatic database download if the cache is missing.
- Doctor show-path: The kenaz doc show-path command now correctly checks if the cache files and directories actually exist before printing their paths, warning the user if they are missing.
- Dynamic Sync URL: The sync module now dynamically uses CARGO_PKG_VERSION to fetch the correct pack for the installed version, preventing future compatibility issues with hardcoded URLs.

### Changed
- CLI Refactor: Refactored the auto-sync prevention logic into a clean Commands::prevents_autosync() method within cli.rs.

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
