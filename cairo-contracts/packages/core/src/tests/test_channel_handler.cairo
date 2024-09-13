use ChannelHandlerComponent::ChannelReaderTrait;
use core::num::traits::Zero;
use snforge_std::{spy_events, test_address, start_cheat_caller_address};
use starknet::ContractAddress;
use starknet::contract_address_const;
use starknet_ibc_core::channel::ChannelHandlerComponent::{
    ChannelInitializerImpl, ChannelWriterTrait
};
use starknet_ibc_core::channel::ChannelHandlerComponent;
use starknet_ibc_core::tests::mocks::channel_handler::MockChannelHandler;
use starknet_ibc_core::tests::{CHANNEL_END, CHANNEL_ID, CLIENT_ID, PORT_ID, SEQUENCE};

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
fn test_read_empty_storage() {
    let state = setup();
    let channel_end_resp = state.read_channel_end(@PORT_ID(), @CHANNEL_ID());
    assert!(channel_end_resp.is_none());

    let receipt_resp = state.read_packet_receipt(@PORT_ID(), @CHANNEL_ID(), @SEQUENCE());
    assert!(receipt_resp.is_none());

    let ack_resp = state.read_packet_ack(@PORT_ID(), @CHANNEL_ID(), @SEQUENCE());
    assert!(ack_resp.is_zero());
}

#[test]
fn test_channel_end_storage() {
    let mut state = setup();
    state.write_channel_end(@PORT_ID(), @CHANNEL_ID(), CHANNEL_END());
    let chan_end_res = state.read_channel_end(@PORT_ID(), @CHANNEL_ID());
    assert_eq!(chan_end_res, Option::Some(CHANNEL_END()));
}
