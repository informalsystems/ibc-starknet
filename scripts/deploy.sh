#!/bin/bash
set -euo pipefail

source .env

# use ggrep for macOS, and grep for Linux
case "$OSTYPE" in
    darwin*) GREP="ggrep" ;;
    linux-gnu*) GREP="grep" ;;
    *) echo "Unknown OS: $OSTYPE" && exit 1 ;;
esac

version() {
    starkli --version 1>&2
    scarb --version 1>&2
}

# build the contract
build() {
    version

    cd "$(dirname "$0")/../contracts"

    output=$(scarb build 2>&1)

    if [[ $output == *"Error"* ]]; then
        echo "Error: $output"
        exit 1
    fi
}

# declare the contract
declare() {
    build

    output=$(
        starkli declare --watch $CONTRACT_SRC \
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

    address=$(echo -e "$output" | "$GREP" -oP '0x[0-9a-fA-F]+' | tail -n 1)

    echo $address
}

# deploy the contract
deploy() {
    if [[ $CLASS_HASH == "" ]]; then
        class_hash=$(declare)
    else
        class_hash=$CLASS_HASH
    fi

    output=$(
        starkli deploy --not-unique \
        --watch $class_hash \
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

    address=$(echo -e "$output" | "$GREP" -oP '0x[0-9a-fA-F]+' | tail -n 1)

    echo $address
}

deploy
