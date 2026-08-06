mod app;
mod args;
mod schema;
mod util;

use app::App;
use args::Args;
use clap::Parser;

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let args = Args::parse();
    let mut app = App::new(args.source, args.destination, args.output);
    app.run()?;

    Ok(())
}
