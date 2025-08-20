# ICS20: IBC Fungible Tokens

## Prerequisites

- An IBC channel between Starknet <> Osmosis.
- A relayer that is listening and relaying on that IBC channel.

## ICS20 on Starknet <> Osmosis

For Osmosis, the ICS20 APIs are already handled by many wallets like
[Keplr][keplr] and [Cosmostation][cosmostation]. In this documentation, we focus
on the APIs exposed by the Starknet IBC implementation.

To initiate token transfers, the wallet application needs to communicate with
the ICS20 contract deployed on Starknet.

A short recap of ICS20 contact on Starknet:

- sending native tokens (e.g. STRK, ETH) will escrow the tokens to the ICS20
  contract.
- receiving remote IBC tokens (e.g. OSMO, ATOM) will mint the tokens by the
  ICS20 contract.
- receiving native tokens (e.g. STRK, ETH) will unescrow the tokens from the
  ICS20 contract.
- sending remote IBC tokens (e.g. OSMO, ATOM) will burn the tokens by the ICS20
  contract.

### Sending fungible tokens

Main invoke function is [`send_transfer`][send-transfer] with a single argument
[`MsgTransfer`][msg-transfer].

```cairo
pub struct MsgTransfer {
    pub port_id_on_a: PortId,
    pub chan_id_on_a: ChannelId,
    pub denom: PrefixedDenom,
    pub amount: u256,
    pub receiver: ByteArray,
    pub memo: Memo,
    pub timeout_height_on_b: Height,
    pub timeout_timestamp_on_b: Timestamp,
}
```

- `port_id_on_a` and `chan_id_on_a` will be socially communicated and should be
  fixed for Starknet <> Osmosis transfers.
- `PrefixedDenom` will be used to specify the token denomination depending on
  native or remote.
- `amount` is just a u256 representing the token amount to be transferred.
- `receiver` is the address of the account that will receive the tokens at
  Osmosis.
- `memo` is to include any additional information with the transfer.
- `timeout_height_on_b` is the timeout height at Osmosis after which the packet
  won't be included.
- `timeout_timestamp_on_b` is the timeout UNIX timestamp in nanoseconds at
  Osmosis block after which the packet won't be included.

> [!NOTE]
> Before transferring the tokens, the wallet application needs to ensure the
> user has approved the ICS20 application to spend on their behalf.

We maintain a [typescript code][starknet-ibc-transfer] to showcase submitting
`send_transfer` message. One can use the script after bootstrapping the Starknet
and Osmosis with packet relaying.

```sh
cd cairo-contracts
scarb --profile release build -p starknet_ibc_contracts
cd ../relayer

export ERC20_CONTRACT="$(pwd)/../cairo-contracts/target/release/starknet_ibc_contracts_ERC20Mintable.contract_class.json"
export ICS20_CONTRACT="$(pwd)/../cairo-contracts/target/release/starknet_ibc_contracts_TransferApp.contract_class.json"
export COMET_CLIENT_CONTRACT="$(pwd)/../cairo-contracts/target/release/starknet_ibc_contracts_CometClient.contract_class.json"
export IBC_CORE_CONTRACT="$(pwd)/../cairo-contracts/target/release/starknet_ibc_contracts_IBCCore.contract_class.json"
export STARKNET_BIN="$HOME/.cargo/bin/madara"

export COMET_LIB_CONTRACT="$(pwd)/../cairo-contracts/target/release/starknet_ibc_contracts_CometLib.contract_class.json"
export ICS23_LIB_CONTRACT="$(pwd)/../cairo-contracts/target/release/starknet_ibc_contracts_Ics23Lib.contract_class.json"
export PROTOBUF_LIB_CONTRACT="$(pwd)/../cairo-contracts/target/release/starknet_ibc_contracts_ProtobufLib.contract_class.json"

export RAW_STORAGE_CONTRACT="$(pwd)/../madara-contracts/target/release/madara_contracts_RawStore.contract_class.json"

export STARKNET_WASM_CLIENT_PATH="$(nix build ..#ibc-starknet-cw --print-out-paths)/ibc_client_starknet_cw.wasm"

cargo run --bin hermes-devnet
```

Once the devnet is bootstrapped, one can use the logged information to execute
the script.

### Receiving fungible tokens

Given a Cosmos wallet triggers a transfer from an Osmosis wallet to a Starknet
wallet, there is nothing to be done on Starknet side to receive the tokens.

But a Starknet wallet may want to listen to the events from the ICS20 contract:
[`ICS20Events`][ics20-events]

```cairo
pub enum Event {
    SendEvent: SendEvent,
    RecvEvent: RecvEvent,
    AckEvent: AckEvent,
    AckStatusEvent: AckStatusEvent,
    TimeoutEvent: TimeoutEvent,
    CreateTokenEvent: CreateTokenEvent,
}
```

- `SendEvent` when a token transfer is initiated from Starknet to Osmosis.
- `RecvEvent` when a token transfer is received from Osmosis to Starknet.
- `AckEvent` when a token transfer is acknowledged by the Osmosis.
- `AckStatusEvent` is emitted along with `AckEvent` which contains the success
  status.
- `TimeoutEvent` when a token transfer times out i.e. not included in Osmosis.
- `CreateTokenEvent` when a new token is created and minted in the ICS20
  contract.

These events provide more context on the IBC transfer information.

### IBC Token Representation

#### Osmosis Token on Starknet

The remote (Osmosis) IBC tokens are created and minted by ICS20 contract as
ERC20 tokens.

While creating an ERC20 tokens, we set its metadata as following:

- name: the base denom from IBC prefixed denom.
- symbol: name with `IBC/` prefix.
- decimals: 0 (zero).

We use 0 decimals because IBC-go conventionally uses base denom.

So, when sending `1 OSMO` or `1e6 uOSMO` (in base denom) from Osmosis, the ICS20
contract mints `1e6 IBC/uOSMO`.

#### Starknet Token on Osmosis

The Starknet IBC tokens are represented using base denom on Osmosis with 0
decimals. The token address is used as denom.

So sending a `1 STRK` from Starknet will mint
`1e18 0x04718f5a0fc34cc1af16a1cdee98ffb20c31f5cd61d6ab07201858f4287c938d` at
Osmosis.

#### Manual Token Registration

The remote (Osmosis) token contracts are not deployed on Starknet unless there
is a transfer from Osmosis.

For UX purposes, we allow users to manually register and deploy these IBC tokens
(with zero token supply) on Starknet before initiating a transfer from Osmosis.

The IBC token registration and deployment are permissionless. One can invoke
[`create_ibc_token`][create-ibc-token] entrypoint of ICS20 contract.

The argument is an IBC `PrefixedDenom` at Starknet. So, to register `uOSMO` IBC
token arriving at channel `channel-0` and port `transfer`, the argument would be
`transfer/channel-0/uOSMO`.

```ts
const ibc_prefixed_denom = {
  trace_path: [{
    port_id: { port_id: "transfer" },
    channel_id: { channel_id: "channel-0" },
  }],
  base: new CairoCustomEnum({
    Native: undefined,
    Hosted: "uOSMO",
  }),
};
```

Note that, the IBC prefixed denom prepends the port and channel. Suppose, the
`uATOM` (native to Cosmohub) is represented as `transfer/channel-13/uATOM` on
Osmosis. Its prefixed denom representation on Starknet would be
`transfer/channel-0/transfer/channel-13/uATOM`. As `PrefixedDenom`, it would be:

```rs
const ibc_prefixed_denom = {
  trace_path: [{
    port_id: { port_id: "transfer" },
    channel_id: { channel_id: "channel-13" },
  },
  {
    port_id: { port_id: "transfer" },
    channel_id: { channel_id: "channel-0" },
  }],
  base: new CairoCustomEnum({
    Native: undefined,
    Hosted: "uOSMO",
  }),
};
```

## References:

- [how does IBC Protocol work?][how-ibc-classic-works]
- [ICS20 Specification][ics20-spec]
- [Unerstand IBC denoms][ibc-denoms]

[keplr]: https://www.keplr.app
[cosmostation]: https://www.cosmostation.io/products/cosmostation_mobile
[send-transfer]: ../cairo-contracts/packages/apps/src/transfer/components/transfer.cairo#L150
[msg-transfer]: ../cairo-contracts/packages/apps/src/transfer/types.cairo#L17
[starknet-ibc-transfer]: ../scripts/starknet_ibc_transfer.ts
[ics20-events]: ../cairo-contracts/packages/apps/src/transfer/components/transfer.cairo#L41
[create-ibc-token]: ../cairo-contracts/packages/apps/src/transfer/components/transfer.cairo#L274
[how-ibc-classic-works]: https://ibcprotocol.dev/how-ibc-classic-works
[ics20-spec]: https://github.com/cosmos/ibc/blob/main/spec/app/ics-020-fungible-token-transfer/README.md
[ibc-denoms]: https://ida.interchain.io/tutorials/6-ibc-dev
