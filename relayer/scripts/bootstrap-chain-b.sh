#!/usr/bin/env bash

nix shell .#osmosis -c \
    cargo run --bin hermes-starknet -- \
        --config test-data/config.toml \
        bootstrap osmosis-chain \
        --chain-id cosmos \
        --chain-store-dir test-data/cosmos \
        --wasm-client-code-path "$(nix build ..#ibc-starknet-cw --print-out-paths)/ibc_client_starknet_cw.wasm" \
        --governance-proposal-authority osmo10d07y265gmmuvt4z0w9aw880jnsr700jjeq4qp