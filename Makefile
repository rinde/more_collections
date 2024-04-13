RUST_NIGHTLY_VERSION  := $(shell cat rust-toolchain-nightly)
SHELL := /bin/bash -eu

.PHONY: test
test:
	cargo test --workspace --all-targets --all-features
	cargo test --workspace --doc

lint: 
	cargo fmt -- --check
	cargo clippy --workspace --all-features --tests --benches -- -D warnings
	cargo doc --all --no-deps --document-private-items --all-features

fmt:
	cargo +$(RUST_NIGHTLY_VERSION) fmt -- --config-path ./rustfmt-nightly.toml

install-nightly:
	rustup toolchain install $(RUST_NIGHTLY_VERSION)

.PHONY: clean
clean:
	rm -rf target

.PHONY: build
build:
	cargo build --workspace --all-features

.PHONY: build-release
build-release:
	cargo build --release --workspace --all-features

bumpdeps:
	cargo install cargo-edit
	cargo upgrade
