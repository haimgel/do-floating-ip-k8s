use crate::errors::Error;
use crate::floating_ip;
use crate::kubernetes_helpers;
use anyhow::Result;
use futures::StreamExt;
use k8s_openapi::api::core::v1::Pod;
use kube::{
    api::{Api, ListParams},
    runtime::controller::{Context, Controller, ReconcilerAction},
};
use tracing::{debug, trace, warn};

// Data we want access to in error/reconcile calls
struct ContextData {
    client: kube::Client,
}

/// Controller triggers this whenever our main object or our children changed
async fn reconcile(pod: Pod, ctx: Context<ContextData>) -> Result<ReconcilerAction, Error> {
    let client = ctx.get_ref().client.clone();

    let name = pod
        .metadata
        .name
        .as_ref()
        .ok_or(Error::MissingObjectKey(".metadata.name"))?;
    if kubernetes_helpers::is_pod_running_and_ready(&pod) {
        debug!(pod = name.as_str(), "Pod is running and ready");
        let node_name = kubernetes_helpers::get_node_name(&pod)?;
        let droplet_id = kubernetes_helpers::get_digital_ocean_droplet_id(client, node_name).await?;
        let floating_ip = kubernetes_helpers::get_pod_floating_ip(&pod)?;
        floating_ip::move_floating_ip_to_node(floating_ip, droplet_id).await?;
    } else {
        debug!(
            pod = name.as_str(),
            "Pod is either not running or not ready, ignoring it"
        );
    }
    Ok(ReconcilerAction { requeue_after: None })
}

/// The controller triggers this on reconcile errors
fn error_policy(error: &Error, _ctx: Context<ContextData>) -> ReconcilerAction {
    warn!(error = format!("{}", error).as_str(), "Reconcile failed");
    ReconcilerAction {
        requeue_after: Some(tokio::time::Duration::from_secs(10)),
    }
}

pub async fn run() -> Result<(), Error> {
    let client = kube::Client::try_default().await?;
    let namespace = std::env::var("NAMESPACE").unwrap_or_else(|_| String::from("default"));
    let pods: Api<Pod> = Api::namespaced(client.clone(), namespace.as_str());

    let lp = ListParams::default().labels("k8s.haim.dev/floating-ip").timeout(60);

    // See:
    // https://github.com/kube-rs/kube-rs/blob/master/examples/secret_syncer.rs
    // https://github.com/kube-rs/kube-rs/blob/master/examples/configmapgen_controller.rs
    Controller::new(pods, lp)
        .shutdown_on_signal()
        .run(reconcile, error_policy, Context::new(ContextData { client }))
        .for_each(|result| async move { trace!("Reconciled: {:?}", result) })
        .await;
    Ok(())
}
