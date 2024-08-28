#[starknet::component]
pub mod ClientHandlerComponent {
    use starknet::ContractAddress;
    use starknet_ibc_core_client::ClientEventEmitterComponent::ClientEventTrait as ClientEventTrait2;
    use starknet_ibc_core_client::ClientEventEmitterComponent;
    use starknet_ibc_core_client::{
        MsgCreateClient, MsgUpdateClient, MsgRecoverClient, MsgUpgradeClient, Height,
        CreateResponse, UpdateResponse, ClientErrors
    };
    use starknet_ibc_core_client::{ClientContract, ClientContractTrait};
    use starknet_ibc_core_host::{ClientId, ClientIdImpl};

    #[storage]
    struct Storage {
        supported_clients: LegacyMap<felt252, ContractAddress>,
    }

    #[event]
    #[derive(Debug, Drop, starknet::Event)]
    pub enum Event {
        #[flat]
        ClientHandlerEvent: ClientEventEmitterComponent::Event,
    }

    #[generate_trait]
    pub impl ClientInitializerImpl<
        TContractState, +HasComponent<TContractState>, +Drop<TContractState>
    > of ClientInitializerTrait<TContractState> {
        fn initializer(ref self: ComponentState<TContractState>) {}
    }

    #[generate_trait]
    pub impl ICS02ClientImpl<
        TContractState,
        +HasComponent<TContractState>,
        +Drop<TContractState>,
        impl EventEmitter: ClientEventEmitterComponent::HasComponent<TContractState>
    > of ICS02ClientTrait<TContractState> {
        fn create_client(ref self: ComponentState<TContractState>, msg: MsgCreateClient) {
            let mut client = self.get_client(msg.client_type);

            let create_resp = client.create(msg);

            self.emit_create_client_event(create_resp);
        }

        fn update_client(ref self: ComponentState<TContractState>, msg: MsgUpdateClient) {
            let mut client = self.get_client(msg.client_id.client_type);

            let update_result = client.update(msg.clone());

            match update_result {
                UpdateResponse::Success(heights) => self
                    .emit_update_client_event(msg.client_id, heights, msg.client_message),
                UpdateResponse::Misbehaviour => self.emit_misbehaviour_event(msg.client_id),
            }
        }

        fn recover_client(ref self: ComponentState<TContractState>, msg: MsgRecoverClient) {
            let mut client = self.get_client(msg.subject_client_id.client_type);

            client.recover(msg);

            self.emit_recover_client_event();
        }

        fn upgrade_client(ref self: ComponentState<TContractState>, msg: MsgUpgradeClient) {
            let mut client = self.get_client(msg.client_id.client_type);

            client.upgrade(msg);

            self.emit_upgrade_client_event();
        }
    }

    #[generate_trait]
    pub(crate) impl ClientInternalImpl<
        TContractState, +HasComponent<TContractState>, +Drop<TContractState>
    > of ClientInternalTrait<TContractState> {
        fn get_client(
            ref self: ComponentState<TContractState>, client_type: felt252
        ) -> ClientContract {
            let client: ClientContract = self.supported_clients.read(client_type).into();

            assert(client.is_non_zero(), ClientErrors::UNSUPPORTED_CLIENT_TYPE);

            client
        }

        fn register_client(
            ref self: ComponentState<TContractState>,
            client_type: felt252,
            client_address: ContractAddress
        ) {
            self.supported_clients.write(client_type, client_address);
        }
    }

    #[generate_trait]
    pub(crate) impl ClientEventImpl<
        TContractState,
        +HasComponent<TContractState>,
        +Drop<TContractState>,
        impl EventEmitter: ClientEventEmitterComponent::HasComponent<TContractState>
    > of ClientEventTrait<TContractState> {
        fn emit_create_client_event(
            ref self: ComponentState<TContractState>, create_resp: CreateResponse
        ) {
            let mut event_emitter = get_dep_component_mut!(ref self, EventEmitter);

            event_emitter.emit_create_client_event(create_resp);
        }

        fn emit_update_client_event(
            ref self: ComponentState<TContractState>,
            client_id: ClientId,
            update_heights: Array<Height>,
            client_message: Array<felt252>
        ) {
            let mut event_emitter = get_dep_component_mut!(ref self, EventEmitter);

            event_emitter.emit_update_client_event(client_id, update_heights, client_message);
        }

        fn emit_misbehaviour_event(ref self: ComponentState<TContractState>, client_id: ClientId) {
            let mut event_emitter = get_dep_component_mut!(ref self, EventEmitter);

            event_emitter.emit_misbehaviour_event(client_id);
        }

        fn emit_recover_client_event(ref self: ComponentState<TContractState>) {
            let mut event_emitter = get_dep_component_mut!(ref self, EventEmitter);

            event_emitter.emit_recover_client_event();
        }

        fn emit_upgrade_client_event(ref self: ComponentState<TContractState>) {
            let mut event_emitter = get_dep_component_mut!(ref self, EventEmitter);

            event_emitter.emit_upgrade_client_event();
        }
    }
}

