#!/bin/bash
set -euo pipefail

source ./scripts/deploy.sh

# invoke the contract
invoke() {
    if [[ $CONTRACT_ADDRESS == "" ]]; then
        address=$(deploy)
    else
        address=$CONTRACT_ADDRESS
    fi

    output=$(
        starkli invoke $address ibc_token_address 0 \
        --rpc $RPC_URL \
        --compiler-version $COMPILER_VERSION \
        --account $ACCOUNT_SRC \
        --keystore $KEYSTORE_SRC \
        --keystore-password $KEYSTORE_PASS \
        2>&1 | tee /dev/tty
    )

    if [[ $output == *"Error"* ]]; then
        echo "Error: $output"
        exit 1
    fi

    echo $output
}

invoke
