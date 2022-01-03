use anyhow::Result;
use k8s_openapi::api::core::v1::Node;
use kube::api::{Api, Patch, PatchParams};
use kube::Client;
use serde_json::json;
use tracing::{debug, info};

pub async fn annotate_node(node_name: &str, anchor_ip: &str) -> Result<()> {
    let client = Client::try_default().await?;
    let nodes: Api<Node> = Api::all(client);

    let patch = json!({
        "apiVersion": "v1",
        "kind": "Node",
        "metadata": {
            "annotations": {
                "k8s.haim.dev/digital-ocean-anchor-ip": anchor_ip
            },
            "label": {
                "k8s.haim.dev/digital-ocean-anchor-ip": "saved"
            },
        },
    });
    debug!(node = node_name, ip = anchor_ip, "Applying annotation");
    let patch_params = PatchParams::default();
    nodes.patch(node_name, &patch_params, &Patch::Strategic(&patch)).await?;
    info!(node = node_name, ip = anchor_ip, "Annotation applied successfully");
    Ok(())
}
