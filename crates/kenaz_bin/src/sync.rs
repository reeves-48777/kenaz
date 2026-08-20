use anyhow::Context;
use std::path::Path;

const FULL_PACK_NAME: &'static str = "kenaz_full_pack.tar.gz";
const CURATED_PACK_NAME: &'static str = "kenaz_curated_pack.tar.gz";

/// Synchronizes kenaz pack
pub fn sync_repo(full: bool) -> anyhow::Result<()> {
    let cache_dir = kenaz_core::util::cache_dir();

    let version = env!("CARGO_PKG_VERSION");
    let pack_name = if full {
        FULL_PACK_NAME
    } else {
        CURATED_PACK_NAME
    };

    let pack_url =
        format!("https://github.com/reeves-48777/kenaz/releases/download/v{version}/{pack_name}");

    if let Err(e) = download_and_extract_pack(&pack_url, &cache_dir) {
        tracing::error!("Could not download initial database: {e}");
        anyhow::bail!("Please try again or build the database manually with --build-engrams")
    }
    Ok(())
}

/// Downloads and extract the engram pack for a given URL
fn download_and_extract_pack(url: &str, extract_to: &Path) -> anyhow::Result<()> {
    tracing::info!("Downloading engram pack from {url}...");

    let response =
        reqwest::blocking::get(url).context("Failed to send HTTP request for engram pack")?;

    if !response.status().is_success() {
        anyhow::bail!("Failed to download pack: HTTP {}", response.status());
    }

    let bytes = response.bytes().context("Failed to read response bytes")?;

    tracing::info!("Extracting pack to {extract_to:?}...");
    std::fs::create_dir_all(extract_to)?;

    let gz = flate2::read::GzDecoder::new(&bytes[..]);
    let mut tar = tar::Archive::new(gz);

    // Iterate over archives entries to remove "kenaz_pack" prefix
    // in order to put files directly to cache folder root
    for entry in tar.entries()? {
        let mut entry = entry.context("Invalid tar entry")?;
        let path = entry.path()?.into_owned();

        // Removed prefix to flatten tree
        if let Ok(relative_path) = path.strip_prefix("kenaz_pack") {
            let dest = extract_to.join(relative_path);

            if let Some(parent) = dest.parent() {
                std::fs::create_dir_all(parent)?;
            }

            entry.unpack(&dest).context("Failed to unpack file")?;
        }
    }

    tracing::info!("Engram pack installed successfully!");
    Ok(())
}
