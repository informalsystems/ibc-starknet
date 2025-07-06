//! This test file insure that the behavior of the methods used to convert the
//! key paths are the same as the ones in the Cairo contract.

use ibc_core::host::types::identifiers::{ChannelId, ConnectionId, PortId, Sequence};
use starknet_crypto_lib::funcs::StarknetCryptoLib;
use starknet_macros::felt;
use starknet_storage_verifier::ibc::{connection_key, next_sequence_key, packet_key};

#[test]
fn test_next_sequence_send_key() {
    let port_id = PortId::transfer();
    let channel_id = ChannelId::new(0);
    let converted_value =
        next_sequence_key::<StarknetCryptoLib>("nextSequenceSend", port_id, channel_id);
    let expected_converted_value =
        felt!("0xd3d1f8a9059807297ef2a9e4ff3c29f83fc879279fe9a90e27542a5a4a657b");
    assert_eq!(expected_converted_value, converted_value,);
}

#[test]
fn test_next_sequence_recv_key() {
    let port_id = PortId::transfer();
    let channel_id = ChannelId::new(0);
    let converted_value =
        next_sequence_key::<StarknetCryptoLib>("nextSequenceRecv", port_id, channel_id);
    let expected_converted_value =
        felt!("0x3425bf7734bd7178fc550139794c9601ca84413b7cc0b3f03e44ef98cd7402c");
    assert_eq!(expected_converted_value, converted_value,);
}

#[test]
fn test_next_sequence_ack_key() {
    let port_id = PortId::transfer();
    let channel_id = ChannelId::new(0);
    let converted_value =
        next_sequence_key::<StarknetCryptoLib>("nextSequenceAck", port_id, channel_id);
    let expected_converted_value =
        felt!("0x3bd37d2d2afff7ce21f13c1fb0d1190cec01aba29de094629ce1e411114762c");
    assert_eq!(expected_converted_value, converted_value,);
}

#[test]
fn test_commitment_key() {
    let port_id = PortId::transfer();
    let channel_id = ChannelId::new(0);
    let sequence = Sequence::from(1);
    let converted_value =
        packet_key::<StarknetCryptoLib>("commitments", port_id, channel_id, sequence);
    let expected_converted_value =
        felt!("0x21d9da1890ea380cfe5af2c3e84da497be6d2c220159f4e81dd7949cf5512f1");
    assert_eq!(expected_converted_value, converted_value,);
}

#[test]
fn test_ack_key() {
    let port_id = PortId::transfer();
    let channel_id = ChannelId::new(0);
    let sequence = Sequence::from(1);
    let converted_value = packet_key::<StarknetCryptoLib>("acks", port_id, channel_id, sequence);
    let expected_converted_value =
        felt!("0x3b02df8da2f3e88823af46061f810747c5b8c87d7d4de448b815f782201f7c9");
    assert_eq!(expected_converted_value, converted_value,);
}

#[test]
fn test_receipt_key() {
    let port_id = PortId::transfer();
    let channel_id = ChannelId::new(0);
    let sequence = Sequence::from(1);
    let converted_value =
        packet_key::<StarknetCryptoLib>("receipts", port_id, channel_id, sequence);
    let expected_converted_value =
        felt!("0x50c0dcd9f4776b5a563887592d4bc3279a818addeafaba5f43f1358bbfd3993");
    assert_eq!(expected_converted_value, converted_value,);
}

#[test]
fn test_channel_ends_key() {
    let port_id = PortId::transfer();
    let channel_id = ChannelId::new(0);
    let converted_value =
        next_sequence_key::<StarknetCryptoLib>("channelEnds", port_id, channel_id);
    let expected_converted_value =
        felt!("0x4763930c1c148bddb94c2603126c8d849bbc6921a0fc46734f21ebd2016579e");
    assert_eq!(expected_converted_value, converted_value,);
}

#[test]
fn test_connection_ends_key() {
    let connection_id = ConnectionId::new(0);
    let converted_value = connection_key::<StarknetCryptoLib>(connection_id);
    let expected_converted_value =
        felt!("0x208b5c68df93de403a4d24a5dae739dd3301be96546dfee39836b2ffa9e0584");
    assert_eq!(expected_converted_value, converted_value,);
}
