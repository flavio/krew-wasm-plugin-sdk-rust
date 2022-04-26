Experimental Rust SDK for writing [krew-wasm](https://github.com/flavio/krew-wasm) plugins.

The purpose of this crate is to provide helper functions and structs to perform the following operations:

* Determine the Kubernetes API server to connect to
* Determine the user identity to be used when interacting with Kubernetes
* Simplify the process of performing HTTP requests using the k8s-openapi crate and the experimental WASI outboung HTTP crate.

