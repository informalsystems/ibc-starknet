#!/usr/bin/env bash

cargo run --bin hermes-starknet -- \
    --config test-data/config.toml \
    create client \
    --target-chain-id 393402133025997798000961 \
    --counterparty-chain-id cosmos