#!/bin/bash
set -euo pipefail

source ./scripts/deploy.sh

# invoke the contract
invoke() {
    if [[ $CONTRACT_ADDRESS == "" ]]; then
        if [[ $ERC20_CLASS_HASH == "" ]]; then
            erc20_class_hash=$(declare $ERC20_CONTRACT_SRC)
        else
            erc20_class_hash=$ERC20_CLASS_HASH
        fi
        if [[ $ICS20_CLASS_HASH == "" ]]; then
            ics20_class_hash=$(declare $ICS20_CONTRACT_SRC)
        else
            ics20_class_hash=$ICS20_CLASS_HASH
        fi
        address=$(deploy $ics20_class_hash $erc20_class_hash)
    else
        address=$CONTRACT_ADDRESS
    fi

    output=$(
        starkli invoke $address ibc_token_address 0 \
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
