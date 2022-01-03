use anyhow::Result;
use do_floating_ip_k8s::annotator;
use do_floating_ip_k8s::digital_ocean_metadata;
use do_floating_ip_k8s::logging;
use tracing::error;

async fn annotate_anchor_ip() -> Result<()> {
    let anchor_ip = digital_ocean_metadata::anchor_ip().await?;
    annotator::annotate(&anchor_ip).await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    logging::setup();
    std::process::exit(annotate_anchor_ip().await.map_or_else(
        |err| {
            error!(root_cause = err.root_cause(), "{}", err);
            1
        },
        |()| 0,
    ))
}
