#!/usr/bin/env bash

cargo run --bin hermes-starknet -- \
    --config test-data/config.toml \
    create starknet-client \
    --target-chain-id cosmos \
    --counterparty-chain-id 393402133025997798000961 \
    --wasm-code-hash "$(sha256sum $(nix build ..#ibc-starknet-cw --print-out-paths)/ibc_client_starknet_cw.wasm | cut -d ' ' -f 1)"