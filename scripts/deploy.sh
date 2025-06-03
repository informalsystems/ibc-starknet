#!/bin/bash
set -euo pipefail

source .env

# use ggrep for macOS, and grep for Linux
case "$OSTYPE" in
    darwin*) GREP="ggrep" ;;
    linux-gnu*) GREP="grep" ;;
    *) echo "Unknown OS: $OSTYPE" && exit 1 ;;
esac

STARKLI_ARGS="--watch"

version() {
    echo "starkli: $(starkli --version)"
    scarb --version 1>&2
}

# build the contract
build() {
    version

    cd "$(dirname "$0")/../cairo-contracts"

    output=$(scarb --profile release build -p starknet_ibc_contracts 1>&2)

    if [[ $output == *"Error"* ]]; then
        echo "Error: $output"
        exit 1
    fi
}

# declare the contract
declare() {
    CONTRACT_SRC=$1

    output=$(
        starkli declare \
        $STARKLI_ARGS \
        "$CONTRACT_SRC" \
        2>&1 | tee /dev/tty
    )

    if [[ $output == *"Error"* ]]; then
        echo "Error: $output"
        exit 1
    fi

    address=$(echo -e "$output" | "$GREP" -oP '0x[0-9a-fA-F]+' | tail -n 1)

    echo "$address"
}

deploy_core() {
    CORE_CLASS_HASH=$1
    PROTOBUF_LIB_CLASS_HASH=$2

    output=$(
        starkli deploy --not-unique \
        $STARKLI_ARGS \
        "$CORE_CLASS_HASH" \
        "$PROTOBUF_LIB_CLASS_HASH" \
        2>&1 | tee /dev/tty
    )

    if [[ $output == *"Error"* ]]; then
        echo "Error: $output"
        exit 1
    fi

    address=$(echo -e "$output" | "$GREP" -oP '0x[0-9a-fA-F]+' | tail -n 1)

    echo "$address"
}

deploy_comet() {
    COMET_CLASS_HASH=$1
    CORE_ADDRESS=$2
    COMET_LIB_CLASS_HASH=$3
    ICS23_LIB_CLASS_HASH=$4
    PROTOBUF_LIB_CLASS_HASH=$5

    output=$(
        starkli deploy --not-unique \
        $STARKLI_ARGS \
        "$COMET_CLASS_HASH" \
        "$CORE_ADDRESS" \
        "$COMET_LIB_CLASS_HASH" \
        "$ICS23_LIB_CLASS_HASH" \
        "$PROTOBUF_LIB_CLASS_HASH" \
        2>&1 | tee /dev/tty
    )

    if [[ $output == *"Error"* ]]; then
        echo "Error: $output"
        exit 1
    fi

    address=$(echo -e "$output" | "$GREP" -oP '0x[0-9a-fA-F]+' | tail -n 1)

    echo "$address"
}

deploy_ics20() {
    ICS20_CLASS_HASH=$1
    CORE_ADDRESS=$2
    ERC20_CLASS_HASH=$3

    output=$(
        starkli deploy --not-unique \
        $STARKLI_ARGS \
        "$ICS20_CLASS_HASH" "$CORE_ADDRESS" "$ERC20_CLASS_HASH" \
        2>&1 | tee /dev/tty
    )

    if [[ $output == *"Error"* ]]; then
        echo "Error: $output"
        exit 1
    fi

    address=$(echo -e "$output" | "$GREP" -oP '0x[0-9a-fA-F]+' | tail -n 1)

    echo "$address"
}

build

if [[ $CORE_CLASS_HASH == "" ]]; then
    core_class_hash=$(declare "$CORE_CONTRACT_SRC")
else
    core_class_hash=$CORE_CLASS_HASH
fi

if [[ $COMET_CLASS_HASH == "" ]]; then
    comet_class_hash=$(declare "$COMET_CONTRACT_SRC")
else
    comet_class_hash=$COMET_CLASS_HASH
fi

if [[ $ERC20_CLASS_HASH == "" ]]; then
    erc20_class_hash=$(declare "$ERC20_CONTRACT_SRC")
else
    erc20_class_hash=$ERC20_CLASS_HASH
fi

if [[ $ICS20_CLASS_HASH == "" ]]; then
    ics20_class_hash=$(declare "$ICS20_CONTRACT_SRC")
else
    ics20_class_hash=$ICS20_CLASS_HASH
fi

if [[ $COMET_LIB_CLASS_HASH == "" ]]; then
    comet_lib_class_hash=$(declare "$COMET_LIB_CONTRACT_SRC")
else
    comet_lib_class_hash=$COMET_LIB_CLASS_HASH
fi

if [[ $ICS23_LIB_CLASS_HASH == "" ]]; then
    ics23_lib_class_hash=$(declare "$ICS23_LIB_CONTRACT_SRC")
else
    ics23_lib_class_hash=$ICS23_LIB_CLASS_HASH
fi

if [[ $PROTOBUF_LIB_CLASS_HASH == "" ]]; then
    protobuf_lib_class_hash=$(declare "$PROTOBUF_LIB_CONTRACT_SRC")
else
    protobuf_lib_class_hash=$PROTOBUF_LIB_CLASS_HASH
fi

echo "Class hashes:"
echo "  CORE: $core_class_hash"
echo "  COMET: $comet_class_hash"
echo "  ICS20: $ics20_class_hash"
echo "  ERC20: $erc20_class_hash"
echo "  COMET LIB: $comet_lib_class_hash"
echo "  ICS23 LIB: $ics23_lib_class_hash"
echo "  PROTOBUF LIB: $protobuf_lib_class_hash"

core_contract_address=$(deploy_core "$core_class_hash" "$protobuf_lib_class_hash")
comet_contract_address=$(deploy_comet "$comet_class_hash" "$core_contract_address" "$comet_lib_class_hash" "$ics23_lib_class_hash" "$protobuf_lib_class_hash")
ics20_contract_address=$(deploy_ics20 "$ics20_class_hash" "$core_contract_address" "$erc20_class_hash")

echo "Contract addresses:"
echo "  CORE: $core_contract_address"
echo "  COMET: $comet_contract_address"
echo "  ICS20: $ics20_contract_address"

ICS20_CONTRACT_ADDRESS=$ics20_contract_address

register_client() {
    CORE_ADDRESS=$1
    COMET_ADDRESS=$2

    # client type uses cairo short-string
    COMET_CLIENT_TYPE=$(starkli to-cairo-string "07-tendermint")

    CALLDATA="$COMET_CLIENT_TYPE $COMET_ADDRESS"

    starkli invoke \
        $STARKLI_ARGS \
        "$CORE_ADDRESS" \
        "register_client" \
        $CALLDATA

    echo "Client type \`07-tendermint\` is registered with address $COMET_ADDRESS"
}

register_ics20() {
    CORE_ADDRESS=$1
    ICS20_ADDRESS=$2

    TRANSFER_PORT=$(starkli to-cairo-string "transfer")

    # https://docs.starknet.io/architecture-and-concepts/smart-contracts/serialization-of-cairo-types/#serialization_of_byte_arrays
    # there is 0 previous bytes, and `transfer` has 8 bytes
    CALLDATA="0x0 $TRANSFER_PORT 0x8 $ICS20_ADDRESS"

    starkli invoke \
        $STARKLI_ARGS \
        "$CORE_ADDRESS" \
        "bind_port_id" \
        $CALLDATA

    echo "Channel port \`transfer\` is bound with address $ICS20_ADDRESS"
}

register_client "$core_contract_address" "$comet_contract_address"
register_ics20 "$core_contract_address" "$ics20_contract_address"
