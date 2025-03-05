<div align="center">
    <h1>Starknet IBC</h1>
</div>

This repository contains the IBC implementation for Starknet.

- IBC protocol implemented in Cairo. [./cairo-contracts](./cairo-contracts)
  - [Core](./cairo-contracts/packages/core)
  - [ICS20 fungible token transfer](./cairo-contracts/packages/apps/src/transfer)
  - [Light Clients](./cairo-contracts/packages/clients/src/cometbft)
- Starknet Wasm Light Client. [./light-client](./light-client)
- Starknet IBC Relayer. [./relayer](./relayer)
- General Cairo libraries. [./cairo-libs](./cairo-libs)

## Getting Started

> [!IMPORTANT]
> Our IBC implementation is not deployed on mainnets. Currently it is only
> deployed on Starknet testnet (`SN_SEPOLIA`) and Osmosis testnet
> (`osmo-test-5`).

If you're a Starknet or Cosmos user, you don't need this repository for using
IBC between Starknet <> Cosmos.

Nonetheless, the following give brief instructions to send tokens between
Starknet and Cosmos.

> [!CAUTION]
> Make sure that there is a relayer running between Starknet and Cosmos for the
> corresponding channel.

### Prerequisites

- `osmosisd`
- `starkli`
- `cargo` (to run Rust binaries from this project)

### From Osmosis to Starknet via `channel-10185`

```bash
osmosisd tx ibc-transfer transfer \
    transfer channel-10185 \
    "$STARKNET_ADDRESS" "${AMOUNT}${BASE_DENOM}" \
    --from "$OSMOSIS_ADDRESS"
```

On block explorer ([Mintscan](https://www.mintscan.io/osmosis-testnet)), you'll
soon receive a `packet_acknowledgement`.

### From Starknet to Osmosis via `channel-0`

Approve ICS20 contract to spend on your behalf

```bash
starkli invoke "$ERC20_TOKEN_ON_STARKNET" \
    approve \
    "$IBC_ICS20_CONTRACT" "u256:$AMOUNT" \
    --account "$STARKNET_ACCOUNT" \
    --strk --watch
```

> [!TIP]
> If you are sending a Cosmos token back to Osmosis, you need to know its ERC20
> token address on Starknet. You can check the Starknet block explorer to find
> it.

Create felt arguments to send token with timeout of 600 seconds.

```bash
SN_TRANSFER_ARGS=$(cargo run -q -p hermes-starknet-tools-cli \
    starknet transfer-args \
    --amount "$AMOUNT" --denom "$ERC20_TOKEN_ON_STARKNET" \
    --receiver "$OSMOSIS_ADDRESS" \
    --channel-id channel-0 \
    --timeout-timestamp "$((`date +%s` + 600))" \
    | cut -d: -f9)
```

> [!IMPORTANT]
> If you're sending a Cosmos token back, you have to pass the ibc prefixed denom
> for `--denom`. e.g. `--denom "transfer/channel-0/uosmo"`

Submit the transaction on Starknet

```bash
starkli invoke $IBC_ICS20_CONTRACT\
    send_transfer\
    $SN_TRANSFER_ARGS \
    --account "$STARKNET_ACCOUNT"\
--strk --watch
```

On block explorer ([Starkscan](https://sepolia.starkscan.co)), you'll soon
receive a `packet_acknowledgement`.

## Operator Setup

### Configure the Relayer

```bash
cp relayer.toml.example relayer.toml
```

In `relayer.toml`, update the values:

- `starknet_chain_config.contract_classes` with class hashes of the declared
  contracts.
- `starknet_chain_config.contract_addresses` with contract addresses of the
  deployed contracts.
- `relayer_wallet` for Starknet relayer wallet.
- `key_store_folder` and `key_name` for the Cosmos account wallet.

> [!CAUTION]
> We are currently using the permissioned-wallet-setup for the relayer. The same
> (Starknet) wallet that deployed the contracts should be used for the relayer.

For Starknet wallet, you'll need a private key, public key and an account
address.

- Use `starkli signer gen-keypair` to generate a key pair.
- Use `starkli account` to deploy an account.

For Osmosis wallet, you'll need to setup via `hermes-v1`.

- Use `hermes keys add` to generate a new hermes-v1 keystore json.

> [!TIP]
> Use https://starknet-faucet.vercel.app and https://faucet.testnet.osmosis.zone
> to fund your Starknet or Osmosis account on testnet.

### Running the Relayer

```bash
cd relayer
RUST_LOG=trace RUST_BACKTRACE=1 cargo run --bin hermes-starknet -- -c ../relayer.toml \
    start cosmos-with-starknet \
    --chain-id-a 393402133025997798000961 --client-id-a 07-tendermint-3 \
    --chain-id-b osmo-test-5 --client-id-b 08-wasm-4459
```

The pending packets in the past blocks can be cleared with `--clear-past-blocks`
flag.

```bash
RUST_LOG=trace RUST_BACKTRACE=1 cargo run --bin hermes-starknet -- -c ../relayer.toml \
    start cosmos-with-starknet \
    --chain-id-a 393402133025997798000961 --client-id-a 07-tendermint-3 \
    --chain-id-b osmo-test-5 --client-id-b 08-wasm-4459 \
    --clear-past-blocks 10m # clear packets in past blocks produced in last 10 minutes
```

> [!IMPORTANT]
> Since we are using permissioned-wallet-setup for the relayer, we have to use
> the same wallet for all relayer instances. So avoid running parallel relayer
> instances as this will cause nonce issues. So when clearing pending packet,
> close the main relayer instance first.

## Developer Setup

### Deploying the contracts

Copy the `.env.example` to `.env`.

```bash
cp .env.example .env
```

In `.env` file, update the values for `STARKNET_*` variables appropriately.

Then, call `./scripts/deploy.sh` to deploy the contracts.

> [!IMPORTANT]
> Make sure `.env` file is updated with the correct values.

```bash
./scripts/deploy.sh
```

> [!CAUTION]
> Note down the declared class hashes and deployed contract addresses and share
> with the relayer operator.

### Creating the IBC Clients, Connections and Channels

```bash
cd relayer
cargo run --example bootstrap_ibc -- ../relayer.toml
```

> [!IMPORTANT]
> Make sure `relayer.toml` file is updated with the correct values.

> [!CAUTION]
> Note down the client, connection and channel IDs and share with the relayer
> operator.
