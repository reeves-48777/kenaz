//! Defines the database schema and the `EngramRecord` struct.
//!
//! This module bridges the mathematical `EngramVector` and the SQLite database.
//! It handles table creation, indexes and the `INSERT OR REPLACE` (upsert) logic
//! to persist extracted style vectors efficiently.

use crate::{
    engram::prelude::{EngramVariant, EngramVector},
    error::Result,
    util,
};
use serde::{Deserialize, Serialize};

/// Represents a single engram row in the SQLite database.
///
/// It flattens the `EngramVector`'s weights array into individual columns
/// (`w_bg`, `w_fg`, etc.) for efficient SQL querying and storage.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngramRecord {
    pub id: Option<i64>,
    pub theme_name: String,
    pub variant: EngramVariant,
    pub token_path: String,
    pub op_type: u8,
    pub weights: [f32; 6],
    pub delta_l: f32,
    pub alpha: f32,
}

impl EngramRecord {
    /// Creates a new in-memory `EngramRecord` ready to be persisted via [`upsert`](Self::upsert).
    ///
    /// `id` is always `None` here - it is assigned by SQLite's `AUTOINCREMENT` on insert.
    pub fn new(
        theme_name: &str,
        variant: EngramVariant,
        token_path: &str,
        vector: EngramVector,
    ) -> Self {
        Self {
            id: None,
            theme_name: theme_name.to_string(),
            variant,
            token_path: token_path.to_string(),
            op_type: vector.op_type as u8,
            weights: vector.weights,
            delta_l: vector.delta_l,
            alpha: vector.alpha,
        }
    }
    /// Initializes the database schema by creating the `engrams` table and its indexes.
    ///
    /// This is safe to call multiple times as it uses `IF NOT EXISTS`.
    /// WARNING: Any schema changes must be applied here (and ideally via a migration system in the future).
    pub fn create_table_if_not_exists(conn: &rusqlite::Connection) -> Result<()> {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS engrams (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                theme_name TEXT NOT NULL,
                variant TEXT NOT NULL,
                token_path TEXT NOT NULL,
                op_type TINYINT NOT NULL,
                w_bg REAL NOT NULL,
                w_fg REAL NOT NULL,
                w_accent REAL NOT NULL,
                w_success REAL NOT NULL,
                w_warning REAL NOT NULL,
                w_error REAL NOT NULL,
                delta_l REAL NOT NULL,
                alpha REAL NOT NULL,
                UNIQUE(theme_name, variant, token_path)
            )",
            [],
        )?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_token_variant ON engrams(token_path, variant)",
            [],
        )?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_theme ON engrams(theme_name)",
            [],
        )?;

        Ok(())
    }

    /// Inserts a new record or replaces the existing one if the `UNIQUE` constraint matches.
    ///
    /// The theme name is automatically normalized before insertion to ensure
    /// consistent querying.
    pub fn upsert(&self, conn: &rusqlite::Connection) -> Result<()> {
        conn.execute(
            "INSERT OR REPLACE INTO engrams (
                theme_name, variant, token_path, op_type,
                w_bg, w_fg, w_accent, w_success, w_warning, w_error,
                delta_l, alpha
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)",
            rusqlite::params![
                util::normalize_theme_name(&self.theme_name),
                self.variant,
                self.token_path,
                self.op_type,
                self.weights[0],
                self.weights[1],
                self.weights[2],
                self.weights[3],
                self.weights[4],
                self.weights[5],
                self.delta_l,
                self.alpha
            ],
        )?;
        Ok(())
    }

    /// Deletes all engrams associated with a specific theme.
    ///
    /// The theme name is normalized the same way as during insertion, so this
    /// works regardless of the original casing/spacing used.
    ///
    /// # Errors
    /// Returns an error if the underlying `DELETE` query fails.
    ///
    /// # Returns
    /// The number of rows deleted
    pub fn delete_by_theme_name(conn: &rusqlite::Connection, theme_name: &str) -> Result<usize> {
        let normalized_name = util::normalize_theme_name(theme_name);
        let rows_deleted = conn.execute(
            "DELETE FROM engrams WHERE theme_name = ?1",
            rusqlite::params![normalized_name],
        )?;
        Ok(rows_deleted)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engram::variant::EngramVariant;
    use crate::engram::vector::OpType;

    #[test]
    fn test_db_init_and_upsert() {
        let conn = rusqlite::Connection::open_in_memory().unwrap();

        EngramRecord::create_table_if_not_exists(&conn).unwrap();

        let v = EngramVector {
            op_type: OpType::Direct,
            weights: [1.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            delta_l: 0.0,
            alpha: 1.0,
        };

        let record = EngramRecord::new("Test theme", EngramVariant::Dark, "editor.background", v);

        assert!(record.upsert(&conn).is_ok());

        let mut stmt = conn
            .prepare("SELECT theme_name FROM engrams WHERE token_path = ?1")
            .unwrap();
        let name: String = stmt
            .query_row(rusqlite::params!["editor.background"], |row| row.get(0))
            .unwrap();
        assert_eq!(name, "test-theme"); // normalized when upserting
    }
}
