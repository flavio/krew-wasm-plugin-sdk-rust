//! This is an experimental crate that simplifies the creation of
//! kubectl plugins written using WebAssembly and WASI.
//! These plugins are then ran by [krew-wasm](https://github.com/flavio/krew-wasm).
//!
//! The purpose of this crate is to provide helper functions and structs to
//! perform the following operations:
//! * Determine the Kubernetes API server to connect to
//! * Determine the user identity to be used when interacting with Kubernetes
//! * Simplify the process of performing HTTP requests using the k8s-openapi
//!   crate and the experimental WASI outboung HTTP crate.

pub mod errors;
pub mod kube_config;
pub mod wasi_outbound_http_helper_k8s;

// re-exports
pub use kube_conf;

wit_bindgen_rust::import!("wit/ephemeral/wasi-outbound-http.wit");

impl std::fmt::Display for wasi_outbound_http::HttpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match *self {
            wasi_outbound_http::HttpError::Success => "success",
            wasi_outbound_http::HttpError::DestinationNotAllowed => "destination not allowed",
            wasi_outbound_http::HttpError::InvalidCfg => "invalid config",
            wasi_outbound_http::HttpError::InvalidUrl => "invalid URL",
            wasi_outbound_http::HttpError::RequestError => "request error",
            wasi_outbound_http::HttpError::RuntimeError => "runtime error",
        };
        write!(f, "{}", msg)
    }
}

impl std::error::Error for wasi_outbound_http::HttpError {}
