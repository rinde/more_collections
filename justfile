set quiet

rust_version         := `sed -nr 's/channel = "(.*)"/\1/p' rust-toolchain.toml`
rust_nightly_version := `sed -nr 's/channel = "(.*)"/\1/p' rust-toolchain-nightly.toml`

@default: fmt lint test

rust-version:
    echo '{{rust_version}}'

rust-nightly-version:
    echo '{{rust_nightly_version}}'

fmt:
    cargo '+{{rust_nightly_version}}' fmt --all

lint strict="":
    cargo '+{{rust_nightly_version}}' fmt -- --check
    cargo clippy \
        --workspace \
        --tests \
        --benches \
        --all-targets \
        --all-features \
        --quiet \
        -- {{ if strict != "" { "-D warnings" } else { "" } }}
    cargo doc --all --no-deps --document-private-items --all-features --quiet

test:
    cargo test --workspace --all-targets --all-features
    cargo test --workspace --doc

install-nightly:
    rustup toolchain install '+{{rust_nightly_version}}'

bumpdeps:
    cargo install cargo-edit
    cargo upgrade