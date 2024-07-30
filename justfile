# Installs development tools including nightly rustfmt, taplo-cli, etc.
install-tools:
	rustup component add rustfmt --toolchain nightly
	cargo install typos-cli taplo-cli

build-cw-contract:
  cd ./light-client/cw-contract && cargo build --target wasm32-unknown-unknown

# Runs formatter and clippy for all the cargo and scarb packages
lint:
  @cargo +nightly fmt --all -- --check && \
  cargo clippy --all-targets --all-features -- -D warnings && \
  cargo clippy --all-targets --no-default-features -- -D warnings && \
  taplo fmt --check && \
  cd ./contracts && scarb fmt

# Builds the Cairo contracts
build-contracts:
  cd ./contracts && scarb build

# Tests the Cairo contracts
test-contracts:
  cd ./contracts && scarb test