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
        starkli invoke $address register_token 1 0x4e91934ce777f807d6bc90fd3b06e1fa49e942ab1fb70a072ca1ad61dc2998d \
        --rpc $RPC_URL \
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
