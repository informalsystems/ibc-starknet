use starknet::contract_address_const;
use starknet_ibc_core::router::RouterHandlerComponent::{RouterInitializerImpl, CoreRouterHandler};
use starknet_ibc_core::router::RouterHandlerComponent;
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
    state.app_address("transfer");
}

#[test]
#[should_panic(expected: 'ICS26: unsupported port id')]
fn test_bind_release_port_id_ok() {
    let mut state = setup();
    let port_id = "transfer";
    let app_address = contract_address_const::<'transfer'>();

    state.bind_port_id(port_id.clone(), app_address);
    let stored_app_address = state.app_address(port_id.clone());
    assert_eq!(stored_app_address, app_address);

    state.release_port_id(port_id.clone());
    state.app_address(port_id);
}
