#[starknet::component]
pub mod ICS07ClientComponent {
    use core::num::traits::zero::Zero;
    use core::traits::Into;
    use starknet::{get_block_timestamp, get_block_number};
    use starknet_ibc::clients::tendermint::{
        TendermintClientState, TendermintClientStateImpl, TendermintConsensusState,
        TendermintConsensusStateImpl, TendermintHeader, TendermintHeaderImpl,
        TendermintConsensusStateTrait, TendermintClientStateTrait, ICS07Errors
    };
    use starknet_ibc::core::client::{
        MsgCreateClient, MsgUpdateClient, MsgRecoverClient, MsgUpgradeClient, Height, Timestamp,
        Status, StatusTrait, UpdateResult, IClientHandler, IClientState, IClientStateValidation,
        IClientStateExecution
    };
    use starknet_ibc::core::host::ClientId;

    #[storage]
    struct Storage {
        client_states: LegacyMap<u64, TendermintClientState>,
        consensus_states: LegacyMap<(u64, Height), TendermintConsensusState>,
        client_processed_times: LegacyMap<(u64, Height), u64>,
        client_processed_heights: LegacyMap<(u64, Height), u64>,
    }

    #[event]
    #[derive(Debug, Drop, starknet::Event)]
    pub enum Event {}

    #[embeddable_as(ICS07ClientHandler)]
    impl ClientHandlerImpl<
        TContractState, +HasComponent<TContractState>, +Drop<TContractState>
    > of IClientHandler<ComponentState<TContractState>> {
        fn create(
            ref self: ComponentState<TContractState>, msg: MsgCreateClient, client_sequence: u64,
        ) {
            self.create_validate(msg.clone(), client_sequence);
            self.create_execute(msg, client_sequence);
        }

        fn update(ref self: ComponentState<TContractState>, msg: MsgUpdateClient) -> UpdateResult {
            self.update_validate(msg.clone());
            self.update_execute(msg)
        }

        fn recover(ref self: ComponentState<TContractState>, msg: MsgRecoverClient) {}

        fn upgrade(ref self: ComponentState<TContractState>, msg: MsgUpgradeClient) {}
    }

    #[generate_trait]
    pub impl CreateClientImpl<
        TContractState, +HasComponent<TContractState>, +Drop<TContractState>
    > of CreateClientTrait<TContractState> {
        fn create_validate(
            self: @ComponentState<TContractState>, msg: MsgCreateClient, client_sequence: u64
        ) {
            assert(msg.client_type == self.client_type(), ICS07Errors::INVALID_CLIENT_TYPE);

            assert(!msg.client_state.is_empty(), ICS07Errors::EMPTY_CLIENT_STATE);

            assert(!msg.consensus_state.is_empty(), ICS07Errors::EMPTY_CONSENSUS_STATE);

            let status = self.status(msg.client_state, client_sequence);

            assert(status.is_active(), ICS07Errors::INACTIVE_CLIENT);
        }

        fn create_execute(
            ref self: ComponentState<TContractState>, msg: MsgCreateClient, client_sequence: u64
        ) {
            self.initialize(client_sequence, msg.client_state, msg.consensus_state);
        }
    }

    #[generate_trait]
    pub impl UpdateClientImpl<
        TContractState, +HasComponent<TContractState>, +Drop<TContractState>
    > of UpdateClientTrait<TContractState> {
        fn update_validate(self: @ComponentState<TContractState>, msg: MsgUpdateClient) {
            assert(
                msg.client_id.client_type == self.client_type(), ICS07Errors::INVALID_CLIENT_TYPE
            );

            assert(!msg.client_message.is_empty(), ICS07Errors::EMPTY_CLIENT_MESSAGE);

            let tendermint_client_state: TendermintClientState = self
                .client_states
                .read(msg.client_id.sequence);

            let status = self._status(tendermint_client_state, msg.client_id.sequence);

            assert(status.is_active(), ICS07Errors::INACTIVE_CLIENT);

            self.verify_client_message(msg.client_id.sequence, msg.client_message);
        }

        fn update_execute(
            ref self: ComponentState<TContractState>, msg: MsgUpdateClient
        ) -> UpdateResult {
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
        fn recover_validate(self: @ComponentState<TContractState>, msg: MsgRecoverClient) {}

        fn recover_execute(ref self: ComponentState<TContractState>, msg: MsgRecoverClient) {}
    }

    #[generate_trait]
    pub impl UpgradeClientImpl<
        TContractState, +HasComponent<TContractState>, +Drop<TContractState>
    > of UpgradeClientTrait<TContractState> {
        fn upgrade_validate(self: @ComponentState<TContractState>, msg: MsgUpgradeClient,) {}

        fn upgrade_execute(ref self: ComponentState<TContractState>, msg: MsgUpgradeClient,) {}
    }

    #[embeddable_as(ICS07ClientState)]
    impl ClientStateImpl<
        TContractState, +HasComponent<TContractState>, +Drop<TContractState>
    > of IClientState<ComponentState<TContractState>> {
        fn client_type(self: @ComponentState<TContractState>) -> felt252 {
            '07-tendermint'
        }

        fn latest_height(self: @ComponentState<TContractState>, client_sequence: u64) -> Height {
            let tendermint_client_state: TendermintClientState = self
                .client_states
                .read(client_sequence);

            tendermint_client_state.latest_height
        }

        fn status(
            self: @ComponentState<TContractState>,
            client_state: Array<felt252>,
            client_sequence: u64
        ) -> Status {
            let tendermint_client_state = TendermintClientStateImpl::deserialize(client_state);

            self._status(tendermint_client_state, client_sequence)
        }
    }

    #[embeddable_as(ICS07ClientValidation)]
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

    #[embeddable_as(ICS07ClientExecution)]
    impl ClientExecutionImpl<
        TContractState, +HasComponent<TContractState>, +Drop<TContractState>
    > of IClientStateExecution<ComponentState<TContractState>> {
        fn initialize(
            ref self: ComponentState<TContractState>,
            client_sequence: u64,
            client_state: Array<felt252>,
            consensus_state: Array<felt252>
        ) {
            let tendermint_client_state = TendermintClientStateImpl::deserialize(client_state);

            let tendermint_consensus_state = TendermintConsensusStateImpl::deserialize(
                consensus_state
            );

            self
                ._update_state(
                    client_sequence, tendermint_client_state, tendermint_consensus_state
                );
        }

        fn update_state(
            ref self: ComponentState<TContractState>,
            client_sequence: u64,
            client_message: Array<felt252>
        ) -> UpdateResult {
            let header: TendermintHeader = TendermintHeaderImpl::deserialize(client_message);

            let header_height = header.clone().trusted_height;

            // TODO: Implement consensus state pruning mechanism.

            let maybe_consensus_state = self
                .consensus_states
                .read((client_sequence, header_height.clone()));

            if maybe_consensus_state.root.is_zero() {
                let mut client_state = self.client_states.read(client_sequence);

                client_state.update(header_height.clone());

                let new_consensus_state: TendermintConsensusState = header.into();

                self._update_state(client_sequence, client_state, new_consensus_state);
            }

            array![header_height].into()
        }

        fn update_on_misbehaviour(
            ref self: ComponentState<TContractState>,
            client_sequence: u64,
            client_message: Array<felt252>
        ) -> UpdateResult {
            let header = TendermintHeaderImpl::deserialize(client_message);

            let mut client_state = self.client_states.read(client_sequence);

            client_state.freeze(header.trusted_height);

            UpdateResult::Misbehaviour
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
    pub(crate) impl ClientInternalImpl<
        TContractState, +HasComponent<TContractState>, +Drop<TContractState>
    > of ClientInternalTrait<TContractState> {
        fn _status(
            self: @ComponentState<TContractState>,
            client_state: TendermintClientState,
            client_sequence: u64
        ) -> Status {
            if !client_state.status.is_active() {
                return client_state.status;
            }

            let latest_consensus_state = self
                .consensus_states
                .read((client_sequence, client_state.latest_height));

            let host_timestamp = get_block_timestamp();

            let consensus_state_status = latest_consensus_state
                .status(host_timestamp, client_state.trusting_period,);

            if !consensus_state_status.is_active() {
                return consensus_state_status;
            }

            Status::Active
        }

        fn _update_state(
            ref self: ComponentState<TContractState>,
            client_sequence: u64,
            client_state: TendermintClientState,
            consensus_state: TendermintConsensusState,
        ) {
            self.client_states.write(client_sequence, client_state.clone());

            let latest_height = client_state.latest_height;

            self
                .consensus_states
                .write((client_sequence, latest_height.clone()), consensus_state.clone());

            let host_height = get_block_number();

            self
                .client_processed_heights
                .write((client_sequence, latest_height.clone()), host_height);

            let host_timestamp = get_block_timestamp();

            self.client_processed_times.write((client_sequence, latest_height), host_timestamp);
        }
    }
}
