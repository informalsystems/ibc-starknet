use starknet_ibc_core::host::{
    ack_key, channel_end_key, commitment_key, connection_end_key, next_sequence_ack_key,
    next_sequence_recv_key, next_sequence_send_key, receipt_key,
};
use starknet_ibc_testkit::dummies::{CHANNEL_ID, CONNECTION_ID, PORT_ID, SEQUENCE};

#[test]
fn test_next_sequence_send_key() {
    let channel_id = CHANNEL_ID(0);
    let port_id = PORT_ID();
    let key = next_sequence_send_key(@port_id, @channel_id);
    let expected_key: felt252 = 0xd3d1f8a9059807297ef2a9e4ff3c29f83fc879279fe9a90e27542a5a4a657b;
    assert_eq!(expected_key, key);
}

#[test]
fn test_next_sequence_recv_key() {
    let channel_id = CHANNEL_ID(0);
    let port_id = PORT_ID();
    let key = next_sequence_recv_key(@port_id, @channel_id);
    let expected_key: felt252 = 0x3425bf7734bd7178fc550139794c9601ca84413b7cc0b3f03e44ef98cd7402c;
    assert_eq!(expected_key, key);
}

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
fn test_ack_key() {
    let channel_id = CHANNEL_ID(0);
    let port_id = PORT_ID();
    let sequence = SEQUENCE(1);
    let key = ack_key(@port_id, @channel_id, @sequence);
    let expected_key: felt252 = 0x3b02df8da2f3e88823af46061f810747c5b8c87d7d4de448b815f782201f7c9;
    assert_eq!(expected_key, key);
}

#[test]
fn test_receipt_key() {
    let channel_id = CHANNEL_ID(0);
    let port_id = PORT_ID();
    let sequence = SEQUENCE(1);
    let key = receipt_key(@port_id, @channel_id, @sequence);
    let expected_key: felt252 = 0x50c0dcd9f4776b5a563887592d4bc3279a818addeafaba5f43f1358bbfd3993;
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

#[test]
fn test_connection_end_key() {
    let connection_id = CONNECTION_ID(0);
    let key = connection_end_key(@connection_id);
    let expected_key: felt252 = 0x208b5c68df93de403a4d24a5dae739dd3301be96546dfee39836b2ffa9e0584;
    assert_eq!(expected_key, key);
}

