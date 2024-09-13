use ChannelHandlerComponent::ChannelReaderTrait;
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
fn test_read_empty_packet_receipt() {
    let state = setup();
    let receipt_res = state.read_packet_receipt(@PORT_ID(), @CHANNEL_ID(), @SEQUENCE());
    assert!(receipt_res.is_none());
}

#[test]
fn test_read_empty_packet_ack() {
    let state = setup();
    let ack_res = state.read_packet_ack(@PORT_ID(), @CHANNEL_ID(), @SEQUENCE());
    assert!(ack_res.len() == 0);
}

#[test]
#[should_panic]
fn test_read_empty_channel_end() {
    let state = setup();
    state.read_channel_end(@PORT_ID(), @CHANNEL_ID());
}

#[test]
fn test_channel_end_storage() {
    let mut state = setup();
    state.write_channel_end(@PORT_ID(), @CHANNEL_ID(), CHANNEL_END());
    let chan_end_res = state.read_channel_end(@PORT_ID(), @CHANNEL_ID());
    assert_eq!(chan_end_res, CHANNEL_END());
}
