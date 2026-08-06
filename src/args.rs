use clap::Parser;
use std::path::PathBuf;

/// Theme fixed for zed
#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Valid theme file made by zed team (will contain most used values)
    #[arg(short, long = "src")]
    pub source: PathBuf,

    /// Theme file to fix (may not contain all values)
    #[arg(short, long = "dst")]
    pub destination: PathBuf,

    /// Output path to write to (optional: defaults to destination path)
    #[arg(short, long)]
    pub output: Option<PathBuf>,
}
