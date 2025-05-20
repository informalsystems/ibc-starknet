use starknet_ibc_core::host::next_sequence_ack_key;
use starknet_ibc_testkit::dummies::PORT_ID;
use starknet_ibc_testkit::dummies::CHANNEL_ID;

#[test]
fn test_next_sequence_ack_key() {
    let channel_id = CHANNEL_ID(1);
    let port_id = PORT_ID();
    let key = next_sequence_ack_key(@port_id, @channel_id);
    let expected_key: felt252 = 0x6df926e78cbebe00e6ce07b5205dd71875e6b69b4cb8d1a07f4e00210b87d2c;
    assert_eq!(expected_key, key);
}