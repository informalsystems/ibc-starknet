use starknet_ibc_core::connection::ConnectionHandlerComponent;
use starknet_ibc_testkit::mocks::MockConnectionHandler;

type ComponentState =
    ConnectionHandlerComponent::ComponentState<MockConnectionHandler::ContractState>;

fn COMPONENT_STATE() -> ComponentState {
    ConnectionHandlerComponent::component_state_for_testing()
}

fn setup() -> ComponentState {
    let mut state = COMPONENT_STATE();
    state
}
