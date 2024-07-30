use core::serde::Serde;
use starknet::ContractAddress;
use starknet::contract_address_const;
use starknet_ibc::apps::transfer::types::{
    MsgTransfer, PacketData, PrefixedDenom, Denom, Memo, TracePrefixTrait
};
use starknet_ibc::core::channel::types::Packet;
use starknet_ibc::core::client::types::{Height, Timestamp};
use starknet_ibc::core::host::types::{ChannelId, PortId, Sequence};

pub(crate) const TOKEN_NAME: felt252 = 'NAME';
pub(crate) const DECIMALS: u8 = 18_u8;
pub(crate) const SUPPLY: u256 = 2000;
pub(crate) const AMOUNT: u256 = 100;
pub(crate) const SALT: felt252 = 'SALT';

pub(crate) fn NAME() -> ByteArray {
    "NAME"
}

pub(crate) fn SYMBOL() -> ByteArray {
    "SYMBOL"
}

pub(crate) fn BARE_DENOM(contract_address: ContractAddress) -> PrefixedDenom {
    PrefixedDenom { trace_path: array![], base: Denom::Native(contract_address.into()) }
}

pub(crate) fn NATIVE_PREFIXED_DENOM(contract_address: ContractAddress) -> PrefixedDenom {
    let trace_prefix = TracePrefixTrait::new(
        PortId { port_id: "transfer" }, ChannelId { channel_id: "channel-0" }
    );
    PrefixedDenom { trace_path: array![trace_prefix], base: Denom::Native(contract_address.into()) }
}

pub(crate) fn HOSTED_PREFIXED_DENOM() -> PrefixedDenom {
    let trace_prefix = TracePrefixTrait::new(
        PortId { port_id: "transfer" }, ChannelId { channel_id: "channel-0" }
    );
    PrefixedDenom { trace_path: array![trace_prefix], base: Denom::Hosted("uatom") }
}

pub(crate) fn PUBKEY() -> ContractAddress {
    contract_address_const::<0x049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7>()
}

pub(crate) fn OWNER() -> ContractAddress {
    contract_address_const::<'OWNER'>()
}

pub(crate) fn RECIPIENT() -> ContractAddress {
    contract_address_const::<'RECIPIENT'>()
}

pub(crate) fn dummy_erc20_call_data() -> Array<felt252> {
    let mut call_data: Array<felt252> = array![];
    Serde::serialize(@NAME(), ref call_data);
    Serde::serialize(@SYMBOL(), ref call_data);
    Serde::serialize(@SUPPLY, ref call_data);
    Serde::serialize(@OWNER(), ref call_data);
    Serde::serialize(@OWNER(), ref call_data);
    call_data
}

pub(crate) fn dummy_msg_transder(
    denom: PrefixedDenom, sender: ContractAddress, receiver: ContractAddress
) -> MsgTransfer {
    MsgTransfer {
        port_id_on_a: PortId { port_id: "transfer" },
        chan_id_on_a: ChannelId { channel_id: "channel-0" },
        packet_data: dummy_packet_data(denom, sender, receiver),
        timeout_height_on_b: Height { revision_number: 0, revision_height: 1000 },
        timeout_timestamp_on_b: Timestamp { timestamp: 1000 }
    }
}

pub(crate) fn dummy_recv_packet(
    denom: PrefixedDenom, sender: ContractAddress, receiver: ContractAddress
) -> Packet {
    let mut serialized_data = array![];
    Serde::serialize(@dummy_packet_data(denom, sender, receiver), ref serialized_data);

    Packet {
        seq_on_a: Sequence { sequence: 0 },
        port_id_on_a: PortId { port_id: "transfer" },
        chan_id_on_a: ChannelId { channel_id: "channel-0" },
        port_id_on_b: PortId { port_id: "transfer" },
        chan_id_on_b: ChannelId { channel_id: "channel-1" },
        data: serialized_data,
        timeout_height_on_b: Height { revision_number: 0, revision_height: 1000 },
        timeout_timestamp_on_b: Timestamp { timestamp: 1000 }
    }
}

pub(crate) fn dummy_packet_data(
    denom: PrefixedDenom, sender: ContractAddress, receiver: ContractAddress
) -> PacketData {
    PacketData { denom, amount: AMOUNT, sender, receiver, memo: Memo { memo: "" }, }
}
