# ICS20: IBC Fungible Tokens

## Prerequisites

- An IBC channel between Starknet <> Osmosis.
- A relayer that is listening and relaying on that IBC channel.

## ICS20 on Starknet <> Osmosis

For Osmosis, the ICS20 APIs are already handled by many wallets like Keplr and
Cosmostation. In this documentation, we solely focus on the APIs exposed by the
Starknet IBC implementation.

To handle token transfers, the wallet application needs to communicate with the
ICS20 contract deployed on Starknet.

A short recap of ICS20:

- sending native tokens will escrow the tokens in the ICS20 contract.
- sending remote IBC tokens will burn the tokens in the ICS20 contract.
- receiving native tokens will escrow the tokens in the ICS20 contract.
- receiving remote IBC tokens will mint the tokens in the ICS20 contract.

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

#### Sending Starknet native tokens

We need to use Native denom. Show how to form this in typescript.

#### Sending back Osmosis tokens

We need to use the TracePrefixed denom.

### Receiving fungible tokens

Considering a Cosmos wallet triggers a transfer from an Osmosis wallet to a
Starknet wallet, there is nothing to be done on Starknet side to receive the
tokens.

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

### IBC tokens

#### Manual Register

If you need to register Osmosis tokens before transferring.

#### IBC Token Metadata

Describe how the IBC tokens are represented.

[send-transfer]: https://github.com/informalsystems/ibc-starknet/blob/4665d0d053d075fa65fb4d748b6d844274064dcd/cairo-contracts/packages/apps/src/transfer/components/transfer.cairo#L150
[msg-transfer]: https://github.com/informalsystems/ibc-starknet/blob/4665d0d053d075fa65fb4d748b6d844274064dcd/cairo-contracts/packages/apps/src/transfer/types.cairo#L17
[ics20-events]: https://github.com/informalsystems/ibc-starknet/blob/4665d0d053d075fa65fb4d748b6d844274064dcd/cairo-contracts/packages/apps/src/transfer/components/transfer.cairo#L41
