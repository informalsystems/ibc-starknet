use ibc_core::host::types::identifiers::{ChannelId, PortId, Sequence};
use starknet_macros::felt;
use starknet_storage_verifier::value::{convert_storage_value, next_sequence_key, packet_key};

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

#[test]
fn test_convert_storage_value_next_ack() {
    let path = "nextSequenceAck/ports/transfer/channels/channel-0";
    let converted_value = convert_storage_value(path);
    let expected_converted_value =
        felt!("0x5e66359665503ac3e2b1ba1f501b205eafb6fc8245b150b582d211be984809");
    assert_eq!(expected_converted_value, converted_value,);
}

#[test]
fn test_convert_storage_value_next_send() {
    let path = "nextSequenceSend/ports/transfer/channels/channel-0";
    let converted_value = convert_storage_value(path);
    let expected_converted_value =
        felt!("0x482fe9fb522fa0612fba38b07450f64552e45c3bf8d7bc016750a20661d0a32");
    assert_eq!(expected_converted_value, converted_value,);
}

#[test]
fn test_convert_storage_value_next_recv() {
    let path = "nextSequenceRecv/ports/transfer/channels/channel-0";
    let converted_value = convert_storage_value(path);
    let expected_converted_value =
        felt!("0x2c5d7959850dd27e4c48418937dc6111ab8296eb7cd11f4f1fada8fc682b7a6");
    assert_eq!(expected_converted_value, converted_value,);
}

#[test]
fn test_convert_storage_value_commitment() {
    let path = "commitments/ports/transfer/channels/channel-0/sequences/1";
    let converted_value = convert_storage_value(path);
    let expected_converted_value =
        felt!("0x4ebb4e50079c5a8fb2c3f930140fe962a536a04de1b497096c030c4247e185a");
    assert_eq!(expected_converted_value, converted_value,);
}

#[test]
fn test_convert_storage_value_ack() {
    let path = "acks/ports/transfer/channels/channel-0/sequences/1";
    let converted_value = convert_storage_value(path);
    let expected_converted_value =
        felt!("0x74b1f690db8f9af2955e740d0780847f3c32f67ed69a896368dcf37982aefb2");
    assert_eq!(expected_converted_value, converted_value,);
}

#[test]
fn test_convert_storage_value_receipt() {
    let path = "receipts/ports/transfer/channels/channel-0/sequences/1";
    let converted_value = convert_storage_value(path);
    let expected_converted_value =
        felt!("0x73e8c053278111020f75cbc1e556b2d03ea598b19d7748535a6fb7054c51d17");
    assert_eq!(expected_converted_value, converted_value,);
}
