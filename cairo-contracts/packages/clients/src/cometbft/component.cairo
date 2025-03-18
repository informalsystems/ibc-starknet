#[starknet::component]
pub mod CometClientComponent {
    use alexandria_data_structures::array_ext::ArrayTraitExt;
    use alexandria_sorting::MergeSort;
    use core::num::traits::Zero;
    use openzeppelin_access::ownable::OwnableComponent;
    use openzeppelin_access::ownable::interface::IOwnable;
    use starknet::storage::{
        Map, StorageMapReadAccess, StorageMapWriteAccess, StoragePointerReadAccess,
        StoragePointerWriteAccess,
    };
    use starknet::{ContractAddress, get_block_number, get_block_timestamp, get_caller_address};
    use starknet_ibc_clients::cometbft::{
        CometClientState, CometClientStateImpl, CometConsensusState, CometConsensusStateImpl,
        CometErrors, CometHeader, CometHeaderImpl,
    };
    use starknet_ibc_core::client::{
        CreateResponse, CreateResponseImpl, Height, HeightImpl, HeightPartialOrd, HeightZero,
        IClientHandler, IClientQuery, IClientStateExecution, IClientStateValidation,
        MsgCreateClient, MsgRecoverClient, MsgUpdateClient, MsgUpgradeClient, Status, StatusTrait,
        StoreHeightArray, Timestamp, UpdateResponse,
    };
    use starknet_ibc_core::commitment::{StateProof, StateRoot, StateValue};
    use starknet_ibc_core::host::ClientIdImpl;
    use starknet_ibc_utils::ValidateBasic;

    #[storage]
    pub struct Storage {
        next_client_sequence: u64,
        update_heights: Map<u64, Array<Height>>,
        client_states: Map<u64, CometClientState>,
        consensus_states: Map<(u64, Height), CometConsensusState>,
        client_processed_times: Map<(u64, Height), u64>,
        client_processed_heights: Map<(u64, Height), u64>,
    }

    #[event]
    #[derive(Debug, Drop, starknet::Event)]
    pub enum Event {}

    // -----------------------------------------------------------
    // IClientHandler
    // -----------------------------------------------------------

    #[embeddable_as(CometClientHandler)]
    impl ClientHandlerImpl<
        TContractState,
        +HasComponent<TContractState>,
        +Drop<TContractState>,
        impl Ownable: OwnableComponent::HasComponent<TContractState>,
    > of IClientHandler<ComponentState<TContractState>> {
        fn create_client(
            ref self: ComponentState<TContractState>, msg: MsgCreateClient,
        ) -> CreateResponse {
            self.assert_owner();
            let client_sequence = self.read_next_client_sequence();
            self.create_validate(client_sequence, msg.clone());
            self.create_execute(client_sequence, msg)
        }

        fn update_client(
            ref self: ComponentState<TContractState>, msg: MsgUpdateClient,
        ) -> UpdateResponse {
            self.assert_owner();
            self.update_validate(msg.clone());
            self.update_execute(msg)
        }

        fn recover_client(ref self: ComponentState<TContractState>, msg: MsgRecoverClient) {
            self.assert_owner();
        }

        fn upgrade_client(ref self: ComponentState<TContractState>, msg: MsgUpgradeClient) {
            self.assert_owner();
        }
    }

    // -----------------------------------------------------------
    // IClientQuery
    // -----------------------------------------------------------

    #[embeddable_as(CometClientQuery)]
    impl ClientQueryImpl<
        TContractState, +HasComponent<TContractState>, +Drop<TContractState>,
    > of IClientQuery<ComponentState<TContractState>> {
        fn client_type(self: @ComponentState<TContractState>) -> felt252 {
            '07-tendermint'
        }

        fn latest_height(self: @ComponentState<TContractState>, client_sequence: u64) -> Height {
            let comet_client_state: CometClientState = self.read_client_state(client_sequence);

            comet_client_state.latest_height
        }

        fn update_height_before(
            self: @ComponentState<TContractState>, client_sequence: u64, target_height: Height,
        ) -> Height {
            let update_heights = self.read_update_heights(client_sequence);

            let mut len = update_heights.len();

            assert(len > 0, CometErrors::ZERO_UPDATE_HEIGHTS);

            let mut height = HeightZero::zero();

            while len > 0 {
                let update_height = update_heights.at(len - 1);
                if @target_height >= update_height {
                    height = *update_height;
                    break;
                }
                if len == 1 && height.is_zero() {
                    height = target_height;
                    break;
                }
                len -= 1;
            }

            height
        }

        fn latest_timestamp(
            self: @ComponentState<TContractState>, client_sequence: u64,
        ) -> Timestamp {
            let latest_height = self.latest_height(client_sequence);

            let consensus_state = self.read_consensus_state(client_sequence, latest_height);

            consensus_state.timestamp
        }

        fn status(self: @ComponentState<TContractState>, client_sequence: u64) -> Status {
            let comet_client_state: CometClientState = self.read_client_state(client_sequence);

            let latest_consensus_state = self
                .read_consensus_state(client_sequence, comet_client_state.latest_height.clone());

            self._status(comet_client_state, latest_consensus_state, client_sequence)
        }

        fn client_state(
            self: @ComponentState<TContractState>, client_sequence: u64,
        ) -> Array<felt252> {
            let mut client_state: Array<felt252> = ArrayTrait::new();

            self.read_client_state(client_sequence).serialize(ref client_state);

            client_state
        }

        fn consensus_state(
            self: @ComponentState<TContractState>, client_sequence: u64, height: Height,
        ) -> Array<felt252> {
            let mut consensus_state: Array<felt252> = ArrayTrait::new();

            self.read_consensus_state(client_sequence, height).serialize(ref consensus_state);

            consensus_state
        }

        fn consensus_state_root(
            self: @ComponentState<TContractState>, client_sequence: u64, height: Height,
        ) -> StateRoot {
            self.read_consensus_state(client_sequence, height).root
        }
    }

    // -----------------------------------------------------------
    // Client handler implementations
    // -----------------------------------------------------------

    #[generate_trait]
    pub(crate) impl CreateClientImpl<
        TContractState, +HasComponent<TContractState>, +Drop<TContractState>,
    > of CreateClientTrait<TContractState> {
        fn create_validate(
            self: @ComponentState<TContractState>, client_sequence: u64, msg: MsgCreateClient,
        ) {
            msg.validate_basic();

            assert(msg.client_type == self.client_type(), CometErrors::INVALID_CLIENT_TYPE);

            let comet_client_state = CometClientStateImpl::deserialize(msg.client_state);

            let comet_consensus_state = CometConsensusStateImpl::deserialize(msg.consensus_state);

            let status = self._status(comet_client_state, comet_consensus_state, client_sequence);

            assert(status.is_active(), CometErrors::INACTIVE_CLIENT);
        }

        fn create_execute(
            ref self: ComponentState<TContractState>, client_sequence: u64, msg: MsgCreateClient,
        ) -> CreateResponse {
            self.initialize(client_sequence, msg.client_state, msg.consensus_state)
        }
    }

    #[generate_trait]
    pub(crate) impl UpdateClientImpl<
        TContractState, +HasComponent<TContractState>, +Drop<TContractState>,
    > of UpdateClientTrait<TContractState> {
        fn update_validate(self: @ComponentState<TContractState>, msg: MsgUpdateClient) {
            msg.validate_basic();

            assert(
                msg.client_id.client_type == self.client_type(), CometErrors::INVALID_CLIENT_TYPE,
            );

            let client_sequence = msg.client_id.sequence;

            let comet_client_state: CometClientState = self.read_client_state(client_sequence);

            let latest_consensus_state = self
                .read_consensus_state(client_sequence, comet_client_state.latest_height.clone());

            let status = self._status(comet_client_state, latest_consensus_state, client_sequence);

            assert(status.is_active(), CometErrors::INACTIVE_CLIENT);

            self.verify_client_message(client_sequence, msg.client_message);
        }

        fn update_execute(
            ref self: ComponentState<TContractState>, msg: MsgUpdateClient,
        ) -> UpdateResponse {
            let client_sequence = msg.client_id.sequence;

            if self.verify_misbehaviour(client_sequence, msg.client_message.clone()) {
                return self.update_on_misbehaviour(client_sequence, msg.client_message.clone());
            }

            self.update_state(client_sequence, msg.client_message.clone())
        }
    }

    #[generate_trait]
    pub(crate) impl RecoverClientImpl<
        TContractState, +HasComponent<TContractState>, +Drop<TContractState>,
    > of RecoverClientTrait<TContractState> {
        fn recover_validate(self: @ComponentState<TContractState>, msg: MsgRecoverClient) {
            msg.validate_basic();
        }

        fn recover_execute(ref self: ComponentState<TContractState>, msg: MsgRecoverClient) {}
    }

    #[generate_trait]
    pub(crate) impl UpgradeClientImpl<
        TContractState, +HasComponent<TContractState>, +Drop<TContractState>,
    > of UpgradeClientTrait<TContractState> {
        fn upgrade_validate(self: @ComponentState<TContractState>, msg: MsgUpgradeClient) {
            msg.validate_basic();
        }

        fn upgrade_execute(ref self: ComponentState<TContractState>, msg: MsgUpgradeClient) {}
    }

    // -----------------------------------------------------------
    // Client Validation/Execution
    // -----------------------------------------------------------

    #[embeddable_as(CometClientValidation)]
    impl ClientValidationImpl<
        TContractState, +HasComponent<TContractState>, +Drop<TContractState>,
    > of IClientStateValidation<ComponentState<TContractState>> {
        fn verify_membership(
            self: @ComponentState<TContractState>,
            client_sequence: u64,
            path: ByteArray,
            value: StateValue,
            proof: StateProof,
            root: StateRoot,
        ) {}

        fn verify_non_membership(
            self: @ComponentState<TContractState>,
            client_sequence: u64,
            path: ByteArray,
            proof: StateProof,
            root: StateRoot,
        ) {}

        fn verify_client_message(
            self: @ComponentState<TContractState>,
            client_sequence: u64,
            client_message: Array<felt252>,
        ) {}

        fn verify_misbehaviour(
            self: @ComponentState<TContractState>,
            client_sequence: u64,
            client_message: Array<felt252>,
        ) -> bool {
            false
        }

        fn verify_substitute(
            self: @ComponentState<TContractState>, substitute_client_state: Array<felt252>,
        ) {}

        fn verify_upgrade(
            self: @ComponentState<TContractState>,
            upgrade_client_state: Array<felt252>,
            upgrade_consensus_state: Array<felt252>,
            proof_upgrade_client: Array<felt252>,
            proof_upgrade_consensus: Array<felt252>,
            root: ByteArray,
        ) {}
    }

    #[embeddable_as(CometClientExecution)]
    impl ClientExecutionImpl<
        TContractState, +HasComponent<TContractState>, +Drop<TContractState>,
    > of IClientStateExecution<ComponentState<TContractState>> {
        fn initialize(
            ref self: ComponentState<TContractState>,
            client_sequence: u64,
            client_state: Array<felt252>,
            consensus_state: Array<felt252>,
        ) -> CreateResponse {
            let comet_client_state = CometClientStateImpl::deserialize(client_state);

            let comet_consensus_state = CometConsensusStateImpl::deserialize(consensus_state);

            self
                ._update_state(
                    client_sequence,
                    comet_client_state.latest_height,
                    comet_client_state.clone(),
                    comet_consensus_state,
                );

            let client_id = ClientIdImpl::new(self.client_type(), client_sequence);

            CreateResponseImpl::new(client_id, comet_client_state.latest_height)
        }

        fn update_state(
            ref self: ComponentState<TContractState>,
            client_sequence: u64,
            client_message: Array<felt252>,
        ) -> UpdateResponse {
            let header: CometHeader = CometHeaderImpl::deserialize(client_message);

            let header_height = header.clone().signed_header.height;

            // TODO: Implement consensus state pruning mechanism.

            if !self.consensus_state_exists(client_sequence, header_height.clone()) {
                let mut client_state = self.read_client_state(client_sequence);

                client_state.update(header_height.clone());

                let new_consensus_state: CometConsensusState = header.into();

                self
                    ._update_state(
                        client_sequence, header_height, client_state, new_consensus_state,
                    );
            }

            array![header_height].into()
        }

        fn update_on_misbehaviour(
            ref self: ComponentState<TContractState>,
            client_sequence: u64,
            client_message: Array<felt252>,
        ) -> UpdateResponse {
            let header = CometHeaderImpl::deserialize(client_message);

            let mut client_state = self.read_client_state(client_sequence);

            client_state.freeze(header.trusted_height);

            self.write_client_state(client_sequence, client_state);

            UpdateResponse::Misbehaviour
        }

        fn update_on_recover(
            ref self: ComponentState<TContractState>,
            subject_client_sequence: u64,
            substitute_client_state: Array<felt252>,
            substitute_consensus_state: Array<felt252>,
        ) {}

        fn update_on_upgrade(
            ref self: ComponentState<TContractState>,
            client_sequence: u64,
            new_client_state: Array<felt252>,
            new_consensus_state: Array<felt252>,
        ) {}
    }

    // -----------------------------------------------------------
    // Client Owner
    // -----------------------------------------------------------

    #[generate_trait]
    pub(crate) impl ClientOwnerImpl<
        TContractState,
        +HasComponent<TContractState>,
        +Drop<TContractState>,
        impl Ownable: OwnableComponent::HasComponent<TContractState>,
    > of ClientOwnerTrait<TContractState> {
        fn owner(self: @ComponentState<TContractState>) -> ContractAddress {
            get_dep_component!(self, Ownable).owner()
        }

        fn assert_owner(self: @ComponentState<TContractState>) {
            assert(self.owner() == get_caller_address(), CometErrors::INVALID_OWNER);
        }
    }

    // -----------------------------------------------------------
    // Client Internal
    // -----------------------------------------------------------

    #[generate_trait]
    pub(crate) impl ClientInternalImpl<
        TContractState, +HasComponent<TContractState>, +Drop<TContractState>,
    > of ClientInternalTrait<TContractState> {
        fn _status(
            self: @ComponentState<TContractState>,
            client_state: CometClientState,
            consensus_state: CometConsensusState,
            client_sequence: u64,
        ) -> Status {
            if !client_state.status.is_active() {
                return client_state.status;
            }

            let consensus_state_status = consensus_state
                .status(client_state.trusting_period, client_state.max_clock_drift);

            if !consensus_state_status.is_active() {
                return consensus_state_status;
            }

            Status::Active
        }

        fn _update_state(
            ref self: ComponentState<TContractState>,
            client_sequence: u64,
            update_height: Height,
            client_state: CometClientState,
            consensus_state: CometConsensusState,
        ) {
            self.write_client_state(client_sequence, client_state.clone());

            self.write_update_height(client_sequence, update_height.clone());

            self
                .write_consensus_state(
                    client_sequence, update_height.clone(), consensus_state.clone(),
                );

            let host_height = get_block_number();

            self.write_client_processed_height(client_sequence, update_height.clone(), host_height);

            let host_timestamp = get_block_timestamp();

            self
                .write_client_processed_time(
                    client_sequence, update_height.clone(), host_timestamp,
                );

            self.write_next_client_sequence(client_sequence + 1);
        }
    }

    // -----------------------------------------------------------
    // Client Reader/Writer
    // -----------------------------------------------------------

    #[generate_trait]
    pub(crate) impl ClientReaderImpl<
        TContractState, +HasComponent<TContractState>, +Drop<TContractState>,
    > of ClientReaderTrait<TContractState> {
        fn read_next_client_sequence(self: @ComponentState<TContractState>) -> u64 {
            self.next_client_sequence.read()
        }

        fn read_client_state(
            self: @ComponentState<TContractState>, client_sequence: u64,
        ) -> CometClientState {
            let client_state = self.client_states.read(client_sequence);

            assert(client_state.is_non_zero(), CometErrors::MISSING_CLIENT_STATE);

            client_state
        }

        fn read_consensus_state(
            self: @ComponentState<TContractState>, client_sequence: u64, height: Height,
        ) -> CometConsensusState {
            let consensus_state = self.consensus_states.read((client_sequence, height));

            assert(consensus_state.is_non_zero(), CometErrors::MISSING_CONSENSUS_STATE);

            consensus_state
        }

        fn consensus_state_exists(
            self: @ComponentState<TContractState>, client_sequence: u64, height: Height,
        ) -> bool {
            self.consensus_states.read((client_sequence, height)).is_non_zero()
        }

        fn read_client_processed_time(
            self: @ComponentState<TContractState>, client_sequence: u64, height: Height,
        ) -> Timestamp {
            let processed_time = self.client_processed_times.read((client_sequence, height));

            assert(processed_time.is_non_zero(), CometErrors::MISSING_CLIENT_PROCESSED_TIME);

            processed_time.into()
        }

        fn read_client_processed_height(
            self: @ComponentState<TContractState>, client_sequence: u64, height: Height,
        ) -> u64 {
            let processed_height = self.client_processed_heights.read((client_sequence, height));

            assert(processed_height.is_non_zero(), CometErrors::MISSING_CLIENT_PROCESSED_HEIGHT);

            processed_height
        }

        fn read_update_heights(
            self: @ComponentState<TContractState>, client_sequence: u64,
        ) -> Array<Height> {
            self.update_heights.read(client_sequence)
        }
    }

    #[generate_trait]
    pub(crate) impl ClientWriterImpl<
        TContractState, +HasComponent<TContractState>, +Drop<TContractState>,
    > of ClientWriterTrait<TContractState> {
        fn write_next_client_sequence(
            ref self: ComponentState<TContractState>, client_sequence: u64,
        ) {
            self.next_client_sequence.write(client_sequence);
        }

        fn write_update_height(
            ref self: ComponentState<TContractState>, client_sequence: u64, update_height: Height,
        ) {
            let mut update_heights = self.update_heights.read(client_sequence);

            if update_heights.contains(@update_height) {
                return;
            }

            let len = update_heights.len();

            if len == 100 {
                update_heights.pop_front().unwrap();
            }

            update_heights.append(update_height);

            let new_update_heights = if len.is_non_zero()
                && update_heights.at(len - 1) > @update_height {
                MergeSort::sort(update_heights.span())
            } else {
                update_heights
            };

            self.update_heights.write(client_sequence, new_update_heights);
        }

        fn write_client_state(
            ref self: ComponentState<TContractState>,
            client_sequence: u64,
            client_state: CometClientState,
        ) {
            self.client_states.write(client_sequence, client_state);
        }

        fn write_consensus_state(
            ref self: ComponentState<TContractState>,
            client_sequence: u64,
            height: Height,
            consensus_state: CometConsensusState,
        ) {
            self.consensus_states.write((client_sequence, height), consensus_state);
        }

        fn write_client_processed_time(
            ref self: ComponentState<TContractState>,
            client_sequence: u64,
            height: Height,
            timestamp: u64,
        ) {
            self.client_processed_times.write((client_sequence, height), timestamp);
        }

        fn write_client_processed_height(
            ref self: ComponentState<TContractState>,
            client_sequence: u64,
            height: Height,
            host_height: u64,
        ) {
            self.client_processed_heights.write((client_sequence, height), host_height);
        }
    }
}
