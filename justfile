# Installs development tools including nightly rustfmt, taplo-cli, etc.
install-tools:
	rustup component add rustfmt --toolchain nightly
  rustup target add wasm32-unknown-unknown
	cargo install typos-cli taplo-cli

build-cw-contract:
  cd ./light-client && cargo build -p ibc-client-starknet-cw --target wasm32-unknown-unknown

# Builds the Cairo contracts
build-cairo-contracts:
  cd ./cairo-contracts && \
  scarb build -p starknet_ibc_contracts

# Tests the Cairo contracts
test-cairo-contracts:
  cd ./cairo-contracts && \
  snforge test --workspace

lint: lint-toml lint-light-client lint-cairo lint-nix lint-relayer

lint-toml:
  taplo fmt --check

# Runs formatter and clippy for all the cargo and scarb packages
lint-light-client:
  cd ./light-client && \
  cargo fmt --all -- --check && \
  cargo clippy --all-targets --all-features -- -D warnings && \
  cargo clippy --all-targets --no-default-features -- -D warnings

lint-relayer:
  cd ./relayer && \
  cargo fmt --all -- --check && \
  cargo clippy --all-targets --all-features -- -D warnings && \
  cargo clippy --all-targets --no-default-features -- -D warnings

# Runs formatter and clippy for all the cargo and scarb packages
lint-cairo:
  cd ./cairo-contracts && \
  scarb fmt --check

lint-nix:
  cd ./relayer && \
  nixfmt --check .