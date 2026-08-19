//! Implements SQLite serialization (`ToSql` / `FromSql`) for `EngramVariant`.
//!
//! This allows the `EngramVariant` enum to be seamlessly stored as a lowercase
//! string (e.g., "dark", "light") in the database and automatically converted
//! back into a strongly-typed enum when queried.

use crate::engram::variant::EngramVariant;

use rusqlite::{
    ToSql,
    types::{FromSql, FromSqlError, ValueRef},
};

/// Serializes an `EngramVariant` into a SQLite TEXT value.
///
/// This allows passing the enum directly to `rusqlite::params!`.
impl ToSql for EngramVariant {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        match self {
            EngramVariant::Dark => Ok("dark".into()),
            EngramVariant::Light => Ok("light".into()),
        }
    }
}

/// Deserializes an SQLite TEXT value into an `EngramVariant`.
///
/// This allows reading the variant directly from a `rusqlite::Row`.
///
/// # Errors
/// Returns an `InvalidType` error if the databse contains an unrecognized string,
/// preventing silent data corruption.
impl FromSql for EngramVariant {
    fn column_result(value: ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        match value.as_str()? {
            "dark" => Ok(EngramVariant::Dark),
            "light" => Ok(EngramVariant::Light),
            _ => Err(FromSqlError::InvalidType),
        }
    }
}
