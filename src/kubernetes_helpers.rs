use crate::errors::Error;
use k8s_openapi::api::core::v1::{Node, Pod};
use lazy_static::lazy_static;
use regex::Regex;
use std::net::IpAddr;

/// Checks if the given pod is running and is in the ready state.
pub fn is_pod_running_and_ready(pod: &Pod) -> bool {
    return if let Some(status) = &pod.status {
        if status.phase != Some("Running".into()) {
            false
        } else if let Some(conditions) = &status.conditions {
            conditions.iter().any(|c| c.type_ == "Ready" && c.status == "True")
        } else {
            false
        }
    } else {
        false
    };
}

/// Gets the floating IP for a given pod, either from the annotation or from the label.
/// Both are supported, because IPv6 addresses cannot be put into labels.
pub fn get_pod_floating_ip(pod: &Pod) -> Result<IpAddr, Error> {
    let annotations = pod
        .metadata
        .annotations
        .as_ref()
        .ok_or(Error::MissingObjectKey(".metadata.annotations"))?;
    if let Some(ip_str) = annotations.get("k8s.haim.dev/floating-ip") {
        let ip = ip_str
            .parse::<IpAddr>()
            .map_err(|_| Error::InvalidFloatingIpAddress(ip_str.clone()))?;
        return Ok(ip);
    }

    let labels = pod
        .metadata
        .labels
        .as_ref()
        .ok_or(Error::MissingObjectKey(".metadata.labels"))?;
    if let Some(ip_str) = labels.get("k8s.haim.dev/floating-ip") {
        let ip = ip_str
            .parse::<IpAddr>()
            .map_err(|_| Error::InvalidFloatingIpAddress(ip_str.clone()))?;
        return Ok(ip);
    }

    Err(Error::MissingFloatingIpAddressAnnotationOrLabel)
}

/// Get pod's node name
pub fn get_node_name(pod: &Pod) -> Result<&String, Error> {
    let spec = pod.spec.as_ref().ok_or(Error::MissingObjectKey(".spec"))?;
    spec.node_name
        .as_ref()
        .ok_or(Error::MissingObjectKey(".spec.node_name"))
}

/// Get a DigitalOcean droplet ID where a given Pos is currently running.
pub async fn get_digital_ocean_droplet_id(client: kube::Client, node_name: &str) -> Result<usize, Error> {
    let nodes: kube::api::Api<Node> = kube::api::Api::all(client);
    let node = nodes.get(node_name).await?;
    let spec = &node.spec.ok_or(Error::MissingObjectKey(".spec"))?;
    let provider_id = spec
        .provider_id
        .as_ref()
        .ok_or(Error::MissingObjectKey(".spec.provider_id"))?;
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^digitalocean://(?P<id>\d+)$").unwrap();
    }
    let result = RE
        .captures(provider_id)
        .and_then(|cap| cap.name("id").map(|id| id.as_str().parse::<usize>()))
        .ok_or(Error::WrongProviderId)?
        .map_err(|_| Error::WrongProviderId)?;
    Ok(result)
}
