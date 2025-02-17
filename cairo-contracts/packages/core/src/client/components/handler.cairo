#[starknet::component]
pub mod ClientHandlerComponent {
    use core::num::traits::Zero;
    use starknet::storage::{
        Map, MutableVecTrait, StorageMapReadAccess, StorageMapWriteAccess, StoragePointerReadAccess,
        StoragePointerWriteAccess, Vec, VecTrait,
    };
    use starknet::{ContractAddress, get_caller_address, get_tx_info};
    use starknet_ibc_core::client::ClientEventEmitterComponent;
    use starknet_ibc_core::client::ClientEventEmitterComponent::ClientEventEmitterTrait;
    use starknet_ibc_core::client::interface::{IClientHandler, IRegisterClient, IRegisterRelayer};
    use starknet_ibc_core::client::{
        ClientContract, ClientContractHandlerTrait, ClientErrors, CreateResponse, Height,
        MsgCreateClient, MsgRecoverClient, MsgUpdateClient, MsgUpgradeClient, UpdateResponse,
    };
    use starknet_ibc_core::host::{ClientId, ClientIdImpl};
    use starknet_ibc_utils::governance::IBCGovernanceComponent;
    use starknet_ibc_utils::governance::IBCGovernanceComponent::GovernanceInternalTrait;

    #[storage]
    pub struct Storage {
        // NOTE: Temporary relayer whitelist for phase two,
        // to be replaced after Comet client contract is implemented.
        allowed_relayers: Vec<ContractAddress>,
        supported_clients: Map<felt252, ContractAddress>,
    }

    #[event]
    #[derive(Debug, Drop, starknet::Event)]
    pub enum Event {}

    // -----------------------------------------------------------
    // Client Initializer
    // -----------------------------------------------------------

    #[generate_trait]
    pub impl ClientInitializerImpl<
        TContractState, +HasComponent<TContractState>, +Drop<TContractState>,
    > of ClientInitializerTrait<TContractState> {
        fn initializer(ref self: ComponentState<TContractState>) {
            // NOTE: authorizing the contract's deployer as a relayer to
            // simplify the process. This avoids an additional registration step
            // for the relayer, as this setup is temporary.
            self.write_allowed_relayer(get_tx_info().deref().account_contract_address);
        }
    }

    // -----------------------------------------------------------
    // IClientHandler
    // -----------------------------------------------------------

    #[embeddable_as(CoreClientHandler)]
    pub impl CoreClientHandlerImpl<
        TContractState,
        +HasComponent<TContractState>,
        +Drop<TContractState>,
        impl EventEmitter: ClientEventEmitterComponent::HasComponent<TContractState>,
    > of IClientHandler<ComponentState<TContractState>> {
        fn create_client(
            ref self: ComponentState<TContractState>, msg: MsgCreateClient,
        ) -> CreateResponse {
            let mut client = self.get_client(msg.client_type);

            let create_resp = client.create(msg);

            self.emit_create_client_event(create_resp.clone());

            create_resp
        }

        fn update_client(
            ref self: ComponentState<TContractState>, msg: MsgUpdateClient,
        ) -> UpdateResponse {
            assert(
                self.in_allowed_relayers(get_caller_address()), ClientErrors::UNAUTHORIZED_RELAYER,
            );

            let mut client = self.get_client(msg.client_id.client_type);

            let update_result = client.update(msg.clone());

            match update_result.clone() {
                UpdateResponse::Success(heights) => self
                    .emit_update_client_event(msg.client_id, heights, msg.client_message),
                UpdateResponse::Misbehaviour => self.emit_misbehaviour_event(msg.client_id),
            }

            update_result
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

    // -----------------------------------------------------------
    // IRegisterClient
    // -----------------------------------------------------------

    #[embeddable_as(CoreRegisterClient)]
    pub impl CoreRegisterClientImpl<
        TContractState, +HasComponent<TContractState>, +Drop<TContractState>,
    > of IRegisterClient<ComponentState<TContractState>> {
        fn register_client(
            ref self: ComponentState<TContractState>,
            client_type: felt252,
            client_address: ContractAddress,
        ) {
            assert(
                self.in_allowed_relayers(get_caller_address()), ClientErrors::UNAUTHORIZED_RELAYER,
            );

            self.write_supported_client(client_type, client_address);
        }
    }

    // -----------------------------------------------------------
    // Allowed Relayers
    // -----------------------------------------------------------

    #[embeddable_as(CoreRegisterRelayer)]
    pub impl CoreRegisterRelayerImpl<
        TContractState,
        +HasComponent<TContractState>,
        +Drop<TContractState>,
        impl Governance: IBCGovernanceComponent::HasComponent<TContractState>,
    > of IRegisterRelayer<ComponentState<TContractState>> {
        fn register_relayer(
            ref self: ComponentState<TContractState>, relayer_address: ContractAddress,
        ) {
            assert(relayer_address.is_non_zero(), ClientErrors::ZERO_RELAYER_ADDRESS);

            assert(
                !self.in_allowed_relayers(relayer_address),
                ClientErrors::RELAYER_ALREADY_REGISTERED,
            );

            let governor = get_dep_component!(@self, Governance).governor();

            assert(
                governor.is_zero() || governor == get_caller_address(),
                ClientErrors::INVALID_GOVERNOR,
            );

            self.write_allowed_relayer(relayer_address);
        }
    }

    // -----------------------------------------------------------
    // Client Internal
    // -----------------------------------------------------------

    #[generate_trait]
    pub(crate) impl ClientInternalImpl<
        TContractState, +HasComponent<TContractState>, +Drop<TContractState>,
    > of ClientInternalTrait<TContractState> {
        fn get_client(
            self: @ComponentState<TContractState>, client_type: felt252,
        ) -> ClientContract {
            self.read_supported_client(client_type).into()
        }
    }

    // -----------------------------------------------------------
    // Client Reader/Writer
    // -----------------------------------------------------------

    #[generate_trait]
    pub(crate) impl ClientReaderImpl<
        TContractState, +HasComponent<TContractState>, +Drop<TContractState>,
    > of ClientReaderTrait<TContractState> {
        fn in_allowed_relayers(
            self: @ComponentState<TContractState>, caller: ContractAddress,
        ) -> bool {
            let mut allowed = false;
            let mut i = 0;
            while i < self.allowed_relayers.len() {
                if self.allowed_relayers.at(i).read() == caller {
                    allowed = true;
                    break;
                }
                i += 1;
            };
            allowed
        }

        fn read_supported_client(
            self: @ComponentState<TContractState>, client_type: felt252,
        ) -> ContractAddress {
            let client_address = self.supported_clients.read(client_type);

            assert(client_address.is_non_zero(), ClientErrors::ZERO_CLIENT_ADDRESS);

            client_address
        }
    }

    #[generate_trait]
    pub(crate) impl ClientWriterImpl<
        TContractState, +HasComponent<TContractState>, +Drop<TContractState>,
    > of ClientWriterTrait<TContractState> {
        fn write_allowed_relayer(
            ref self: ComponentState<TContractState>, relayer_address: ContractAddress,
        ) {
            self.allowed_relayers.append().write(relayer_address);
        }

        fn write_supported_client(
            ref self: ComponentState<TContractState>,
            client_type: felt252,
            client_address: ContractAddress,
        ) {
            self.supported_clients.write(client_type, client_address);
        }
    }

    // -----------------------------------------------------------
    // Client Event Emitter
    // -----------------------------------------------------------

    #[generate_trait]
    pub(crate) impl EventEmitterImpl<
        TContractState,
        +HasComponent<TContractState>,
        +Drop<TContractState>,
        impl EventEmitter: ClientEventEmitterComponent::HasComponent<TContractState>,
    > of EventEmitterTrait<TContractState> {
        fn emit_create_client_event(
            ref self: ComponentState<TContractState>, create_resp: CreateResponse,
        ) {
            let mut event_emitter = get_dep_component_mut!(ref self, EventEmitter);

            event_emitter.emit_create_client_event(create_resp);
        }

        fn emit_update_client_event(
            ref self: ComponentState<TContractState>,
            client_id: ClientId,
            update_heights: Array<Height>,
            client_message: Array<felt252>,
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

