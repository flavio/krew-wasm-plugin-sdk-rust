.PHONY: check
check:
	cargo check --features enable-k8s-openapi-version

.PHONY: build
build:
	cargo build --features enable-k8s-openapi-version

.PHONY: doc
doc:
	cargo doc --features enable-k8s-openapi-version

.PHONY: fmt
fmt:
	cargo fmt --all -- --check

.PHONY: lint
lint:
	cargo clippy --features enable-k8s-openapi-version -- -D warnings

.PHONY: test
test: fmt lint doc
	cargo test --workspace --features enable-k8s-openapi-version

.PHONY: clean
clean:
	cargo clean
