use starknet::ContractAddress;
use starknet::contract_address_const;
use starknet_ibc::apps::transfer::types::{MsgTransfer, Packet, PacketData, Token, Denom, Memo};
use starknet_ibc::core::types::{Height, Timestamp, ChannelId, PortId, Sequence};

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

pub(crate) fn PREFIXED_DENOM() -> ByteArray {
    "transfer/channel-0/uatom"
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

pub(crate) fn dummy_msg_transder(denom: Denom) -> MsgTransfer {
    let port_id_on_a = PortId { port_id: 'transfer' };

    let chan_id_on_a = ChannelId { channel_id: 'channel-0' };

    let packet_data = PacketData {
        token: Token { denom: denom, amount: AMOUNT },
        sender: OWNER(),
        receiver: RECIPIENT(),
        memo: Memo { memo: "" },
    };

    MsgTransfer { port_id_on_a, chan_id_on_a, packet_data, }
}

pub(crate) fn dummy_recv_packet(denom: Denom) -> Packet {
    let data = PacketData {
        token: Token { denom: denom, amount: AMOUNT },
        sender: OWNER(),
        receiver: RECIPIENT(),
        memo: Memo { memo: "" },
    };

    Packet {
        seq_on_a: Sequence { sequence: 0 },
        port_id_on_a: PortId { port_id: 'transfer' },
        chan_id_on_a: ChannelId { channel_id: 'channel-0' },
        port_id_on_b: PortId { port_id: 'transfer' },
        chan_id_on_b: ChannelId { channel_id: 'channel-1' },
        data,
        timeout_height_on_b: Height { revision_number: 0, revision_height: 1000 },
        timeout_timestamp_on_b: Timestamp { timestamp: 1000 }
    }
}
