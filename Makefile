SHELL := /bin/bash -eu

.PHONY: test
test:
	cargo nextest run --all-features
	cargo test --workspace --doc

lint: 
	cargo fmt -- --check
	cargo clippy --workspace --all-features --tests --benches -- -D clippy::style -D clippy::perf -D warnings
	cargo doc --all --no-deps --document-private-items

fmt:
	cargo +nightly fmt -- --config-path ./rustfmt-nightly.toml

.PHONY: clean
clean:
	rm -rf target

.PHONY: build
build:
	cargo build

.PHONY: build-release
build-release:
	cargo build --release
