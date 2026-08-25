use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

pub struct FileManager {
    output: PathBuf,
}

impl FileManager {
    pub fn new(output: &Path) -> Self {
        Self {
            output: output.to_path_buf(),
        }
    }

    pub fn save(&self, contents: String) -> anyhow::Result<()> {
        std::fs::write(&self.output, contents)?;
        tracing::info!("wrote theme in {:?}", self.output);
        Ok(())
    }

    /// Creates a timestamped backup of the output file if it already exists
    ///
    /// This prevents data loss when iterating on a theme. The backup is named
    /// using a Unix timestamp (e.g., `theme_1692198400.json.bak`).
    pub fn backup(&self) -> anyhow::Result<()> {
        if !self.output.exists() {
            return Ok(());
        }

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);

        let stem = self
            .output
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("theme");
        let extension = self
            .output
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("json");

        let backup_name = format!("{}_{}.{}", stem, timestamp, extension);

        let mut backup_path = self.output.to_path_buf();
        backup_path.set_file_name(backup_name);

        std::fs::rename(&self.output, &backup_path)?;
        tracing::info!("Backup created at: {:?}", backup_path);

        Ok(())
    }
}
