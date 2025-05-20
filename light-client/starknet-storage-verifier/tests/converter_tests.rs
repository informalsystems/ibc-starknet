use ibc_core::host::types::identifiers::{ChannelId, PortId};
use starknet_macros::{felt, selector};
use starknet_storage_verifier::value::{
    convert_storage_value, convert_value, next_sequence_ack_key,
};

#[test]
fn test_convert_value() {
    let base_key = selector!("store/ibc/key");
    let converted_value = convert_value(b"store/ibc/key");
    assert_eq!(base_key, converted_value);
}

#[test]
fn test_convert_value_2() {
    let base_key = selector!("ERC20_total_supply");
    let converted_value = convert_value(b"ERC20_total_supply");
    assert_eq!(base_key, converted_value);
}

#[test]
fn test_convert_value_3() {
    let base_key = selector!("ERC20_balances");
    let converted_value = convert_value(b"ERC20_balances");
    assert_eq!(base_key, converted_value);
}

// This test insures that the behavior is the same as the one in the Cairo contract
// cairo-contracts/packages/core/src/tests/keys.cairo
#[test]
fn test_next_sequence_ack_key() {
    let port_id = PortId::transfer();
    let channel_id = ChannelId::new(1);
    let converted_value = next_sequence_ack_key(port_id, channel_id);
    let expected_converted_value =
        felt!("0x6df926e78cbebe00e6ce07b5205dd71875e6b69b4cb8d1a07f4e00210b87d2c");
    assert_eq!(expected_converted_value, converted_value,);
}

#[test]
fn test_convert_storage_value() {
    let storage_member = selector!("nextSequenceAck");
    let path = "nextSequenceAck/ports/transfer/channels/channel-1";
    let converted_value = convert_storage_value(path, &storage_member);
    let expected_converted_value =
        felt!("0x737ee4d03531754bd7e5efc6e03842e0cc636aa06dd5d7537d51f6a8253d7dd");
    assert_eq!(expected_converted_value, converted_value,);
}
