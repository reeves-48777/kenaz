//! Defines the databse schema and the `EngramRecord` struct.
//!
//! This module bridges the mathematical `EngramVector` and the SQLite database.
//! It handles table creation, indexes and the `INSERT OR REPLACE` (upsert) logic
//! to persist extracted style vectors efficiently.

use crate::{
    engram::prelude::{EngramVariant, EngramVector},
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
    /// Creates a new `EngramRecordBuilder` to compose the record step-by-step.
    pub fn builder() -> EngramRecordBuilder {
        EngramRecordBuilder::new()
    }

    /// Initializes the database schema by creating the `engrams` table and its indexes.
    ///
    /// This is safe to call multiple times as it uses `IF NOT EXISTS`.
    /// WARNING: Any schema changes must be applied here (and ideally via a migration system in the future).
    pub fn init_db(conn: &rusqlite::Connection) -> anyhow::Result<()> {
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
    pub fn upsert(&self, conn: &rusqlite::Connection) -> anyhow::Result<()> {
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
}

/// A builder struct to construct an `EngramRecord` step-by-step
///
/// All fields are `Option` to facilitate composing the record from different
/// data sources (metadata like theme name vs. mathematical data like the vector).
pub struct EngramRecordBuilder {
    theme_name: Option<String>,
    variant: Option<EngramVariant>,
    token_path: Option<String>,
    vector: Option<EngramVector>,
}

impl EngramRecordBuilder {
    pub fn new() -> Self {
        Self {
            theme_name: None,
            variant: None,
            token_path: None,
            vector: None,
        }
    }

    pub fn with_name(mut self, name: &str) -> Self {
        self.theme_name = Some(name.to_owned());
        self
    }

    pub fn with_variant(mut self, variant: EngramVariant) -> Self {
        self.variant = Some(variant);
        self
    }

    pub fn with_token_path(mut self, token_path: &str) -> Self {
        self.token_path = Some(token_path.to_owned());
        self
    }

    pub fn with_vector(mut self, vector: EngramVector) -> Self {
        self.vector = Some(vector);
        self
    }

    /// Consumes the builder and return the final `EngramRecord`.
    ///
    /// # Panics
    /// Panics if any of the expected fields (`theme_name`, `variant`, `token_path`, `vector`)
    /// have not been provided.
    pub fn build(self) -> anyhow::Result<EngramRecord> {
        let theme_name = self.theme_name.expect("Theme name is present");
        let variant = self.variant.expect("EngramVariant is present");
        let token_path = self.token_path.expect("Token path is present");
        let vector = self.vector.expect("EngramVector is present");

        Ok(EngramRecord {
            id: None,
            theme_name,
            variant,
            token_path,
            op_type: vector.op_type as u8,
            weights: vector.weights,
            delta_l: vector.delta_l,
            alpha: vector.alpha,
        })
    }
}
