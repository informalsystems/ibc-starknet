#[starknet::component]
pub mod TransferrableComponent {
    use starknet_ibc_app_transfer::TransferErrors;
    use starknet_ibc_app_transfer::transferrable::interface::ITransferrable;

    #[storage]
    struct Storage {
        send_capability: bool,
        receive_capability: bool,
    }

    #[event]
    #[derive(Drop, Debug, starknet::Event)]
    pub enum Event {}

    #[embeddable_as(Transferrable)]
    pub impl TransferrableImpl<
        TContractState, +HasComponent<TContractState>, +Drop<TContractState>
    > of ITransferrable<ComponentState<TContractState>> {
        fn can_send(self: @ComponentState<TContractState>) {
            let send_capability = self.send_capability.read();
            assert(send_capability, TransferErrors::NO_SEND_CAPABILITY);
        }
        fn can_receive(self: @ComponentState<TContractState>) {
            let receive_capability = self.receive_capability.read();
            assert(receive_capability, TransferErrors::NO_RECEIVE_CAPABILITY);
        }
    }

    #[generate_trait]
    pub impl TransferrableInternalImpl<
        TContractState, +HasComponent<TContractState>, +Drop<TContractState>
    > of TransferrableInternalTrait<TContractState> {
        fn initializer(ref self: ComponentState<TContractState>) {
            self.send_capability.write(true);
            self.receive_capability.write(true);
        }
    }
}

