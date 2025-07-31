#[starknet::component]
pub mod ClientHandlerComponent {
    use core::num::traits::Zero;
    use openzeppelin_access::ownable::OwnableComponent;
    use openzeppelin_access::ownable::OwnableComponent::InternalTrait;
    use starknet::storage::{
        IntoIterRange, Map, MutableVecTrait, StorageMapReadAccess, StorageMapWriteAccess,
        StoragePointerReadAccess, Vec,
    };
    use starknet::{ContractAddress, get_caller_address, get_tx_info};
    use starknet_ibc_core::client::ClientEventEmitterComponent::ClientEventEmitterTrait;
    use starknet_ibc_core::client::interface::{
        IClientHandler, IRegisterClient, IRegisterRelayer, IScheduleUpgrade,
    };
    use starknet_ibc_core::client::{
        ClientContract, ClientContractHandlerTrait, ClientErrors, ClientEventEmitterComponent,
        CreateResponse, Height, MsgCreateClient, MsgRecoverClient, MsgScheduleUpgrade,
        MsgUpdateClient, MsgUpgradeClient, StarknetClientState, StarknetConsensusState,
        UpdateResponse,
    };
    use starknet_ibc_core::host::{ClientId, ClientIdImpl};
    use starknet_ibc_utils::governance::IBCGovernanceComponent;
    use starknet_ibc_utils::governance::IBCGovernanceComponent::GovernanceInternalTrait;
    use starknet_ibc_utils::{ComputeKey, ValidateBasic};

    #[storage]
    pub struct Storage {
        // NOTE: Temporary relayer whitelist for phase two,
        // to be replaced after Comet client contract is implemented.
        allowed_relayers: Vec<ContractAddress>,
        supported_clients: Map<felt252, ContractAddress>,
        upgraded_states: Map<u64, (StarknetClientState, StarknetConsensusState)>,
        // commitments for the upgraded client state and consensus state
        upgraded_client_state_commitments: Map<u64, felt252>,
        upgraded_consensus_state_commitments: Map<u64, felt252>,
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

            let client_id = msg.client_id.clone();
            let client_message = msg.client_message.clone();

            let update_result = client.update(msg);

            match update_result.clone() {
                UpdateResponse::Success(heights) => self
                    .emit_update_client_event(client_id, heights, client_message),
                UpdateResponse::Misbehaviour => self.emit_misbehaviour_event(client_id),
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
    // Schedule Upgrade
    // -----------------------------------------------------------

    #[embeddable_as(CoreScheduleUpgrade)]
    pub impl CoreScheduleUpgradeImpl<
        TContractState,
        +HasComponent<TContractState>,
        +Drop<TContractState>,
        impl EventEmitter: ClientEventEmitterComponent::HasComponent<TContractState>,
        impl Ownable: OwnableComponent::HasComponent<TContractState>,
    > of IScheduleUpgrade<ComponentState<TContractState>> {
        fn schedule_upgrade(ref self: ComponentState<TContractState>, msg: MsgScheduleUpgrade) {
            {
                // only admin can schedule an upgrade
                let ownable = get_dep_component!(@self, Ownable);
                ownable.assert_only_owner();
            }

            msg.validate_basic();

            let MsgScheduleUpgrade { upgraded_client_state, upgraded_consensus_state } = msg;

            let upgraded_height = upgraded_client_state.latest_height;

            self
                .upgraded_states
                .write(
                    upgraded_height,
                    (upgraded_client_state.clone(), upgraded_consensus_state.clone()),
                );

            self
                .upgraded_client_state_commitments
                .write(upgraded_height, upgraded_client_state.key());
            self
                .upgraded_consensus_state_commitments
                .write(upgraded_height, upgraded_consensus_state.key());

            self.emit_schedule_upgrade_event(upgraded_client_state, upgraded_consensus_state);
        }

        // TODO(rano): implement read, update, delete methods for upgraded states 
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
            let mut iterator = self.allowed_relayers.into_iter_full_range();

            while let Some(relayer) = iterator.next() {
                if relayer.read() == caller {
                    allowed = true;
                    break;
                }
            }

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
            self.allowed_relayers.push(relayer_address);
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

        fn emit_schedule_upgrade_event(
            ref self: ComponentState<TContractState>,
            upgraded_client_state: StarknetClientState,
            upgraded_consensus_state: StarknetConsensusState,
        ) {
            let mut event_emitter = get_dep_component_mut!(ref self, EventEmitter);

            event_emitter
                .emit_schedule_upgrade_event(upgraded_client_state, upgraded_consensus_state);
        }
    }
}

