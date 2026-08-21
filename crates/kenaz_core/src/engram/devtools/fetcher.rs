//! Network abstraction for fetching theme data.
//!
//! This module defines the `ThemeFetcher` trait, allowing for Dependency Injection.
//! This makes the network logic fully testable without making real HTTP requests
//! by substituting the `GithubFetcher` with a `MockFetcher` during tests.

use super::{
    api_response::{ExtensionResponse, TreeResponse},
    consts::*,
};
use std::time::Duration;
use ureq::{
    Body, SendBody,
    http::{HeaderValue, Request, Response},
    middleware::MiddlewareNext,
};

/// Cleans and validate a GitHub repository URL to extract "owner/repo" format.
pub fn clean_github_url(repo_url: &str) -> anyhow::Result<String> {
    let target = repo_url
        .trim_end_matches('/')
        .strip_prefix(GITHUB_ROOT_URL)
        .ok_or_else(|| anyhow::anyhow!("Unexpected repository URL format: {repo_url}"))?;

    let target = target.trim_end_matches(".git");
    let target = target.split('/').take(2).collect::<Vec<_>>().join("/");

    if target.split('/').count() != 2 {
        return Err(anyhow::anyhow!(
            "Invalid repo format after cleaning: {target}"
        ));
    }

    Ok(target)
}

/// A trait for fetching theme data. Allows for mocking in tests.
///
/// Requires `Send + Sync` to be safely shared across multiple threads via `Arc`.
pub trait ThemeFetcher: Send + Sync {
    fn fetch_repos(&self) -> anyhow::Result<ExtensionResponse>;
    fn fetch_tree(&self, repo_url: &str) -> anyhow::Result<(Vec<String>, String)>;
    fn fetch_raw_file(&self, url: &str) -> anyhow::Result<String>;
}

/// The real fetcher that hits GitHub and Zed's API using `ureq`.
pub struct GithubFetcher {
    agent: ureq::Agent,
}

impl GithubFetcher {
    const ZED_EXTENSION_API_URL: &'static str =
        "https://cloud.zed.dev/extensions?max_schema_version=1&provides=themes";
    const GITHUB_API_ROOT_URL: &'static str = "https://api.github.com/repos";

    /// Initializes the HTTP client with the correct headers and user agent.
    pub fn try_new() -> anyhow::Result<Self> {
        let client = ureq::Agent::config_builder()
            .user_agent("kenaz")
            .timeout_global(Some(Duration::from_secs(15)))
            .middleware(Self::middleware)
            .build()
            .into();
        Ok(Self { agent: client })
    }

    /// Middleware that adds needed headers to each reqwest
    fn middleware(
        mut req: Request<SendBody>,
        next: MiddlewareNext,
    ) -> Result<Response<Body>, ureq::Error> {
        if let Ok(token) = std::env::var("GITHUB_TOKEN") {
            let auth = format!("Bearer {token}");
            req.headers_mut().insert(
                "Authorization",
                HeaderValue::from_str(&auth)
                    .unwrap_or_else(|_| HeaderValue::from_static("Bearer invalid_token")),
            );
        }

        next.handle(req)
    }
}

impl ThemeFetcher for GithubFetcher {
    fn fetch_repos(&self) -> anyhow::Result<ExtensionResponse> {
        Ok(self
            .agent
            .get(Self::ZED_EXTENSION_API_URL)
            .call()?
            .body_mut()
            .read_json()?)
    }

    fn fetch_tree(&self, repo_url: &str) -> anyhow::Result<(Vec<String>, String)> {
        let target = clean_github_url(repo_url)?;

        let root_url = Self::GITHUB_API_ROOT_URL;
        let url = format!("{root_url}/{target}/git/trees/main?recursive=1");

        let (mut response, branch) = match self.agent.get(url).call() {
            Ok(r) => (r, "main".to_string()),
            Err(ureq::Error::StatusCode(404)) => {
                tracing::debug!("'main' branch not found for {target}, trying 'master' instead...");

                let root_url = Self::GITHUB_API_ROOT_URL;
                let url_master = format!("{root_url}/{target}/git/trees/master?recursive=1");
                (self.agent.get(&url_master).call()?, "master".to_string())
            }
            Err(e) => return Err(e.into()),
        };

        let response: TreeResponse = response.body_mut().read_json()?;
        let theme_files = response
            .tree
            .into_iter()
            .filter(|entry| entry.r#type == "blob")
            .filter(|entry| entry.path.contains("themes/") && entry.path.ends_with(".json"))
            .map(|entry| entry.path)
            .collect();

        Ok((theme_files, branch))
    }

    fn fetch_raw_file(&self, url: &str) -> anyhow::Result<String> {
        Ok(self.agent.get(url).call()?.body_mut().read_to_string()?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clean_github_url_valid() {
        let url = "https://github.com/zed-industries/zed";
        let target = clean_github_url(url).unwrap();
        assert_eq!(target, "zed-industries/zed");
    }

    #[test]
    fn test_clean_github_url_with_dot_git() {
        let url = "https://github.com/KimNorgaard/zed-neovim-default.git";
        let target = clean_github_url(url).unwrap();
        assert_eq!(target, "KimNorgaard/zed-neovim-default");
    }

    #[test]
    fn test_clean_github_url_invalid_prefix() {
        let url = "https://gitlab.com/invalid/repo";
        let result = clean_github_url(url);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Unexpected repository URL format")
        );
    }

    #[test]
    fn test_clean_github_url_subdirectory() {
        let url = "https://github.com/kepano/flexoki/tree/main/zed";
        let target = clean_github_url(url).unwrap();
        assert_eq!(target, "kepano/flexoki");
    }
}
