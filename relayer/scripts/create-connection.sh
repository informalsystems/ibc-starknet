#!/usr/bin/env bash

cargo run --bin hermes-starknet -- \
    --config test-data/config.toml \
    create connection \
    --target-chain-id 393402133025997798000961 \
    --counterparty-chain-id cosmos \
    --target-client-id 07-tendermint-0 \
    --counterparty-client-id 08-wasm-0