use ibc_core::host::types::identifiers::{ChannelId, PortId};
use starknet_macros::felt;
use starknet_storage_verifier::value::{convert_storage_value, next_sequence_key};

// This test insures that the behavior is the same as the one in the Cairo contract
// cairo-contracts/packages/core/src/tests/keys.cairo
#[test]
fn test_next_sequence_ack_key() {
    let port_id = PortId::transfer();
    let channel_id = ChannelId::new(1);
    let converted_value = next_sequence_key("nextSequenceAck", port_id, channel_id);
    let expected_converted_value =
        felt!("0x6df926e78cbebe00e6ce07b5205dd71875e6b69b4cb8d1a07f4e00210b87d2c");
    assert_eq!(expected_converted_value, converted_value,);
}

#[test]
fn test_convert_storage_value() {
    let path = "nextSequenceAck/ports/transfer/channels/channel-1";
    let converted_value = convert_storage_value(path);
    let expected_converted_value =
        felt!("0x77e8da345d7cd987c2e7789209a63f676420fc5188f319993633a0d940f8fa3");
    assert_eq!(expected_converted_value, converted_value,);
}
