use crate::errors::Error;
use anyhow::Context;
use digitalocean::prelude::*;
use std::net::IpAddr;
use tracing::{debug, error, info};

pub async fn move_floating_ip_to_node(ip: IpAddr, droplet_id: usize) -> Result<(), Error> {
    let api_key = std::env::var("DIGITALOCEAN_TOKEN").context("DIGITALOCEAN_TOKEN environment variable not defined")?;

    // TODO: digitalocean crate is using old (sync) Reqwest, ideally should switch to 0.11.x reqwest
    let action: Option<Action> = {
        let ip = ip;
        let droplet_id = droplet_id;
        let api_key = api_key.clone();
        tokio::task::spawn_blocking(move || {
            let client = digitalocean::DigitalOcean::new(api_key.as_str())?; // { client: reqwest::client::Client::new(), token: api_key };
            let floating_ip = FloatingIp::get(ip).execute(&client)?;
            if let Some(droplet) = floating_ip.droplet() {
                if droplet.id() == &droplet_id {
                    info!(
                        ip = ip.to_string().as_str(),
                        droplet = droplet_id,
                        "Floating IP is already attached to this droplet"
                    );
                    return Result::<Option<Action>, Error>::Ok(None);
                }
            }
            Ok(Some(FloatingIp::get(ip).assign(droplet_id).execute(&client)?))
        })
        .await
        .map_err(anyhow::Error::from)??
    };

    if let Some(action) = action {
        loop {
            let action: Action = {
                let action_id = *action.id();
                let api_key = api_key.clone();
                tokio::task::spawn_blocking(move || {
                    let client = digitalocean::DigitalOcean::new(api_key.as_str())?; // { client: reqwest::client::Client::new(), token: api_key };
                    Result::<Action, Error>::Ok(Action::get(action_id).execute(&client)?)
                })
                .await
                .map_err(anyhow::Error::from)??
            };
            if action.status() == "completed" {
                info!(
                    ip = ip.to_string().as_str(),
                    droplet = droplet_id,
                    "Floating IP attached to droplet"
                );
                return Ok(());
            }
            if action.status() == "errored" {
                error!(
                    ip = ip.to_string().as_str(),
                    droplet = droplet_id,
                    "Floating IP attach failed"
                );
                return Err(Error::FloatingIpAttachFailed);
            }
            debug!(
                ip = ip.to_string().as_str(),
                droplet = droplet_id,
                "Floating IP attach in progress"
            );
            tokio::time::sleep(std::time::Duration::from_secs(5)).await;
        }
    }
    Ok(())
}
