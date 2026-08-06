use crate::schema;
use std::path::Path;

pub fn parse_file<T>(file_path: T) -> anyhow::Result<schema::ZedThemeSpec>
where
    T: AsRef<Path>,
{
    let json = std::fs::read_to_string(file_path)?;
    let spec: schema::ZedThemeSpec = serde_json::from_str(&json)?;
    Ok(spec)
}
