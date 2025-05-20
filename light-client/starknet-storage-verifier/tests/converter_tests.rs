use ibc_core::host::types::identifiers::{ChannelId, PortId};
use starknet_macros::felt;
use starknet_storage_verifier::value::{convert_storage_value, next_sequence_key};

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
