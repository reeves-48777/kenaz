//! Developer tools for building the engram database.
//!
//! This module is only compile with the `dev-tools` feature. It provides the
//! `EngramBuilder` which scrapes the Zed extension ecosystem (GitHub), downloads
//! theme JSON files, extracts their mathematical DNA (engrams), and populates
//! the local SQLite database.
//!
//! Because it makes heavy use of network requests, it uses an asynchronous
//! runtime (tokio) with a semaphore to avoir hitting Github's rate limits.

use crate::{
    engram::{db::prelude::*, fit, prelude::EngramVariant},
    schema, util,
};
use serde::Deserialize;

use std::path::Path;

/// The list of theme to include in the lightweight "Curated Pack".
/// These are the themes that will be downloaded by end-users.
const CURATED_REPOS: &[&str] = &[
    "zed_official",
    "catppuccin",
    "catppuccin",
    "tokyo night themes",
    "nord",
    "ashen",
    "dracula",
];

/// Exports a curated subset of the database and its JSON canvas files.
///
/// This creates a lightweight `engrams.db` and a `styles/` folder containing
/// only the themes defined in `CURATED_REPOS`.
pub fn export_pack(source_db_path: &Path, output_archive: &Path, full: bool) -> anyhow::Result<()> {
    use flate2::{Compression, write::GzEncoder};
    use tar::Builder;

    // 1. Create temporary directory to prepare file structure
    let temp_dir = std::env::temp_dir().join("kenaz_export_tmp");
    if temp_dir.exists() {
        std::fs::remove_dir_all(&temp_dir)?;
    }
    std::fs::create_dir_all(&temp_dir)?;

    let dest_db_path = temp_dir.join("engrams.db");
    let dest_styles_dir = temp_dir.join("styles");
    std::fs::create_dir_all(&dest_styles_dir)?;

    // 2. Copy the curated canvas folders and extract theme names
    let source_style_dir = util::styles_dir();
    let mut theme_names_to_export = Vec::new();

    // If `full` is true, we gather all cache subdirectories
    // Else we only browse CURATED_REPOS list
    let repos_to_export: Vec<String> = if full {
        std::fs::read_dir(&source_style_dir)?
            .filter_map(|e| {
                let path = e.ok()?.path();
                if path.is_dir() {
                    path.file_name()?.to_str().map(|s| s.to_string())
                } else {
                    None
                }
            })
            .collect()
    } else {
        CURATED_REPOS.iter().map(|r| r.to_string()).collect()
    };

    for repo_name in repos_to_export {
        let src_repo_dir = source_style_dir.join(&repo_name);
        if !src_repo_dir.exists() {
            tracing::warn!("Curated repo '{repo_name}' not found in cache, skipping.");
            continue;
        }

        let dest_repo_dir = dest_styles_dir.join(repo_name);
        std::fs::create_dir_all(&dest_repo_dir)?;

        // Copy json file of source directory
        for file_entry in std::fs::read_dir(&src_repo_dir)? {
            let file_path = file_entry?.path();
            if !file_path.is_file() || file_path.extension().map_or(true, |ext| ext != "json") {
                continue;
            }

            let dest_file = dest_repo_dir.join(file_path.file_name().unwrap());
            std::fs::copy(&file_path, &dest_file)?;

            if let Ok(content) = std::fs::read_to_string(&file_path) {
                if let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) {
                    if let Some(themes) = json.get("themes").and_then(|t| t.as_array()) {
                        for theme in themes {
                            if let Some(name) = theme.get("name").and_then(|n| n.as_str()) {
                                theme_names_to_export.push(name.to_lowercase());
                            }
                        }
                    }
                }
            }
        }
    }

    tracing::info!(
        "Copied canvas JSON files for {} themes",
        theme_names_to_export.len()
    );

    // 3. Export database subset
    let source_conn = rusqlite::Connection::open(source_db_path)?;
    let dest_conn = rusqlite::Connection::open(&dest_db_path)?;

    // Initialize schema in destination
    EngramRecord::init_db(&dest_conn)?;

    let dest_db_path_str = dest_db_path.to_str().unwrap().replace("'", "''");

    // Attach destination DB to source to copy data easily
    source_conn.execute(
        &format!("ATTACH DATABASE '{}' AS dest", dest_db_path_str),
        [],
    )?;

    // Build the SQL IN clause: ('one dark', 'monokai', ...)
    let theme_names_sql = theme_names_to_export
        .iter()
        .map(|t| format!("'{}'", t.replace("'", "''")))
        .collect::<Vec<_>>()
        .join(", ");

    // Copy matching rows
    source_conn.execute(&format!("
        INSERT INTO dest.engrams (theme_name, variant, token_path, op_type, w_bg, w_fg, w_accent, w_success, w_warning, w_error, delta_l, alpha)
        SELECT theme_name, variant, token_path, op_type, w_bg, w_fg, w_accent, w_success, w_warning, w_error, delta_l, alpha
        FROM engrams
        WHERE LOWER(theme_name) IN ({})", theme_names_sql), [])?;

    source_conn.execute("DETACH DATABASE dest", [])?;
    dest_conn.close().map_err(|(_, e)| anyhow::anyhow!(e))?;
    source_conn.close().map_err(|(_, e)| anyhow::anyhow!(e))?;

    tracing::info!("Database exported to temp dir");

    // 4. Creating tar.gz archive
    tracing::info!("Packing into {output_archive:?}");
    let tar_gz = std::fs::File::create(output_archive)?;
    let enc = GzEncoder::new(tar_gz, Compression::default());
    let mut tar = Builder::new(enc);

    tar.append_dir_all("kenaz_pack", &temp_dir)?;
    tar.finish()?;

    std::fs::remove_dir_all(&temp_dir)?;

    tracing::info!("Pack successfully created at {output_archive:?}");

    Ok(())
}

// TODO: Consider renaming to `EngramFactory` to better reflect its role.
/// Builds the engram database by fetching and processing Zed themes.
///
/// This acts as a factory that orchestrates the scraping of official Zed theme
/// repositories, extract their style vectors, and persists them to SQLite.
pub struct EngramBuilder {
    client: Option<reqwest::Client>,
    skip_fetch: bool,
}

impl EngramBuilder {
    const GITHUB_ROOT_URL: &'static str = "https://github.com/";
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
        Self {
            client: None,
            skip_fetch: false,
        }
    }

    /// Initiliazes the HTTP client with the correct headers and user agent.
    ///
    /// Requires the `GITHUB_TOKEN` environment variable to be set to authenticate
    /// and avoid GitHub API rate limits.
    pub fn try_init_client(mut self) -> anyhow::Result<Self> {
        let token = std::env::var("GITHUB_TOKEN")?;
        let client = reqwest::Client::builder()
            .user_agent("kenaz")
            .default_headers({
                let mut headers = reqwest::header::HeaderMap::new();
                headers.insert(
                    reqwest::header::AUTHORIZATION,
                    format!("Bearer {token}").parse()?,
                );
                headers
            })
            .build()?;

        self.client = Some(client);
        Ok(self)
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
    pub async fn build_engrams(&mut self) -> anyhow::Result<()> {
        if !self.skip_fetch {
            self.fetch_themes().await?;
        }

        // Ensure the database directory exists before opening a connection
        let db_path = super::db::path();
        if let Some(parent_dir) = db_path.parent() {
            std::fs::create_dir_all(parent_dir)?;
        }
        let conn = rusqlite::Connection::open(db_path)?;
        EngramRecord::init_db(&conn)?;

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
                        let record = EngramRecord::builder()
                            .with_name(theme_name)
                            .with_variant(variant.clone())
                            .with_token_path(&token_path)
                            .with_vector(vector)
                            .build()?;

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
        conn.close()
            .map_err(|(_, e)| anyhow::anyhow!("Connection error: {e}"))?;

        tracing::info!("Engram build ended ! {total_themes} themes built and saved in database");
        Ok(())
    }

    /// Fetches theme JSON files from official Zed extension repositories.
    ///
    /// Queries the Zed API for a list of theme extensions, then concurrently
    /// downloads the raw JSON file from their respective GitHub repositories.
    /// Uses a semaphore to limit concurrent requests and avoid rate limiting.
    async fn fetch_themes(&mut self) -> anyhow::Result<()> {
        use std::sync::Arc;
        use tokio::{sync::Semaphore, task::JoinSet};

        // fetching official zed themes
        self.fetch_official_themes().await?;

        // we get repos lists here from zed api
        let repos: ExtensionResponse = self
            .client
            .as_ref()
            .expect("Client initialized")
            .get("https://cloud.zed.dev/extensions?max_schema_version=1&provides=themes")
            .send()
            .await?
            .json()
            .await?;

        // Limit concurrent downloads to be nice to the GitHub API
        let semaphore = Arc::new(Semaphore::new(Self::CONCURRENT_REQUESTS_LIMIT));
        let mut set = JoinSet::new();

        // here we fetch json theme files from repos list given above
        for repo in repos.data {
            let client = self.client.clone();
            let styles_dir = util::styles_dir();
            let semaphore = Arc::clone(&semaphore);

            set.spawn(async move {
                let _permit = semaphore.acquire_owned().await.unwrap();

                let theme_paths =
                    Self::find_theme_files(client.as_ref().unwrap(), &repo.repository)
                        .await
                        .map_err(|e| {
                            anyhow::anyhow!("cannot list theme files for {}: {e}", repo.repository)
                        })?;

                let repo_dir = styles_dir.join(&repo.name);
                std::fs::create_dir_all(&repo_dir)?;

                for theme_path in &theme_paths {
                    let raw_url = format!(
                        "https://raw.githubusercontent.com/{}/main/{}",
                        repo.repository.trim_start_matches(Self::GITHUB_ROOT_URL),
                        theme_path
                    );

                    let content = client
                        .as_ref()
                        .unwrap()
                        .get(&raw_url)
                        .send()
                        .await?
                        .text()
                        .await?;

                    let file_name = theme_path.rsplit('/').next().unwrap_or(theme_path);

                    std::fs::write(repo_dir.join(file_name), content)?;
                }

                Ok::<(), anyhow::Error>(())
            });
        }

        // Wait for all tasks to complete and collect results
        let mut results = Vec::new();
        while let Some(res) = set.join_next().await {
            match res {
                Ok(theme_result) => results.push(theme_result),
                Err(join_err) => {
                    tracing::warn!("Task panicked: {join_err}");
                }
            }
        }

        let (ok, errors): (Vec<_>, Vec<_>) = results.into_iter().partition(Result::is_ok);
        tracing::info!(
            "{} repos successfully fetched, {} failures",
            ok.len(),
            errors.len()
        );

        tracing::info!("themes written at: {:?}", util::styles_dir());

        Ok(())
    }

    /// Queries the GitHub Tree API to find all `.json` files within a repository's `themes/` directory.
    ///
    /// This is more efficient that downloading the whole repository as a zip file.
    async fn find_theme_files(client: &reqwest::Client, url: &str) -> anyhow::Result<Vec<String>> {
        let target = url
            .strip_prefix(Self::GITHUB_ROOT_URL)
            .ok_or_else(|| anyhow::anyhow!("unexpected repository URL format: {url}"))?;

        let url = format!("https://api.github.com/repos/{target}/git/trees/main?recursive=1");

        let response: TreeResponse = client.get(&url).send().await?.json().await?;

        tracing::debug!("Received {} tree entries for {url}", response.tree.len());

        let theme_files = response
            .tree
            .into_iter()
            .filter(|entry| entry.r#type == "blob")
            .filter(|entry| entry.path.contains("themes/") && entry.path.ends_with(".json"))
            .map(|entry| entry.path)
            .collect();

        Ok(theme_files)
    }

    /// Fetches official base themes from the Zed repository
    async fn fetch_official_themes(&mut self) -> anyhow::Result<()> {
        tracing::info!("Fetching official Zed themes...");
        let tree_url = "https://api.github.com/repos/zed-industries/zed/git/trees/main?recursive=1";

        let response: TreeResponse = self
            .client
            .as_ref()
            .unwrap()
            .get(tree_url)
            .send()
            .await?
            .json()
            .await?;

        let styles_dir = util::styles_dir();
        let repo_dir = styles_dir.join("zed_official");
        std::fs::create_dir_all(&repo_dir)?;

        for entry in response.tree {
            // we only use .json files in assets/themes
            if entry.r#type == "blob"
                && entry.path.contains("assets/themes")
                && entry.path.ends_with(".json")
            {
                let raw_url = format!(
                    "https://raw.githubusercontent.com/zed-industries/zed/main/{}",
                    entry.path
                );
                let content = self
                    .client
                    .as_ref()
                    .unwrap()
                    .get(&raw_url)
                    .send()
                    .await?
                    .text()
                    .await?;

                let file_name = entry.path.rsplit('/').next().unwrap_or("theme.json");
                std::fs::write(repo_dir.join(file_name), content)?;
            }
        }
        tracing::info!("Official Zed themes fetched.");
        Ok(())
    }
}

/// Represents the GitHub Tree API response.
#[derive(Debug, Deserialize)]
struct TreeResponse {
    tree: Vec<TreeEntry>,
}

/// Represents a single entry in the GitHub Tree API response.
#[derive(Debug, Deserialize)]
struct TreeEntry {
    r#type: String,
    path: String,
}

/// Represents the Zed extension API response.
#[derive(Debug, Deserialize)]
struct ExtensionResponse {
    data: Vec<ExtensionEntry>,
}

/// Represents a single extension in the Zed extension API response.
#[derive(Debug, Deserialize)]
struct ExtensionEntry {
    name: String,
    repository: String,
}
