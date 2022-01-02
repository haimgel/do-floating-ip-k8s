use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Missing object key: {0}")]
    MissingObjectKey(&'static str),
    #[error("Missing floating IP annotation or label")]
    MissingFloatingIpAddressAnnotationOrLabel,
    #[error("Wrong provider ID")]
    WrongProviderId,
    #[error("Kubernetes API error: {0}")]
    KubeApiFailure(#[from] kube::error::Error),
    #[error("Missing environment variable {0}")]
    MissingEnvVar(#[from] std::env::VarError),
    #[error("Invalid floating IP address annotation: {0}")]
    InvalidFloatingIpAddress(String),
    #[error("DigitalOcean API error: {0}")]
    DigitalOceanApiFailure(#[from] digitalocean::error::Error),
    #[error("Floating IP attachment failed")]
    FloatingIpAttachFailed,
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
