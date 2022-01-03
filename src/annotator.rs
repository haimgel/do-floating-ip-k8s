use anyhow::{Context, Result};
use k8s_openapi::api::core::v1::Pod;
use kube::api::{Api, Patch, PatchParams};
use kube::Client;
use serde_json::json;
use std::env;
use tracing::{debug, info};

#[tracing::instrument]
pub async fn annotate(anchor_ip: &str) -> Result<()> {
    let pod_name = env::var("POD_NAME").context("POD_NAME is not defined")?;
    let pod_namespace = env::var("POD_NAMESPACE").unwrap_or(String::from("default"));

    let client = Client::try_default().await?;
    let pods: Api<Pod> = Api::namespaced(client, &pod_namespace);

    let patch = json!({
        "apiVersion": "v1",
        "kind": "Pod",
        "metadata": {
            "annotations": {
                "k8s.haim.dev/digital-ocean-anchor-ip": anchor_ip
            },
        },
    });
    debug!(
        pod = pod_name.as_str(),
        namespace = pod_namespace.as_str(),
        ip = anchor_ip,
        "Applying annotation"
    );
    let patch_params = PatchParams::default();
    pods.patch(&pod_name, &patch_params, &Patch::Strategic(&patch)).await?;
    info!(
        pod = pod_name.as_str(),
        namespace = pod_namespace.as_str(),
        ip = anchor_ip,
        "Annotation applied successfully"
    );
    Ok(())
}
