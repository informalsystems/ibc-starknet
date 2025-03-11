#!/usr/bin/env bash

cargo run --bin hermes-starknet -- \
    --config test-data/config.toml \
    create starknet-client \
    --target-chain-id cosmos \
    --counterparty-chain-id 393402133025997798000961 \
    --wasm-code-hash ccb2041e457ac14e8a1bc1ac330c70a7b5ba958c5535ecb7955e203a08740b20