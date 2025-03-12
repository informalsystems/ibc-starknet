#!/usr/bin/env bash

nix shell .#starknet-devnet -c \
    cargo run --bin hermes-starknet -- \
        --config test-data/config.toml \
        bootstrap starknet-chain \
        --chain-id starknet \
        --chain-store-dir test-data/starknet \
        --erc20-contract-path ../cairo-contracts/target/dev/starknet_ibc_contracts_ERC20Mintable.contract_class.json \
        --ics20-contract-path ../cairo-contracts/target/dev/starknet_ibc_contracts_TransferApp.contract_class.json \
        --comet-client-contract-path ../cairo-contracts/target/dev/starknet_ibc_contracts_CometClient.contract_class.json \
        --ibc-core-contract-path ../cairo-contracts/target/dev/starknet_ibc_contracts_IBCCore.contract_class.json