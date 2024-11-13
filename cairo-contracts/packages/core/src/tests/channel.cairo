use core::num::traits::Zero;
use starknet_ibc_core::channel::ChannelHandlerComponent::{ChannelWriterTrait, ChannelReaderTrait};
use starknet_ibc_core::channel::{ChannelHandlerComponent, Receipt, ReceiptTrait};
use starknet_ibc_testkit::dummies::{CHANNEL_END, CHANNEL_ID, PORT_ID, SEQUENCE};
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
fn test_intial_state() {
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
#[should_panic(expected: 'ICS04: missing commitment')]
fn test_missing_packet_commitment() {
    let state = setup();
    state.read_packet_commitment(@PORT_ID(), @CHANNEL_ID(0), @SEQUENCE(0));
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
