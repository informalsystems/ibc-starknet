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
    echo "starkli: $(starkli --version)"
    scarb --version 1>&2
}

# build the contract
build() {
    version

    cd "$(dirname "$0")/../cairo-contracts"

    output=$(scarb build -p starknet_ibc_contracts 1>&2)

    if [[ $output == *"Error"* ]]; then
        echo "Error: $output"
        exit 1
    fi
}

# declare the contract
declare() {
    CONTRACT_SRC=$1

    output=$(
        starkli declare --watch $CONTRACT_SRC \
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

    address=$(echo -e "$output" | "$GREP" -oP '0x[0-9a-fA-F]+' | tail -n 1)

    echo $address
}

# deploy the contract
deploy() {
    ICS20_CLASS_HASH=$1
    ERC20_CLASS_HASH=$2

    output=$(
        starkli deploy --not-unique \
        --watch $ICS20_CLASS_HASH '1' $ERC20_CLASS_HASH \
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

build

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

deploy $ics20_class_hash $erc20_class_hash
