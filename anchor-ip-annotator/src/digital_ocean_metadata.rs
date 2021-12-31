use anyhow::Result;
use serde::Deserialize;
use std::env;
use tracing::{debug, info};

// Only the minimal set of values are deserialized here, we care only about the Anchor IP
// Other params are defined only so that we have at least a minimal validation during deserialization

#[allow(dead_code)]
#[derive(Deserialize)]
struct DigitalOceanInterfaceInfo {
    pub ip_address: String,
    pub netmask: String,
    pub gateway: String,
}

#[derive(Deserialize)]
struct DigitalOceanPublicInterface {
    anchor_ipv4: DigitalOceanInterfaceInfo,
}

#[derive(Deserialize)]
struct DigitalOceanInterfaces {
    public: [DigitalOceanPublicInterface; 1],
}

#[allow(dead_code)]
#[derive(Deserialize)]
struct DigitalOceanNodeInfo {
    droplet_id: i64,
    hostname: String,
    interfaces: DigitalOceanInterfaces,
}

#[tracing::instrument]
pub async fn anchor_ip() -> Result<String> {
    // This is a link-local address that is available to all Digital Ocean nodes (and pods on them)
    let url = env::var("DIGITAL_OCEAN_METADATA_URL")
        .unwrap_or(String::from("http://169.254.169.254/metadata/v1.json"));
    debug!(url = url.as_str(), "Fetching Digital Ocean host metadata");
    let res: DigitalOceanNodeInfo = reqwest::get(url).await?.json().await?;
    let anchor_ip = res.interfaces.public[0].anchor_ipv4.ip_address.clone();
    info!(
        hostname = res.hostname.as_str(),
        droplet = res.droplet_id,
        anchor_ip = anchor_ip.as_str(),
        "Fetched Digital Ocean host metadata"
    );
    Ok(anchor_ip)
}
