//! Provides SQLite persistence and retrieval for engrams
//!
//! This module bridges the gap between the mathematical `EngramVector` and the
//! disk. It handles database initialization, querying for CLI display, and
//! loading specific theme styles into memory for the style transfer engine.

mod record;
mod variant;

use crate::{KenazError, engram::prelude::*, error::Result};

pub mod prelude {
    pub use super::ConnectionExt;
    pub use super::record::EngramRecord;
}

/// Extension trait for [`rusqlite::Connection`] to simplify error handling.
pub trait ConnectionExt {
    /// Closes the connection, converting rusqlite's `(Connection, Error)` tuple
    /// error into a plain [`KenazError`] for use with the `?` operator.
    fn close_safely(self) -> Result<()>;
}

impl ConnectionExt for rusqlite::Connection {
    fn close_safely(self) -> Result<()> {
        self.close().map_err(|(_, e)| KenazError::Database(e))
    }
}

/// Queries the database and prints a formatted list of all available theme engrams.
///
/// This is used by the CLI `--list-engrams` command. It groups the engrams by
/// theme name and counts the number of tokens extracted per theme.
pub fn list_engrams(conn: &rusqlite::Connection) -> Result<()> {
    let mut stmt = conn.prepare("SELECT theme_name, COUNT(*) AS tokens FROM engrams GROUP BY theme_name ORDER BY theme_name")?;
    let engrams = stmt.query_map([], |row| {
        let name: String = row.get(0)?;
        let tokens: i32 = row.get(1)?;
        Ok((name, tokens))
    })?;

    println!("🎨 Available engrams:");
    println!("{:-<42}", "-");

    let mut count = 0;
    for engram in engrams {
        let (name, tokens) = engram?;
        println!("  - {name:<25} ({tokens} tokens)");
        count += 1;
    }

    if count == 0 {
        println!("   (Database is empty. Run with --build-engrams first)");
    } else {
        println!("\nTotal: {} styles available.", count);
    }

    Ok(())
}

/// Retrieves all engram vectors for a specific theme and variant.
///
/// Queries the SQLite database and reconstructs an in-memory `Engram` (`HashMap`)
/// mapping token paths to their corresponding `EngramVector`.
///
/// # Errors
/// Returns an error if the database query fails of if no engrams are found
/// for the given theme and variant combination.
pub fn get_by_theme_name_and_variant(
    conn: &rusqlite::Connection,
    theme_name: &str,
    variant: &EngramVariant,
) -> Result<Engram> {
    let mut stmt = conn.prepare(
        "SELECT token_path, op_type, w_bg, w_fg, w_accent, w_success, w_warning, w_error,
            delta_l, alpha
        FROM engrams WHERE theme_name = ?1 AND variant = ?2",
    )?;

    let rows = stmt.query_map(rusqlite::params![theme_name, variant], |row| {
        let token_path: String = row.get(0)?;
        let op_type_raw: u8 = row.get(1)?;

        Ok((
            token_path,
            op_type_raw,
            EngramVector {
                op_type: OpType::Direct,
                weights: [
                    row.get(2)?,
                    row.get(3)?,
                    row.get(4)?,
                    row.get(5)?,
                    row.get(6)?,
                    row.get(7)?,
                ],
                delta_l: row.get(8)?,
                alpha: row.get(9)?,
            },
        ))
    })?;

    let mut engram: Engram = std::collections::HashMap::new();
    for row in rows {
        let (token_path, op_type_raw, mut vector) = row?;
        vector.op_type = OpType::from(op_type_raw);
        engram.insert(token_path, vector);
    }

    if engram.is_empty() {
        return Err(crate::KenazError::StyleNotFound {
            theme_name: theme_name.to_string(),
            variant: *variant,
        });
    }

    Ok(engram)
}

/// Resolves the path to the `engrams.db` SQLite database
///
/// It attempts to use the OS-specific local data directory. If unavailable,
/// it falls back to the general data directory, and finally to the current
/// working directory as a last resort.
pub fn path() -> std::path::PathBuf {
    use crate::util::cache_dir;
    let mut path = cache_dir();
    path.push("engrams.db");
    path
}
