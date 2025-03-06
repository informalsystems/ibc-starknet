#!/usr/bin/env bash

nix shell .#wasm-simapp -c \
    cargo run --bin hermes-starknet -- \
        --config test-data/config.toml \
        bootstrap cosmos-chain \
        --chain-id cosmos \
        --chain-store-dir test-data/cosmos \
        --chain-command-path simd