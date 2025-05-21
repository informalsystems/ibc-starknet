use ibc_core::host::types::identifiers::{ChannelId, PortId, Sequence};
use starknet_macros::felt;
use starknet_storage_verifier::value::{next_sequence_key, packet_key};

// This test insures that the behavior is the same as the one in the Cairo contract
// cairo-contracts/packages/core/src/tests/keys.cairo
#[test]
fn test_next_sequence_ack_key() {
    let port_id = PortId::transfer();
    let channel_id = ChannelId::new(0);
    let converted_value = next_sequence_key("nextSequenceAck", port_id, channel_id);
    let expected_converted_value =
        felt!("0x3bd37d2d2afff7ce21f13c1fb0d1190cec01aba29de094629ce1e411114762c");
    assert_eq!(expected_converted_value, converted_value,);
}

// This test insures that the behavior is the same as the one in the Cairo contract
// cairo-contracts/packages/core/src/tests/keys.cairo
#[test]
fn test_commitment_key() {
    let port_id = PortId::transfer();
    let channel_id = ChannelId::new(0);
    let sequence = Sequence::from(1);
    let converted_value = packet_key("commitments", port_id, channel_id, sequence);
    let expected_converted_value =
        felt!("0x21d9da1890ea380cfe5af2c3e84da497be6d2c220159f4e81dd7949cf5512f1");
    assert_eq!(expected_converted_value, converted_value,);
}

// This test insures that the behavior is the same as the one in the Cairo contract
// cairo-contracts/packages/core/src/tests/keys.cairo
#[test]
fn test_channel_ends_key() {
    let port_id = PortId::transfer();
    let channel_id = ChannelId::new(0);
    let converted_value = next_sequence_key("channelEnds", port_id, channel_id);
    let expected_converted_value =
        felt!("0x4763930c1c148bddb94c2603126c8d849bbc6921a0fc46734f21ebd2016579e");
    assert_eq!(expected_converted_value, converted_value,);
}
