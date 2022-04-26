//! Set of structs that represent connection details against Kubernetes API
//! server

use std::fs;

use super::wasi_outbound_http;
use crate::errors::{KrewWasmSDKError, Result};

/// Connection details for a Kubernetes API server. Includes
/// details about the server and the user identity to be used.
pub struct ConnectionConfig {
    pub identity: UserIdentity,
    pub server: Server,
}

impl ConnectionConfig {
    /// Create a `ConnectionConfig` by parsing the contents of the default
    /// kubeconfig file
    pub fn from_kube_config() -> Result<ConnectionConfig> {
        let config = kube_conf::Config::load_default()?;

        let kube_ctx = config
            .get_current_context()
            .ok_or(KrewWasmSDKError::KubeConfigNoContextError())?;

        let cluster = kube_ctx
            .get_cluster(&config)
            .ok_or(KrewWasmSDKError::KubeConfigNoClusterDefinitionError())?;

        let user = kube_ctx
            .get_user(&config)
            .ok_or(KrewWasmSDKError::KubeConfigNoUserDefinitionError())?;

        let identity = UserIdentity::from_kube_user_and_cluster(&user, &cluster)?;
        let server = Server::from_cluster(&cluster)?;

        Ok(ConnectionConfig { identity, server })
    }

    /// Connecting against a Kubernetes API server requires some tuning for the
    /// WASI http guest library. This includes operations like: adding extra
    /// trusted root CA, handling the user identity.
    /// This can be done with the `wasi_outbound_http::register_request_config`
    /// function, but this method is a convenient wrapper around it.
    ///
    /// The method returns the ID that identifies this specific connection
    /// configuration. The method can be called as many times as wanted, but
    /// it will always return a new ID. Because of that, it's recommended to
    /// call the method only once and store the ID in a safe place.
    pub fn register<'a>(&'a self) -> Result<String> {
        let accept_invalid_hostnames = false;
        let accept_invalid_certificates = false;

        let server_cert = wasi_outbound_http::Certificate {
            encoding: wasi_outbound_http::CertificateEncoding::Pem,
            data: &self.server.ca,
        };
        let extra_root_certificates = vec![server_cert];

        let ui: &'a UserIdentity = &self.identity;
        let identity: wasi_outbound_http::Identity<'a> = ui.into();

        let req_cfg = wasi_outbound_http::RequestConfig {
            accept_invalid_hostnames,
            accept_invalid_certificates,
            extra_root_certificates: extra_root_certificates.as_slice(),
            identity: Some(identity),
        };

        let cfg_id = wasi_outbound_http::register_request_config(req_cfg, None)?;
        Ok(cfg_id)
    }
}

/// Describe the identity to be used when connecting to the Kubernetes
/// API server
pub struct UserIdentity {
    /// Private key assigned to the user
    pub key: Vec<u8>,
    /// Certificate assigned to the user
    pub cert: Vec<u8>,
    /// CA that issued the certificate
    pub ca: Vec<u8>,
}

impl UserIdentity {
    /// Creates a `UserIdentity` object by parsing the data stored inside of
    /// the given kubeconfig.
    fn from_kube_user_and_cluster(
        user: &kube_conf::user::User,
        cluster: &kube_conf::cluster::Cluster,
    ) -> Result<Self> {
        let ca = if let Some(data) = cluster.certificate_authority_data.as_ref() {
            base64::decode(data).map_err(|_| {
                KrewWasmSDKError::KubeConfigGenericError(
                    "cannot decode server certificate".to_string(),
                )
            })
        } else if let Some(path) = cluster.certificate_authority.as_ref() {
            fs::read(path).map_err(|_| {
                KrewWasmSDKError::KubeConfigGenericError("cannot read certificate file".to_string())
            })
        } else {
            Err(KrewWasmSDKError::KubeConfigGenericError(
                "cannot determine cluster CA".to_string(),
            ))
        }?;

        let key = if let Some(data) = user.client_key_data.as_ref() {
            base64::decode(data).map_err(|_| {
                KrewWasmSDKError::KubeConfigGenericError(
                    "cannot decode embedded user key".to_string(),
                )
            })
        } else if let Some(path) = user.client_key.as_ref() {
            fs::read(path).map_err(|_| {
                KrewWasmSDKError::KubeConfigGenericError("cannot read user key".to_string())
            })
        } else {
            Err(KrewWasmSDKError::KubeConfigGenericError(
                "cannot determine user key".to_string(),
            ))
        }?;

        let cert = if let Some(data) = user.client_certificate_data.as_ref() {
            base64::decode(data).map_err(|_| {
                KrewWasmSDKError::KubeConfigGenericError(
                    "cannot decode embedded user cert".to_string(),
                )
            })
        } else if let Some(path) = user.client_certificate.as_ref() {
            fs::read(path).map_err(|_| {
                KrewWasmSDKError::KubeConfigGenericError("cannot read user cert".to_string())
            })
        } else {
            Err(KrewWasmSDKError::KubeConfigGenericError(
                "cannot determine user certificate".to_string(),
            ))
        }?;

        Ok(UserIdentity { key, cert, ca })
    }
}

/// Minimum set of information required to access a Kubernetes API server
pub struct Server {
    /// URL of the address, this includes the protocol and the port number
    pub url: String,
    /// CA of the server
    pub ca: Vec<u8>,
}

impl Server {
    /// Create a `Server` instance by parsing the data found inside of the
    /// given kubeconfig
    fn from_cluster(cluster: &kube_conf::cluster::Cluster) -> Result<Self> {
        let ca = if let Some(data) = cluster.certificate_authority_data.as_ref() {
            base64::decode(data).map_err(|_| {
                KrewWasmSDKError::KubeConfigGenericError(
                    "cannot decode embedded server certificate".to_string(),
                )
            })
        } else if let Some(path) = cluster.certificate_authority.as_ref() {
            fs::read(path).map_err(|_| {
                KrewWasmSDKError::KubeConfigGenericError(
                    "cannot read server certificate".to_string(),
                )
            })
        } else {
            Err(KrewWasmSDKError::KubeConfigGenericError(
                "cannot determine cluster CA".to_string(),
            ))
        }?;

        Ok(Server {
            ca,
            url: cluster.server.clone(),
        })
    }
}

impl<'a> From<&'a UserIdentity> for wasi_outbound_http::Identity<'a> {
    fn from(ui: &'a UserIdentity) -> Self {
        wasi_outbound_http::Identity {
            key: &ui.key,
            cert: &ui.cert,
            ca: &ui.ca,
        }
    }
}
