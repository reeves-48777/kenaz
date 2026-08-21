//! Serde structs for deserializing API responses from Zed and GitHub.

use serde::Deserialize;

/// Represents the GitHub Tree API response.
#[derive(Debug, Deserialize)]
pub struct TreeResponse {
    pub tree: Vec<TreeEntry>,
}

/// Represents a single entry in the GitHub Tree API response.
#[derive(Debug, Deserialize)]
pub struct TreeEntry {
    pub r#type: String,
    pub path: String,
}

/// Represents the Zed extension API response.
#[derive(Debug, Deserialize)]
pub struct ExtensionResponse {
    pub data: Vec<ExtensionEntry>,
}

/// Represents a single extension in the Zed extension API response.
#[derive(Debug, Deserialize)]
pub struct ExtensionEntry {
    pub name: String,
    pub repository: String,
}
