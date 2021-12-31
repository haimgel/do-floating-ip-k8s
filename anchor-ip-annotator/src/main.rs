use anyhow::Result;
use std::env;
mod annotator;
mod digital_ocean_metadata;
use tracing::error;
use tracing_subscriber::{EnvFilter, FmtSubscriber};

fn setup_logging() {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info");
    }
    let subscriber = FmtSubscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        .json()
        .flatten_event(true)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
}

async fn annotate_anchor_ip() -> Result<()> {
    let anchor_ip = digital_ocean_metadata::anchor_ip().await?;
    annotator::annotate(&anchor_ip).await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    setup_logging();
    std::process::exit(annotate_anchor_ip().await.map_or_else(
        |err| {
            error!(root_cause = err.root_cause(), "{}", err);
            1
        },
        |()| 0,
    ))
}
