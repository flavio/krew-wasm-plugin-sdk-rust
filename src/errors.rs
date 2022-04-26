//! Define the errors raised by this crate

use super::wasi_outbound_http;
use k8s_openapi::http;
use thiserror::Error;

/// Custom error types
#[derive(Error, Debug)]
pub enum KrewWasmSDKError {
    #[error("kube-conf error")]
    KubeConfigError(#[from] kube_conf::errors::Error),
    #[error("kube-conf: no default kubernetes context")]
    KubeConfigNoContextError(),
    #[error("kube-conf: no cluster definition")]
    KubeConfigNoClusterDefinitionError(),
    #[error("kube-conf: no user definition")]
    KubeConfigNoUserDefinitionError(),
    #[error("kube-conf error: {0}")]
    KubeConfigGenericError(String),

    #[error("wasi-http error")]
    KubeWasiHttpError(#[from] wasi_outbound_http::HttpError),
    #[error("http method not handled by wasi_outbound_http")]
    KubeWasiHttpMethodConversionError(),
    #[error("http error")]
    HttpError(#[from] http::Error),
    #[error("http invalid URI")]
    HttpInvalidUriError(#[from] http::uri::InvalidUri),
    #[error("invalid header (expected {expected:?}, found {found:?})")]
    InvalidHeader { expected: String, found: String },
    #[error("unknown data store error")]
    Unknown,
}

/// An alias to quickly define blocks of code that return a std `Result` that
/// uses a [`KrewWasmSDKError`] as Error type
pub type Result<T> = std::result::Result<T, KrewWasmSDKError>;
