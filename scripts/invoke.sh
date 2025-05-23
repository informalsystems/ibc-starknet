#!/bin/bash
set -euo pipefail

source ./scripts/deploy.sh

# invoke the contract
invoke() {
    output=$(
        starkli invoke "$ICS20_CONTRACT_ADDRESS" ibc_token_address 0 \
        --rpc "$RPC_URL" \
        --account "$ACCOUNT_SRC" \
        --keystore "$KEYSTORE_SRC" \
        --keystore-password "$KEYSTORE_PASS" \
        2>&1 | tee /dev/tty
    )

    if [[ $output == *"Error"* ]]; then
        echo "Error: $output"
        exit 1
    fi

    echo "$output"
}

invoke
