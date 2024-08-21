#[starknet::component]
pub mod ICS02ClientComponent {
    use starknet::ContractAddress;
    use starknet_ibc_core_client::{
        MsgCreateClient, MsgUpdateClient, MsgRecoverClient, MsgUpgradeClient, Height, UpdateResult,
        ClientErrors
    };
    use starknet_ibc_core_client::{ClientContract, ClientContractTrait};
    use starknet_ibc_core_host::{ClientId, ClientIdImpl};

    #[storage]
    struct Storage {
        client_sequence: u64,
        supported_clients: LegacyMap<felt252, ContractAddress>,
    }

    #[event]
    #[derive(Debug, Drop, starknet::Event)]
    pub enum Event {
        CreateClientEvent: CreateClientEvent,
        UpdateClientEvent: UpdateClientEvent,
        MisbehaviourEvent: MisbehaviourEvent,
        RecoverClientEvent: RecoverClientEvent,
        UpgradeClientEvent: UpgradeClientEvent,
    }

    #[derive(Debug, Drop, starknet::Event)]
    pub struct CreateClientEvent {
        #[key]
        pub client_id: ClientId,
        pub consensus_height: Height,
    }

    #[derive(Debug, Drop, starknet::Event)]
    pub struct UpdateClientEvent {
        #[key]
        pub client_id: ClientId,
        pub consensus_heights: Array<Height>,
        pub header: Array<felt252>,
    }

    #[derive(Debug, Drop, starknet::Event)]
    pub struct MisbehaviourEvent {
        #[key]
        pub client_id: ClientId,
    }

    #[derive(Debug, Drop, starknet::Event)]
    pub struct RecoverClientEvent {}

    #[derive(Debug, Drop, starknet::Event)]
    pub struct UpgradeClientEvent {}

    #[generate_trait]
    pub impl ClientInitializerImpl<
        TContractState, +HasComponent<TContractState>, +Drop<TContractState>
    > of ClientInitializerTrait<TContractState> {
        fn initializer(ref self: ComponentState<TContractState>) {
            self.client_sequence.write(0);
        }
    }

    #[generate_trait]
    pub impl ICS02ClientImpl<
        TContractState, +HasComponent<TContractState>, +Drop<TContractState>
    > of ICS02ClientTrait<TContractState> {
        fn create_client(ref self: ComponentState<TContractState>, msg: MsgCreateClient) {
            let mut client = self.get_client(msg.client_type);

            let client_sequence = self.client_sequence.read();

            client.create(msg, client_sequence);

            self.emit_create_client_event(@client, client_sequence);

            self.client_sequence.write(client_sequence + 1);
        }

        fn update_client(ref self: ComponentState<TContractState>, msg: MsgUpdateClient) {
            let mut client = self.get_client(msg.client_id.client_type);

            let update_result = client.update(msg.clone());

            match update_result {
                UpdateResult::Success(heights) => self
                    .emit_update_client_event(
                        @client, msg.client_id.sequence, heights, msg.client_message
                    ),
                UpdateResult::Misbehaviour => self.emit_misbehaviour_event(@client),
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
        TContractState, +HasComponent<TContractState>, +Drop<TContractState>
    > of ClientEventTrait<TContractState> {
        fn emit_create_client_event(
            ref self: ComponentState<TContractState>, client: @ClientContract, client_sequence: u64
        ) {
            self
                .emit(
                    CreateClientEvent {
                        client_id: ClientIdImpl::new(client.client_type(), client_sequence),
                        consensus_height: client.latest_height(client_sequence),
                    }
                );
        }

        fn emit_update_client_event(
            ref self: ComponentState<TContractState>,
            client: @ClientContract,
            client_sequence: u64,
            update_heights: Array<Height>,
            client_message: Array<felt252>
        ) {
            self
                .emit(
                    UpdateClientEvent {
                        client_id: ClientIdImpl::new(client.client_type(), client_sequence,),
                        consensus_heights: update_heights,
                        header: client_message,
                    }
                );
        }

        fn emit_misbehaviour_event(
            ref self: ComponentState<TContractState>, client: @ClientContract
        ) {
            self
                .emit(
                    MisbehaviourEvent {
                        client_id: ClientIdImpl::new(
                            client.client_type(), self.client_sequence.read()
                        ),
                    }
                );
        }

        fn emit_recover_client_event(ref self: ComponentState<TContractState>) {}

        fn emit_upgrade_client_event(ref self: ComponentState<TContractState>) {}
    }
}

