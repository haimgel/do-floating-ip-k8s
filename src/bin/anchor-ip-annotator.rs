use anyhow::Result;
use do_floating_ip_k8s::annotator;
use do_floating_ip_k8s::digital_ocean_metadata;
use do_floating_ip_k8s::logging;
use tracing::error;

async fn annotate_anchor_ip() -> Result<()> {
    let node_info = digital_ocean_metadata::node_info().await?;
    annotator::annotate_node(&node_info.hostname, &node_info.anchor_ip).await?;
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
