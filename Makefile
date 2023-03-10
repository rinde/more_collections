RUST_NIGHTLY_VERSION  := nightly-2023-03-10
SHELL := /bin/bash -eu

.PHONY: test
test:
	cargo test --workspace --all-targets --all-features
	cargo test --workspace --doc

lint: 
	cargo fmt -- --check
	cargo clippy --workspace --all-features --tests --benches -- -D clippy::style -D clippy::perf -D warnings
	cargo doc --all --no-deps --document-private-items --all-features

fmt:
	cargo +$(RUST_NIGHTLY_VERSION) fmt -- --config-path ./rustfmt-nightly.toml

.PHONY: clean
clean:
	rm -rf target

.PHONY: build
build:
	cargo build

.PHONY: build-release
build-release:
	cargo build --release
