use starknet_ibc_core::router::RouterHandlerComponent;
use starknet_ibc_core::router::RouterHandlerComponent::{CoreRouterHandler, RouterInitializerImpl};
use starknet_ibc_testkit::dummies::PORT_ID;
use starknet_ibc_testkit::mocks::MockRouterHandler;

type ComponentState = RouterHandlerComponent::ComponentState<MockRouterHandler::ContractState>;

fn COMPONENT_STATE() -> ComponentState {
    RouterHandlerComponent::component_state_for_testing()
}

fn setup() -> ComponentState {
    let mut state = COMPONENT_STATE();
    state.initializer();
    state
}

#[test]
#[should_panic(expected: 'ICS26: unsupported port id')]
fn test_missing_app_address() {
    let mut state = setup();
    state.app_address(PORT_ID());
}

#[test]
#[should_panic(expected: 'ICS26: unsupported port id')]
fn test_bind_release_port_id_ok() {
    let mut state = setup();
    let app_address = 'transfer'.try_into().unwrap();

    state.bind_port_id(PORT_ID(), app_address);
    let stored_app_address = state.app_address(PORT_ID());
    assert_eq!(stored_app_address, app_address);

    state.release_port_id(PORT_ID());
    state.app_address(PORT_ID());
}
