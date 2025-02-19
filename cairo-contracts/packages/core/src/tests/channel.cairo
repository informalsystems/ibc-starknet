use core::num::traits::Zero;
use starknet_ibc_core::channel::ChannelHandlerComponent::{ChannelReaderTrait, ChannelWriterTrait};
use starknet_ibc_core::channel::{ChannelHandlerComponent, IChannelQuery, Receipt, ReceiptTrait};
use starknet_ibc_core::commitment::compute_packet_commitment;
use starknet_ibc_core::host::SequenceImpl;
use starknet_ibc_testkit::configs::TransferAppConfigImpl;
use starknet_ibc_testkit::configs::TransferAppConfigTrait;
use starknet_ibc_testkit::dummies::{
    CHANNEL_END, CHANNEL_ID, COSMOS, NATIVE_DENOM, PORT_ID, SEQUENCE, STARKNET, TIMEOUT_HEIGHT,
    TIMEOUT_TIMESTAMP,
};
use starknet_ibc_testkit::mocks::MockChannelHandler;

type ComponentState = ChannelHandlerComponent::ComponentState<MockChannelHandler::ContractState>;

fn COMPONENT_STATE() -> ComponentState {
    ChannelHandlerComponent::component_state_for_testing()
}

fn setup() -> ComponentState {
    let mut state = COMPONENT_STATE();
    state
}

#[test]
fn test_initial_state() {
    let state = setup();
    let next_channel_sequence = state.read_next_channel_sequence();
    assert!(next_channel_sequence.is_zero());

    let next_sequence_send = state.read_next_sequence_send(@PORT_ID(), @CHANNEL_ID(0));
    assert!(next_sequence_send.is_zero());

    let next_sequence_recv = state.read_next_sequence_recv(@PORT_ID(), @CHANNEL_ID(0));
    assert!(next_sequence_recv.is_zero());

    let next_sequence_ack = state.read_next_sequence_ack(@PORT_ID(), @CHANNEL_ID(0));
    assert!(next_sequence_ack.is_zero());
}

#[test]
fn test_write_read_channel_end_ok() {
    let mut state = setup();
    state.write_channel_end(@PORT_ID(), @CHANNEL_ID(10), CHANNEL_END(1));
    let channel_end = state.read_channel_end(@PORT_ID(), @CHANNEL_ID(10));
    assert_eq!(channel_end, CHANNEL_END(1));
}

#[test]
#[should_panic(expected: 'ICS04: missing channel end')]
fn test_missing_channel_end() {
    let state = setup();
    state.read_channel_end(@PORT_ID(), @CHANNEL_ID(10));
}

#[test]
fn test_write_read_packet_receipt_ok() {
    let mut state = setup();
    state.write_packet_receipt(@PORT_ID(), @CHANNEL_ID(10), @SEQUENCE(10), Receipt::Ok);
    let receipt = state.read_packet_receipt(@PORT_ID(), @CHANNEL_ID(10), @SEQUENCE(10));
    assert_eq!(receipt, Receipt::Ok);
}

#[test]
fn test_missing_packet_receipt() {
    let state = setup();
    let receipt = state.read_packet_receipt(@PORT_ID(), @CHANNEL_ID(0), @SEQUENCE(0));
    assert!(receipt.is_none());
}

#[test]
fn test_missing_packet_commitment() {
    let state = setup();
    let commitment = state.read_packet_commitment(@PORT_ID(), @CHANNEL_ID(0), @SEQUENCE(0));
    assert!(commitment.is_zero())
}

#[test]
#[should_panic(expected: 'ICS04: missing packet ack')]
fn test_missing_packet_ack() {
    let state = setup();
    state.read_packet_ack(@PORT_ID(), @CHANNEL_ID(0), @SEQUENCE(0));
}

#[test]
fn test_packet_ack_existence() {
    let state = setup();
    let if_exists = state.packet_ack_exists(@PORT_ID(), @CHANNEL_ID(0), @SEQUENCE(0));
    assert!(!if_exists);
}

#[test]
fn test_packet_commitment_sequences() {
    let mut state = setup();
    let cfg = TransferAppConfigImpl::default();
    let packet_data = cfg.dummy_packet_data(NATIVE_DENOM(), STARKNET(), COSMOS());
    let packet_commitment = compute_packet_commitment(
        @serde_json::to_byte_array(packet_data), TIMEOUT_HEIGHT(0), TIMEOUT_TIMESTAMP(1000),
    );
    let port_id = PORT_ID();
    let channel_id = CHANNEL_ID(0);
    // Assuming packets with sequence number of 2 and 4 are finalized(hence no commitment exists).
    let seq_1 = SEQUENCE(1);
    let seq_2 = SEQUENCE(3);
    let seq_3 = SEQUENCE(5);
    state.write_packet_commitment(@port_id, @channel_id, @seq_1, packet_commitment.clone());
    state.write_packet_commitment(@port_id, @channel_id, @seq_2, packet_commitment.clone());
    state.write_packet_commitment(@port_id, @channel_id, @seq_3, packet_commitment);
    state.write_next_sequence_send(@port_id, @channel_id, SEQUENCE(6));

    let sequences = state.packet_commitment_sequences(port_id, channel_id);
    assert_eq!(sequences, array![seq_3, seq_2, seq_1]);
}
