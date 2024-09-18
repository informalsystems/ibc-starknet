use ChannelHandlerComponent::ChannelReaderTrait;
use core::num::traits::Zero;
use starknet_ibc_core::channel::ChannelHandlerComponent::{
    ChannelInitializerImpl, ChannelWriterTrait
};
use starknet_ibc_core::channel::ChannelHandlerComponent;
use starknet_ibc_core::host::SequenceTrait;
use starknet_ibc_core::tests::{CHANNEL_END, CHANNEL_ID, PORT_ID, SEQUENCE, MockChannelHandler};

type ComponentState = ChannelHandlerComponent::ComponentState<MockChannelHandler::ContractState>;

fn COMPONENT_STATE() -> ComponentState {
    ChannelHandlerComponent::component_state_for_testing()
}

fn setup() -> ComponentState {
    let mut state = COMPONENT_STATE();
    state.initializer();
    state
}

#[test]
fn test_intial_state() {
    let state = setup();
    let channel_end_resp = state.read_channel_end(@PORT_ID(), @CHANNEL_ID(0));
    assert!(channel_end_resp.is_some());

    let channel_end_resp = state.read_channel_end(@PORT_ID(), @CHANNEL_ID(1));
    assert!(channel_end_resp.is_none());

    let receipt_resp = state.read_packet_receipt(@PORT_ID(), @CHANNEL_ID(0), @SEQUENCE(0));
    assert!(receipt_resp.is_none());

    let ack_resp = state.read_packet_ack(@PORT_ID(), @CHANNEL_ID(0), @SEQUENCE(0));
    assert!(ack_resp.is_zero());

    let next_seq_resp = state.read_next_sequence_recv(@PORT_ID(), @CHANNEL_ID(0));
    assert!(next_seq_resp.is_zero());
}

#[test]
fn test_write_channel_end_ok() {
    let mut state = setup();
    state.write_channel_end(@PORT_ID(), @CHANNEL_ID(1), CHANNEL_END());
    let chan_end_res = state.read_channel_end(@PORT_ID(), @CHANNEL_ID(1));
    assert_eq!(chan_end_res, Option::Some(CHANNEL_END()));
}
