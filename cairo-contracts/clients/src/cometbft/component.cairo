#[starknet::component]
pub mod CometClientComponent {
    use core::num::traits::zero::Zero;
    use starknet::storage::Map;
    use starknet::{get_block_timestamp, get_block_number};
    use starknet_ibc_clients::cometbft::{
        CometClientState, CometClientStateImpl, CometConsensusState, CometConsensusStateImpl,
        CometHeader, CometHeaderImpl, CometErrors
    };
    use starknet_ibc_core::client::{
        MsgCreateClient, MsgUpdateClient, MsgRecoverClient, MsgUpgradeClient, Height, Timestamp,
        Status, StatusTrait, CreateResponse, CreateResponseImpl, UpdateResponse, IClientHandler,
        IClientState, IClientStateValidation, IClientStateExecution
    };
    use starknet_ibc_core::host::ClientIdImpl;
    use starknet_ibc_utils::ValidateBasicTrait;

    #[storage]
    struct Storage {
        client_sequence: u64,
        client_states: Map<u64, CometClientState>,
        consensus_states: Map<(u64, Height), CometConsensusState>,
        client_processed_times: Map<(u64, Height), u64>,
        client_processed_heights: Map<(u64, Height), u64>,
    }

    #[event]
    #[derive(Debug, Drop, starknet::Event)]
    pub enum Event {}

    #[embeddable_as(CometClientHandler)]
    impl ClientHandlerImpl<
        TContractState, +HasComponent<TContractState>, +Drop<TContractState>
    > of IClientHandler<ComponentState<TContractState>> {
        fn create_client(
            ref self: ComponentState<TContractState>, msg: MsgCreateClient
        ) -> CreateResponse {
            self.create_validate(msg.clone());
            self.create_execute(msg)
        }

        fn update_client(
            ref self: ComponentState<TContractState>, msg: MsgUpdateClient
        ) -> UpdateResponse {
            self.update_validate(msg.clone());
            self.update_execute(msg)
        }

        fn recover_client(ref self: ComponentState<TContractState>, msg: MsgRecoverClient) {}

        fn upgrade_client(ref self: ComponentState<TContractState>, msg: MsgUpgradeClient) {}
    }

    #[generate_trait]
    pub impl CreateClientImpl<
        TContractState, +HasComponent<TContractState>, +Drop<TContractState>
    > of CreateClientTrait<TContractState> {
        fn create_validate(self: @ComponentState<TContractState>, msg: MsgCreateClient) {
            msg.validate_basic();

            assert(msg.client_type == self.client_type(), CometErrors::INVALID_CLIENT_TYPE);

            let client_sequence = self.read_client_sequence();

            let comet_client_state = CometClientStateImpl::deserialize(msg.client_state);

            let comet_consensus_state = CometConsensusStateImpl::deserialize(msg.consensus_state);

            let status = self._status(comet_client_state, comet_consensus_state, client_sequence);

            assert(status.is_active(), CometErrors::INACTIVE_CLIENT);
        }

        fn create_execute(
            ref self: ComponentState<TContractState>, msg: MsgCreateClient
        ) -> CreateResponse {
            let client_sequence = self.read_client_sequence();

            self.initialize(client_sequence, msg.client_state, msg.consensus_state)
        }
    }

    #[generate_trait]
    pub impl UpdateClientImpl<
        TContractState, +HasComponent<TContractState>, +Drop<TContractState>
    > of UpdateClientTrait<TContractState> {
        fn update_validate(self: @ComponentState<TContractState>, msg: MsgUpdateClient) {
            msg.validate_basic();

            assert(
                msg.client_id.client_type == self.client_type(), CometErrors::INVALID_CLIENT_TYPE
            );

            let client_sequence = msg.client_id.sequence;

            let comet_client_state: CometClientState = self.read_client_state(client_sequence);

            let latest_consensus_state = self
                .read_consensus_state(client_sequence, comet_client_state.latest_height.clone());

            assert(!latest_consensus_state.is_zero(), CometErrors::ZERO_CONSENSUS_STATE);

            let status = self
                ._status(comet_client_state, latest_consensus_state, msg.client_id.sequence);

            assert(status.is_active(), CometErrors::INACTIVE_CLIENT);

            self.verify_client_message(msg.client_id.sequence, msg.client_message);
        }

        fn update_execute(
            ref self: ComponentState<TContractState>, msg: MsgUpdateClient
        ) -> UpdateResponse {
            if self.verify_misbehaviour(msg.client_id.sequence, msg.client_message.clone()) {
                return self
                    .update_on_misbehaviour(msg.client_id.sequence, msg.client_message.clone());
            }

            self.update_state(msg.client_id.sequence, msg.client_message.clone())
        }
    }

    #[generate_trait]
    pub impl RecoverClientImpl<
        TContractState, +HasComponent<TContractState>, +Drop<TContractState>
    > of RecoverClientTrait<TContractState> {
        fn recover_validate(self: @ComponentState<TContractState>, msg: MsgRecoverClient) {
            msg.validate_basic();
        }

        fn recover_execute(ref self: ComponentState<TContractState>, msg: MsgRecoverClient) {}
    }

    #[generate_trait]
    pub impl UpgradeClientImpl<
        TContractState, +HasComponent<TContractState>, +Drop<TContractState>
    > of UpgradeClientTrait<TContractState> {
        fn upgrade_validate(self: @ComponentState<TContractState>, msg: MsgUpgradeClient) {
            msg.validate_basic();
        }

        fn upgrade_execute(ref self: ComponentState<TContractState>, msg: MsgUpgradeClient,) {}
    }

    #[embeddable_as(CometCommonClientState)]
    impl ClientStateImpl<
        TContractState, +HasComponent<TContractState>, +Drop<TContractState>
    > of IClientState<ComponentState<TContractState>> {
        fn client_type(self: @ComponentState<TContractState>) -> felt252 {
            '07-cometbft'
        }

        fn latest_height(self: @ComponentState<TContractState>, client_sequence: u64) -> Height {
            let comet_client_state: CometClientState = self.read_client_state(client_sequence);

            comet_client_state.latest_height
        }

        fn status(self: @ComponentState<TContractState>, client_sequence: u64) -> Status {
            let comet_client_state: CometClientState = self.read_client_state(client_sequence);

            let latest_consensus_state = self
                .read_consensus_state(client_sequence, comet_client_state.latest_height.clone());

            assert(!latest_consensus_state.is_zero(), CometErrors::ZERO_CONSENSUS_STATE);

            self._status(comet_client_state, latest_consensus_state, client_sequence)
        }
    }

    #[embeddable_as(CometClientValidation)]
    impl ClientValidationImpl<
        TContractState, +HasComponent<TContractState>, +Drop<TContractState>
    > of IClientStateValidation<ComponentState<TContractState>> {
        fn verify_client_message(
            self: @ComponentState<TContractState>,
            client_sequence: u64,
            client_message: Array<felt252>
        ) {}

        fn verify_misbehaviour(
            self: @ComponentState<TContractState>,
            client_sequence: u64,
            client_message: Array<felt252>
        ) -> bool {
            false
        }

        fn verify_substitute(
            self: @ComponentState<TContractState>, substitute_client_state: Array<felt252>
        ) {}

        fn verify_upgrade(
            self: @ComponentState<TContractState>,
            upgrade_client_state: Array<felt252>,
            upgrade_consensus_state: Array<felt252>,
            proof_upgrade_client: Array<felt252>,
            proof_upgrade_consensus: Array<felt252>,
            root: felt252
        ) {}
    }

    #[embeddable_as(CometClientExecution)]
    impl ClientExecutionImpl<
        TContractState, +HasComponent<TContractState>, +Drop<TContractState>
    > of IClientStateExecution<ComponentState<TContractState>> {
        fn initialize(
            ref self: ComponentState<TContractState>,
            client_sequence: u64,
            client_state: Array<felt252>,
            consensus_state: Array<felt252>
        ) -> CreateResponse {
            let comet_client_state = CometClientStateImpl::deserialize(client_state);

            let comet_consensus_state = CometConsensusStateImpl::deserialize(consensus_state);

            self._update_state(client_sequence, comet_client_state.clone(), comet_consensus_state);

            let client_id = ClientIdImpl::new(self.client_type(), client_sequence);

            CreateResponseImpl::new(client_id, comet_client_state.latest_height)
        }

        fn update_state(
            ref self: ComponentState<TContractState>,
            client_sequence: u64,
            client_message: Array<felt252>
        ) -> UpdateResponse {
            let header: CometHeader = CometHeaderImpl::deserialize(client_message);

            let header_height = header.clone().signed_header.height;

            // TODO: Implement consensus state pruning mechanism.

            let maybe_consensus_state = self
                .read_consensus_state(client_sequence, header_height.clone());

            if maybe_consensus_state.is_zero() {
                let mut client_state = self.read_client_state(client_sequence);

                client_state.update(header_height.clone());

                let new_consensus_state: CometConsensusState = header.into();

                self._update_state(client_sequence, client_state, new_consensus_state);
            }

            array![header_height].into()
        }

        fn update_on_misbehaviour(
            ref self: ComponentState<TContractState>,
            client_sequence: u64,
            client_message: Array<felt252>
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
            substitute_consensus_state: Array<felt252>
        ) {}

        fn update_on_upgrade(
            ref self: ComponentState<TContractState>,
            client_sequence: u64,
            new_client_state: Array<felt252>,
            new_consensus_state: Array<felt252>
        ) {}
    }

    #[generate_trait]
    impl ClientInternalImpl<
        TContractState, +HasComponent<TContractState>, +Drop<TContractState>
    > of ClientInternalTrait<TContractState> {
        fn _status(
            self: @ComponentState<TContractState>,
            client_state: CometClientState,
            consensus_state: CometConsensusState,
            client_sequence: u64
        ) -> Status {
            if !client_state.status.is_active() {
                return client_state.status;
            }

            let host_timestamp = get_block_timestamp();

            let consensus_state_status = consensus_state
                .status(host_timestamp, client_state.trusting_period);

            if !consensus_state_status.is_active() {
                return consensus_state_status;
            }

            Status::Active
        }

        fn _update_state(
            ref self: ComponentState<TContractState>,
            client_sequence: u64,
            client_state: CometClientState,
            consensus_state: CometConsensusState,
        ) {
            self.write_client_state(client_sequence, client_state.clone());

            let latest_height = client_state.latest_height;

            self
                .write_consensus_state(
                    client_sequence, latest_height.clone(), consensus_state.clone()
                );

            let host_height = get_block_number();

            self.write_client_processed_height(client_sequence, latest_height.clone(), host_height);

            let host_timestamp = get_block_timestamp();

            self
                .write_client_processed_time(
                    client_sequence, latest_height.clone(), host_timestamp
                );
        }
    }

    #[generate_trait]
    impl ClientReaderImpl<
        TContractState, +HasComponent<TContractState>, +Drop<TContractState>
    > of ClientReaderTrait<TContractState> {
        fn read_client_sequence(self: @ComponentState<TContractState>) -> u64 {
            self.client_sequence.read()
        }

        fn read_client_state(
            self: @ComponentState<TContractState>, client_sequence: u64
        ) -> CometClientState {
            self.client_states.read(client_sequence)
        }

        fn read_consensus_state(
            self: @ComponentState<TContractState>, client_sequence: u64, height: Height
        ) -> CometConsensusState {
            self.consensus_states.read((client_sequence, height))
        }

        fn read_client_processed_time(
            self: @ComponentState<TContractState>, client_sequence: u64, height: Height
        ) -> Timestamp {
            self.client_processed_times.read((client_sequence, height)).into()
        }

        fn read_client_processed_height(
            self: @ComponentState<TContractState>, client_sequence: u64, height: Height
        ) -> u64 {
            self.client_processed_heights.read((client_sequence, height))
        }
    }

    #[generate_trait]
    impl ClientWriterImpl<
        TContractState, +HasComponent<TContractState>, +Drop<TContractState>
    > of ClientWriterTrait<TContractState> {
        fn write_client_sequence(ref self: ComponentState<TContractState>, client_sequence: u64) {
            self.client_sequence.write(client_sequence);
        }

        fn write_client_state(
            ref self: ComponentState<TContractState>,
            client_sequence: u64,
            client_state: CometClientState
        ) {
            self.client_states.write(client_sequence, client_state);
        }

        fn write_consensus_state(
            ref self: ComponentState<TContractState>,
            client_sequence: u64,
            height: Height,
            consensus_state: CometConsensusState
        ) {
            self.consensus_states.write((client_sequence, height), consensus_state);
        }

        fn write_client_processed_time(
            ref self: ComponentState<TContractState>,
            client_sequence: u64,
            height: Height,
            timestamp: u64
        ) {
            self.client_processed_times.write((client_sequence, height), timestamp);
        }

        fn write_client_processed_height(
            ref self: ComponentState<TContractState>,
            client_sequence: u64,
            height: Height,
            host_height: u64
        ) {
            self.client_processed_heights.write((client_sequence, height), host_height);
        }
    }
}
