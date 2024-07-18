# Run formatter and clippy for all the cargo and scarb packages
lint:
  @cd crates && \
  cargo +nightly fmt --all -- --check && \
  cargo clippy --all-targets --all-features -- -D warnings && \
  cd ../contracts && scarb fmt