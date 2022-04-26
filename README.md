Status | Docs | License
 ----------------------|------|---------
 ![experimental](https://img.shields.io/badge/status-experimental-orange) | [![Docs](https://img.shields.io/badge/docs-%20-blue)](https://flavio.github.io/krew-wasm-plugin-sdk-rust/krew_wasm_plugin_sdk) |  [![License: Apache 2.0](https://img.shields.io/badge/License-Apache2.0-brightgreen.svg)](https://opensource.org/licenses/Apache-2.0)

Experimental Rust SDK for writing [krew-wasm](https://github.com/flavio/krew-wasm) plugins.

The purpose of this crate is to provide helper functions and structs to perform the following operations:

* Determine the Kubernetes API server to connect to
* Determine the user identity to be used when interacting with Kubernetes
* Simplify the process of performing HTTP requests using the k8s-openapi crate and the experimental WASI outboung HTTP crate.

