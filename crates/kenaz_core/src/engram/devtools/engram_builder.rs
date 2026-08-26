//! The main orchestrator for scraping and building the engram database.

use rayon::iter::IntoParallelRefIterator;

use super::consts::*;
use crate::{
    engram::{
        db::{self, prelude::*},
        devtools::fetcher::{GithubFetcher, ThemeFetcher},
        fit,
        variant::EngramVariant,
    },
    error::Result,
    schema, util,
};
use ::std::sync::Arc;

// TODO: Consider renaming to `EngramFactory` to better reflect its role.
/// Builds the engram database by fetching and processing Zed themes.
///
/// This acts as a factory that orchestrates the scraping of official Zed theme
/// repositories, extract their style vectors, and persists them to SQLite.
pub struct EngramBuilder {
    fetcher: Arc<dyn ThemeFetcher>,
    skip_fetch: bool,
}

impl EngramBuilder {
    // 10 is a pretty decent number, we do not need much
    const CONCURRENT_REQUESTS_LIMIT: usize = 10;

    /// Loads environment variables from a `.env` file.
    ///
    /// Returns `true` if the file was successfully loaded, `false` otherwise.
    pub fn ensure_dot_env() -> bool {
        match dotenvy::dotenv() {
            Ok(_) => true,
            Err(_) => {
                tracing::error!("No env file present...");
                false
            }
        }
    }

    pub fn new() -> Self {
        let fetcher = Arc::new(GithubFetcher::new());
        Self {
            fetcher,
            skip_fetch: false,
        }
    }

    #[cfg(test)]
    pub fn with_fetcher(fetcher: Arc<dyn ThemeFetcher>) -> Self {
        Self {
            fetcher,
            skip_fetch: false,
        }
    }

    /// If set to `true`, skips the network fetching phase and reuses the
    /// raw JSON files already present in the cache directory.
    pub fn skip_fetch(mut self, skip_fetch: bool) -> Self {
        self.skip_fetch = skip_fetch;
        self
    }

    /// Create or populate the SQLite database with engrams.
    ///
    /// This is the main orchestrator. It optionally fetches themes, then iterates
    /// through all downloaded JSON files, extracts the anchors, calculates the
    /// engram vectors, and saves them in a single database transaction for performance.
    pub fn build_engrams(&mut self) -> Result<()> {
        if !self.skip_fetch {
            self.fetch_themes()?;
        }

        // Ensure the database directory exists before opening a connection
        let db_path = db::path();
        if let Some(parent_dir) = db_path.parent() {
            std::fs::create_dir_all(parent_dir)?;
        }
        let conn = rusqlite::Connection::open(db_path)?;
        EngramRecord::create_table_if_not_exists(&conn)?;

        // Uses an unchecked transaction for massive performance gains during inserts
        let tx = conn.unchecked_transaction()?;

        let styles_dir = util::styles_dir();
        let mut total_themes = 0;

        for repo_entry in std::fs::read_dir(&styles_dir)? {
            let repo_dir = repo_entry?.path();
            if !repo_dir.is_dir() {
                continue;
            }

            for file_entry in std::fs::read_dir(&repo_dir)? {
                let path = file_entry?.path();
                if !path.is_file() || path.extension().map_or(true, |ext| ext != "json") {
                    continue;
                }

                let content = match std::fs::read_to_string(&path) {
                    Ok(c) => c,
                    Err(e) => {
                        tracing::warn!("Cannot read {path:?}: {e}");
                        continue;
                    }
                };

                // Parse as a generic JSON Value first to tolerate missing root fields (like 'author')
                let root_value: serde_json::Value = match serde_json::from_str(&content) {
                    Ok(s) => s,
                    Err(e) => {
                        tracing::warn!("Invalid JSON for {path:?}: {e}");
                        continue;
                    }
                };

                let Some(themes_array) = root_value.get("themes").and_then(|t| t.as_array()) else {
                    continue;
                };

                for theme_value in themes_array {
                    // Attempt to parse the individual theme into our strict schema
                    let theme: schema::ThemeContent =
                        match serde_json::from_value(theme_value.clone()) {
                            Ok(t) => t,
                            Err(e) => {
                                tracing::warn!("Theme ignored in {path:?}: {e}");
                                continue;
                            }
                        };

                    let theme_name = &theme.name;

                    let variant = EngramVariant::from(theme.appearance);

                    // Extract the 6 semantic anchors (bg, fg, etc.) from the theme
                    let anchors = match fit::extract_anchors(&theme) {
                        Ok(a) => a,
                        Err(e) => {
                            tracing::warn!("Missing anchors for {theme_name}: {e}");
                            continue;
                        }
                    };

                    // Calculate the engram vector for all tokens in this theme
                    let fitted_tokens = fit::fit_theme(&[&theme], &anchors);

                    // Persist the vectors to the database
                    for (token_path, vector) in fitted_tokens {
                        let record =
                            EngramRecord::new(theme_name, variant.clone(), &token_path, vector);

                        if let Err(e) = record.upsert(&tx) {
                            tracing::warn!("Upsert error {token_path} for {theme_name}: {e}");
                        }
                    }

                    total_themes += 1;
                    tracing::info!("Engrams saved for {theme_name} ({variant:?})");
                }
            }
        }

        tx.commit()?;
        conn.close_safely()?;

        tracing::info!("Engram build ended ! {total_themes} themes built and saved in database");
        Ok(())
    }

    /// Fetches theme JSON files from official Zed extension repositories.
    ///
    /// Queries the Zed API for a list of theme extensions, then concurrently
    /// downloads the raw JSON file from their respective GitHub repositories.
    /// Uses a semaphore to limit concurrent requests and avoid rate limiting.
    fn fetch_themes(&mut self) -> Result<()> {
        use rayon::iter::ParallelIterator;

        // fetching official zed themes
        self.fetch_official_themes()?;

        let repos = self.fetcher.fetch_repos()?;

        let fetcher = Arc::clone(&self.fetcher);

        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(Self::CONCURRENT_REQUESTS_LIMIT)
            .build()?;
        let errors = pool.install(|| {
            // here we fetch json theme files from repos list given above
            repos
                .data
                .par_iter()
                .filter_map(|repo| {
                    let result = (|| -> Result<()> {
                        let (theme_paths, branch) = fetcher.fetch_tree(&repo.repository)?;

                        let repo_dir = util::styles_dir().join(&repo.name);
                        std::fs::create_dir_all(&repo_dir)?;

                        for theme_path in &theme_paths {
                            let raw_url = format!(
                                "https://raw.githubusercontent.com/{}/{}/{}",
                                repo.repository.trim_start_matches(GITHUB_ROOT_URL),
                                branch,
                                theme_path
                            );

                            let content = fetcher.fetch_raw_file(&raw_url)?;

                            let file_name = theme_path.rsplit('/').next().unwrap_or(theme_path);
                            std::fs::write(repo_dir.join(file_name), content)?;
                        }
                        Ok(())
                    })();

                    if let Err(e) = result {
                        tracing::warn!("Thread failed: {e}");
                        Some(())
                    } else {
                        None
                    }
                })
                .count()
        });

        tracing::info!("Themes fetched. {errors} failures");
        Ok(())
    }

    /// Fetches official base themes from the Zed repository
    fn fetch_official_themes(&mut self) -> Result<()> {
        tracing::info!("Fetching official Zed themes...");
        let tree_url = "https://github.com/zed-industries/zed";

        let (theme_paths, branch) = self.fetcher.fetch_tree(tree_url)?;

        let styles_dir = util::styles_dir();
        let repo_dir = styles_dir.join("zed_official");
        std::fs::create_dir_all(&repo_dir)?;

        for path in &theme_paths {
            if path.contains("assets/themes") {
                let raw_url = format!(
                    "https://raw.githubusercontent.com/zed-industries/zed/{}/{}",
                    branch, path
                );
                let content = self.fetcher.fetch_raw_file(&raw_url)?;
                let file_name = path.rsplit('/').next().unwrap_or("theme.json");
                std::fs::write(repo_dir.join(file_name), content)?;
            }
        }
        tracing::info!("Official Zed themes fetched.");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::engram::devtools::api_response::{ExtensionEntry, ExtensionResponse};

    use super::*;

    struct MockFetcher;

    impl ThemeFetcher for MockFetcher {
        fn fetch_repos(&self) -> Result<ExtensionResponse> {
            let data = (0..15)
                .map(|i| ExtensionEntry {
                    name: format!("Mock theme {i}"),
                    repository: format!("https://github.com/mock/repo_{i}"),
                })
                .collect();
            Ok(ExtensionResponse { data })
        }

        fn fetch_tree(&self, _repo_url: &str) -> Result<(Vec<String>, String)> {
            Ok((
                vec!["themes/mock-theme.json".to_string()],
                "main".to_string(),
            ))
        }

        fn fetch_raw_file(&self, _url: &str) -> Result<String> {
            Ok(r##"{
                "themes": [{
                    "name": "Mock Theme",
                    "appearance": "dark",
                    "style": {
                        "background": "#1e1e1e",
                        "text": "#d4d4d4",
                        "text.accent": "#569cd6",
                        "success": "#4ec9b0",
                        "warning": "#dcdcaa",
                        "error": "#f44747",
                    }
                }]
            }"##
            .to_string())
        }
    }

    #[test]
    fn test_fetch_and_save_themes() {
        let temp_dir = std::env::temp_dir().join("kenaz_mock_test");
        unsafe {
            std::env::set_var("KENAZ_TEST_CACHE_DIR", &temp_dir);
        }

        let mock_fetcher: Arc<dyn ThemeFetcher> = Arc::new(MockFetcher);
        let mut builder = EngramBuilder::with_fetcher(mock_fetcher);

        builder.fetch_themes().unwrap();

        let styles_dir = temp_dir.join("styles");
        let dir_count = std::fs::read_dir(&styles_dir).unwrap().count();
        assert_eq!(
            dir_count,
            16, // "15 mocks + zed_official"
            "Should have created 15 mock dirs + 1 official theme dir concurrently"
        );

        let mock_file = temp_dir
            .join("styles")
            .join("Mock Theme 0")
            .join("mock-theme.json");
        assert!(mock_file.exists(), "Mock theme file should exist in cache");

        let content = std::fs::read_to_string(&mock_file).unwrap();
        assert!(
            content.contains("Mock Theme"),
            "File content must be the mock JSON"
        );

        unsafe {
            std::env::remove_var("KENAZ_TEST_CACHE_DIR");
        }

        let _ = fs::remove_dir_all(&temp_dir);
    }
}
