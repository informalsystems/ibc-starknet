#!/usr/bin/env bash

cargo run --bin hermes-starknet -- \
    --config test-data/config.toml \
    create cosmos-client \
    --target-chain-id 393402133025997798000961 \
    --counterparty-chain-id cosmos