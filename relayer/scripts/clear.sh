#!/usr/bin/env bash

cargo run --bin hermes-starknet -- \
    --config test-data/config.toml \
    start starknet-with-cosmos \
    --chain-id-a 393402133025997798000961 \
    --chain-id-b cosmos \
    --client-id-a 07-tendermint-0 \
    --client-id-b 08-wasm-0 \
    --clear-past-blocks 60s \
    --stop-after-blocks 0s