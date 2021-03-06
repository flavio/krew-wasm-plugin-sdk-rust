//! Helper functions required to perform HTTP requests against the Kubernetes
//! API server

use k8s_openapi::http;
use std::convert::TryFrom;

use super::wasi_outbound_http;
use crate::errors::{KrewWasmSDKError, Result};
use crate::kube_config::ConnectionConfig;

impl TryFrom<&http::Method> for wasi_outbound_http::Method {
    type Error = KrewWasmSDKError;

    fn try_from(method: &http::Method) -> std::result::Result<Self, Self::Error> {
        match *method {
            http::Method::GET => Ok(wasi_outbound_http::Method::Get),
            http::Method::POST => Ok(wasi_outbound_http::Method::Post),
            http::Method::PUT => Ok(wasi_outbound_http::Method::Put),
            http::Method::DELETE => Ok(wasi_outbound_http::Method::Delete),
            http::Method::PATCH => Ok(wasi_outbound_http::Method::Patch),
            http::Method::HEAD => Ok(wasi_outbound_http::Method::Head),
            http::Method::OPTIONS => Ok(wasi_outbound_http::Method::Options),
            _ => Err(KrewWasmSDKError::KubeWasiHttpMethodConversionError()),
        }
    }
}

/// Perform HTTP request using the given connection configuration object and
/// the given request configuration
///
/// The `http:Request` objects created by the `k8s-openapi` crate do not have
/// a target host specified. This function takes care of that by looking at the
/// Kubernetes settings provided by the [`ConnectionConfig`] object.
pub fn make_request(
    k8s_req: http::Request<Vec<u8>>,
    connection_config: &ConnectionConfig,
    req_cfg_id: &str,
) -> Result<wasi_outbound_http::Response> {
    let method: wasi_outbound_http::Method = k8s_req.method().try_into()?;
    let body: Option<&[u8]> = Some(k8s_req.body().as_slice());
    let headers: Vec<(&str, &str)> = k8s_req
        .headers()
        .iter()
        .filter_map(|(k, v)| match v.to_str() {
            Ok(vs) => Some((k.as_str(), vs)),
            Err(_) => None,
        })
        .collect();
    let params: Vec<(&str, &str)> = match k8s_req.uri().query() {
        None => Vec::new(),
        Some(query) => query
            .split('&')
            .filter_map(|q| {
                let tmp: Vec<&str> = q.splitn(2, '=').collect();
                if tmp.len() == 2 {
                    Some((tmp[0], tmp[1]))
                } else {
                    None
                }
            })
            .collect(),
    };

    let server_url = connection_config.server.url.parse::<http::uri::Uri>()?;

    let uri = http::uri::Builder::new()
        .scheme(server_url.scheme_str().unwrap_or("https"))
        .authority(server_url.authority().unwrap().as_str())
        .path_and_query(k8s_req.uri().path_and_query().unwrap().as_str())
        .build()?
        .to_string();

    let req = wasi_outbound_http::Request {
        method,
        uri: &uri,
        headers: &headers,
        params: &params,
        body,
    };

    let response = wasi_outbound_http::request(req, Some(req_cfg_id))?;
    Ok(response)
}
