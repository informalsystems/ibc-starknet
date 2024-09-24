#[starknet::component]
pub mod RouterHandlerComponent {
    use core::num::traits::Zero;
    use starknet::ContractAddress;
    use starknet::storage::Map;
    use starknet_ibc_core::host::{PortId, PortIdImpl};
    use starknet_ibc_core::router::{RouterErrors, IRouter, ApplicationContract};
    use starknet_ibc_utils::ComputeKey;

    #[storage]
    struct Storage {
        pub port_id_to_app: Map<felt252, ContractAddress>,
    }

    #[event]
    #[derive(Debug, Drop, starknet::Event)]
    pub enum Event {}

    // -----------------------------------------------------------
    // Router Initializer
    // -----------------------------------------------------------

    #[generate_trait]
    pub impl RouterInitializerImpl<
        TContractState, +HasComponent<TContractState>, +Drop<TContractState>
    > of RouterInitializerTrait<TContractState> {
        fn initializer(ref self: ComponentState<TContractState>) {}
    }

    // -----------------------------------------------------------
    // IRouter
    // -----------------------------------------------------------

    #[embeddable_as(CoreRouterHandler)]
    impl CoreRouterHandlerImpl<
        TContractState, +HasComponent<TContractState>, +Drop<TContractState>
    > of IRouter<ComponentState<TContractState>> {
        fn get_app_address(
            self: @ComponentState<TContractState>, port_id: ByteArray
        ) -> Option<ContractAddress> {
            let app_address = self.read_app_address(@PortIdImpl::new(port_id));

            if app_address.is_non_zero() {
                Option::Some(app_address)
            } else {
                Option::None
            }
        }

        fn bind_port_id(
            ref self: ComponentState<TContractState>,
            port_id: ByteArray,
            app_address: ContractAddress
        ) {
            self.write_app_address(PortIdImpl::new(port_id), app_address)
        }

        fn release_port_id(ref self: ComponentState<TContractState>, port_id: ByteArray) {
            self.remove_app_address(PortIdImpl::new(port_id))
        }
    }

    // -----------------------------------------------------------
    // Router Internal
    // -----------------------------------------------------------

    #[generate_trait]
    pub(crate) impl RouterInternalImpl<
        TContractState, +HasComponent<TContractState>, +Drop<TContractState>
    > of RouterInternalTrait<TContractState> {
        fn get_app(self: @ComponentState<TContractState>, port_id: @PortId) -> ApplicationContract {
            let maybe_app_address = self.read_app_address(port_id);

            assert(maybe_app_address.is_non_zero(), RouterErrors::UNSUPPORTED_PORT_ID);

            maybe_app_address.into()
        }
    }

    // -----------------------------------------------------------
    // Router Reader/Writer
    // -----------------------------------------------------------

    #[generate_trait]
    pub(crate) impl RouterReaderImpl<
        TContractState, +HasComponent<TContractState>, +Drop<TContractState>
    > of RouterReaderTrait<TContractState> {
        fn read_app_address(
            self: @ComponentState<TContractState>, port_id: @PortId
        ) -> ContractAddress {
            self.port_id_to_app.read(port_id.key())
        }
    }

    #[generate_trait]
    pub(crate) impl RouterWriterImpl<
        TContractState, +HasComponent<TContractState>, +Drop<TContractState>
    > of RouterWriterTrait<TContractState> {
        fn write_app_address(
            ref self: ComponentState<TContractState>, port_id: PortId, app_address: ContractAddress
        ) {
            self.port_id_to_app.write(port_id.key(), app_address)
        }

        fn remove_app_address(ref self: ComponentState<TContractState>, port_id: PortId) {
            self.port_id_to_app.write(port_id.key(), Zero::zero())
        }
    }
}

