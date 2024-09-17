use starknet_ibc_apps::tests::{MockTransferApp, CLASS_HASH};
use starknet_ibc_apps::transfer::TokenTransferComponent::{
    TransferInitializerImpl, TransferReaderImpl
};
use starknet_ibc_apps::transfer::TokenTransferComponent;

type ComponentState = TokenTransferComponent::ComponentState<MockTransferApp::ContractState>;

fn COMPONENT_STATE() -> ComponentState {
    TokenTransferComponent::component_state_for_testing()
}

fn setup() -> ComponentState {
    let mut state = COMPONENT_STATE();
    state.initializer(CLASS_HASH());
    state
}

#[test]
fn test_init_state() {
    let state = setup();
    let class_hash = state.read_erc20_class_hash();
    assert_eq!(class_hash, CLASS_HASH());
}
