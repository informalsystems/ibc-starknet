#[starknet::component]
pub mod IBCGovernanceComponent {
    use starknet::storage::{StoragePointerReadAccess, StoragePointerWriteAccess};
    use starknet::{ContractAddress, get_tx_info};
    use starknet_ibc_utils::governance::IGovernance;

    #[storage]
    pub struct Storage {
        governor: ContractAddress,
    }

    #[event]
    #[derive(Drop, Debug, starknet::Event)]
    pub enum Event {}

    #[embeddable_as(Governance)]
    pub impl GovernanceImpl<
        TContractState, +HasComponent<TContractState>, +Drop<TContractState>,
    > of IGovernance<ComponentState<TContractState>> {}

    #[generate_trait]
    pub impl GovernanceInternalImpl<
        TContractState, +HasComponent<TContractState>, +Drop<TContractState>,
    > of GovernanceInternalTrait<TContractState> {
        fn initializer(ref self: ComponentState<TContractState>) {
            self.governor.write(get_tx_info().deref().account_contract_address);
        }

        fn governor(self: @ComponentState<TContractState>) -> ContractAddress {
            self.governor.read()
        }
    }
}

