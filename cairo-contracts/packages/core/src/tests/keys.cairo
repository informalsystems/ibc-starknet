use starknet_ibc_core::host::next_sequence_ack_key;
use starknet_ibc_testkit::dummies::{CHANNEL_ID, PORT_ID};

#[test]
fn test_next_sequence_ack_key() {
    let channel_id = CHANNEL_ID(0);
    let port_id = PORT_ID();
    let key = next_sequence_ack_key(@port_id, @channel_id);
    let expected_key: felt252 = 0x3bd37d2d2afff7ce21f13c1fb0d1190cec01aba29de094629ce1e411114762c;
    assert_eq!(expected_key, key);
}
