#!/usr/bin/env bash

nix shell .#starknet-devnet -c \
    cargo run --bin hermes-starknet -- \
        --config test-data/config.toml \
        bootstrap starknet-chain \
        --chain-id starknet \
        --chain-store-dir test-data/starknet