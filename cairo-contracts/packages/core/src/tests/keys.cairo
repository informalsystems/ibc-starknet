use starknet_ibc_core::host::{channel_end_key, commitment_key, next_sequence_ack_key};
use starknet_ibc_testkit::dummies::{CHANNEL_ID, PORT_ID, SEQUENCE};

#[test]
fn test_next_sequence_ack_key() {
    let channel_id = CHANNEL_ID(0);
    let port_id = PORT_ID();
    let key = next_sequence_ack_key(@port_id, @channel_id);
    let expected_key: felt252 = 0x3bd37d2d2afff7ce21f13c1fb0d1190cec01aba29de094629ce1e411114762c;
    assert_eq!(expected_key, key);
}

#[test]
fn test_commitment_key() {
    let channel_id = CHANNEL_ID(0);
    let port_id = PORT_ID();
    let sequence = SEQUENCE(1);
    let key = commitment_key(@port_id, @channel_id, @sequence);
    let expected_key: felt252 = 0x21d9da1890ea380cfe5af2c3e84da497be6d2c220159f4e81dd7949cf5512f1;
    assert_eq!(expected_key, key);
}

#[test]
fn test_channel_end_key() {
    let channel_id = CHANNEL_ID(0);
    let port_id = PORT_ID();
    let key = channel_end_key(@port_id, @channel_id);
    let expected_key: felt252 = 0x4763930c1c148bddb94c2603126c8d849bbc6921a0fc46734f21ebd2016579e;
    assert_eq!(expected_key, key);
}

