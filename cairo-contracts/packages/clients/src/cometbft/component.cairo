#[starknet::component]
pub mod CometClientComponent {
    use cometbft::types::{Options, TrustedBlockState, UntrustedBlockState};
    use core::num::traits::Zero;
    use ibc_utils::array::span_contains;
    use ibc_utils::bytes::ByteArrayIntoArrayU8;
    use ibc_utils::storage::{ArrayFelt252Store, read_raw_key};
    use ics23::Proof;
    use openzeppelin_access::ownable::OwnableComponent;
    use openzeppelin_access::ownable::OwnableComponent::InternalTrait;
    use starknet::storage::{
        Map, StorageMapReadAccess, StorageMapWriteAccess, StoragePointerReadAccess,
        StoragePointerWriteAccess,
    };
    use starknet::{get_block_number, get_block_timestamp};
    use starknet_ibc_clients::cometbft::{
        ClientMessage, CometClientState, CometClientStateImpl, CometConsensusState,
        CometConsensusStateImpl, CometConsensusStateZero, CometErrors, CometHeader, CometHeaderImpl,
        CometHeaderIntoConsensusState, Misbehaviour, MisbehaviourImpl,
    };
    use starknet_ibc_core::client::{
        CreateResponse, CreateResponseImpl, Height, HeightImpl, HeightPartialOrd, HeightZero,
        IClientHandler, IClientQuery, IClientStateExecution, IClientStateValidation,
        MsgCreateClient, MsgRecoverClient, MsgUpdateClient, MsgUpgradeClient, Status, StatusTrait,
        StoreHeightArray, Timestamp, TimestampImpl, TimestampPartialOrd, TimestampToProto,
        UpdateResponse,
    };
    use starknet_ibc_core::commitment::{StateProof, StateRoot, StateValue};
    use starknet_ibc_core::host::{
        BasePrefix, ClientIdImpl, client_upgrade_path, consensus_upgrade_path,
    };
    use starknet_ibc_libs::comet::{ICometDispatcherTrait, ICometLibraryDispatcher};
    use starknet_ibc_libs::ics23::{IIcs23DispatcherTrait, IIcs23LibraryDispatcher};
    use starknet_ibc_libs::protobuf::{IProtobufDispatcherTrait, IProtobufLibraryDispatcher};
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

    #[derive(Serde, Drop)]
    pub struct MessageWithHint {
        client_message: ClientMessage,
        serialized_hints: Array<felt252>,
    }

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
            {
                let ownable = get_dep_component!(@self, Ownable);
                ownable.assert_only_owner();
            }
            let client_sequence = self.read_next_client_sequence();
            self.create_validate(client_sequence, @msg);
            self.create_execute(client_sequence, msg)
        }

        fn update_client(
            ref self: ComponentState<TContractState>, msg: MsgUpdateClient,
        ) -> UpdateResponse {
            self.update_validate(@msg);
            self.update_execute(msg)
        }

        fn recover_client(ref self: ComponentState<TContractState>, msg: MsgRecoverClient) {
            {
                let ownable = get_dep_component!(@self, Ownable);
                ownable.assert_only_owner();
            }
            self.recover_validate(msg.clone());
            self.recover_execute(msg)
        }

        fn upgrade_client(ref self: ComponentState<TContractState>, msg: MsgUpgradeClient) {
            self.upgrade_validate(msg.clone());
            self.upgrade_execute(msg);
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
        ) -> Option<Height> {
            let update_heights = self.read_update_heights(client_sequence);

            let mut len = update_heights.len();

            assert(len > 0, CometErrors::ZERO_UPDATE_HEIGHTS);

            // FIXME: do binary search

            let mut update_heights_span = update_heights.span();

            let mut result = None;

            while let Some(update_height) = update_heights_span.pop_back() {
                if @target_height >= update_height {
                    result = Some(*update_height);
                    break;
                }
            }

            result
        }

        fn update_height_after(
            self: @ComponentState<TContractState>, client_sequence: u64, target_height: Height,
        ) -> Option<Height> {
            let update_heights = self.read_update_heights(client_sequence);

            let mut len = update_heights.len();

            assert(len > 0, CometErrors::ZERO_UPDATE_HEIGHTS);

            // FIXME: do binary search

            let mut update_heights_span = update_heights.span();

            let mut result = None;

            while let Some(update_height) = update_heights_span.pop_front() {
                if @target_height <= update_height {
                    result = Some(*update_height);
                    break;
                }
            }

            result
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
                .read_consensus_state(client_sequence, comet_client_state.latest_height);

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
            let mut serialized: Array<felt252> = ArrayTrait::new();

            let consensus_state: CometConsensusState = self
                .read_consensus_state(client_sequence, height);

            consensus_state.serialize(ref serialized);

            serialized
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
            self: @ComponentState<TContractState>, client_sequence: u64, msg: @MsgCreateClient,
        ) {
            msg.validate_basic();

            assert(msg.client_type == @self.client_type(), CometErrors::INVALID_CLIENT_TYPE);

            let comet_client_state = CometClientStateImpl::deserialize(msg.client_state.clone());

            let comet_consensus_state = CometConsensusStateImpl::deserialize(
                msg.consensus_state.clone(),
            );

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
        fn update_validate(self: @ComponentState<TContractState>, msg: @MsgUpdateClient) {
            msg.validate_basic();

            assert(
                msg.client_id.client_type == @self.client_type(), CometErrors::INVALID_CLIENT_TYPE,
            );

            let client_sequence = *msg.client_id.sequence;

            let comet_client_state: CometClientState = self.read_client_state(client_sequence);

            let latest_consensus_state = self
                .read_consensus_state(client_sequence, comet_client_state.latest_height);

            let status = self._status(comet_client_state, latest_consensus_state, client_sequence);

            assert(status.is_active(), CometErrors::INACTIVE_CLIENT);

            self.verify_client_message(client_sequence, msg.client_message.clone());
        }

        fn update_execute(
            ref self: ComponentState<TContractState>, msg: MsgUpdateClient,
        ) -> UpdateResponse {
            let client_sequence = msg.client_id.sequence;

            if self.verify_misbehaviour(client_sequence, msg.client_message.clone()) {
                self.update_on_misbehaviour(client_sequence, msg.client_message.clone())
            } else {
                self.update_state(client_sequence, msg.client_message)
            }
        }
    }

    #[generate_trait]
    pub(crate) impl RecoverClientImpl<
        TContractState, +HasComponent<TContractState>, +Drop<TContractState>,
    > of RecoverClientTrait<TContractState> {
        fn recover_validate(self: @ComponentState<TContractState>, msg: MsgRecoverClient) {
            msg.validate_basic();

            // TODO: validate signer once assert_owner() has been removed

            let subject_client_sequence = msg.subject_client_id.sequence;
            let substitute_client_sequence = msg.substitute_client_id.sequence;

            let subject_client_state: CometClientState = self
                .read_client_state(subject_client_sequence);
            let substitute_client_state: CometClientState = self
                .read_client_state(substitute_client_sequence);

            let subject_consensus_state = self
                .read_consensus_state(subject_client_sequence, subject_client_state.latest_height);
            let substitute_consensus_state = self
                .read_consensus_state(
                    substitute_client_sequence, substitute_client_state.latest_height,
                );

            let subject_status = self
                ._status(
                    subject_client_state.clone(), subject_consensus_state, subject_client_sequence,
                );
            let substitute_status = self
                ._status(
                    substitute_client_state.clone(),
                    substitute_consensus_state,
                    substitute_client_sequence,
                );

            assert(
                subject_client_state.latest_height < substitute_client_state.latest_height,
                CometErrors::INVALID_CLIENT_SUBSTITUTE,
            );

            assert(
                subject_status.is_expired() | subject_status.is_frozen(),
                CometErrors::ACTIVE_CLIENT,
            );
            assert(substitute_status.is_active(), CometErrors::INACTIVE_CLIENT);

            assert(
                subject_client_state.substitute_client_matches(substitute_client_state),
                CometErrors::INVALID_CLIENT_SUBSTITUTE,
            );
        }

        fn recover_execute(ref self: ComponentState<TContractState>, msg: MsgRecoverClient) {
            let subject_client_sequence = msg.subject_client_id.sequence;
            let substitute_client_sequence = msg.substitute_client_id.sequence;

            let substitute_client_state: CometClientState = self
                .read_client_state(substitute_client_sequence);

            let substitute_consensus_state = self
                .read_consensus_state(
                    substitute_client_sequence, substitute_client_state.latest_height,
                );

            let mut serialised_client_state = array![];
            let mut serialised_consensus_state = array![];
            substitute_client_state.serialize(ref serialised_client_state);
            substitute_consensus_state.serialize(ref serialised_consensus_state);

            self
                .update_on_recover(
                    subject_client_sequence,
                    substitute_client_sequence,
                    serialised_client_state,
                    serialised_consensus_state,
                );
        }
    }

    #[generate_trait]
    pub(crate) impl UpgradeClientImpl<
        TContractState, +HasComponent<TContractState>, +Drop<TContractState>,
    > of UpgradeClientTrait<TContractState> {
        fn upgrade_validate(self: @ComponentState<TContractState>, msg: MsgUpgradeClient) {
            msg.validate_basic();

            self
                .verify_upgrade(
                    msg.client_id.sequence,
                    msg.upgraded_client_state,
                    msg.upgraded_consensus_state,
                    msg.proof_upgrade_client,
                    msg.proof_upgrade_consensus,
                );
        }

        fn upgrade_execute(ref self: ComponentState<TContractState>, msg: MsgUpgradeClient) {
            self
                .update_on_upgrade(
                    msg.client_id.sequence, msg.upgraded_client_state, msg.upgraded_consensus_state,
                );
        }
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
            paths: Array<ByteArray>,
            value: StateValue,
            proof: StateProof,
            root: StateRoot,
        ) {
            let decoded_proof = IProtobufLibraryDispatcher {
                class_hash: read_raw_key::<'protobuf-library'>(),
            }
                .merkle_proof_decode(proof.proof);

            let specs = self.read_client_state(client_sequence).proof_spec;
            let mut proofs: Array<Proof> = ArrayTrait::new();
            for proof in decoded_proof.proofs {
                proofs.append(proof.proof);
            }
            let root = root.root;
            let mut keys = array![];
            for path in paths {
                let path_bytes = ByteArrayIntoArrayU8::into(path);
                keys.append(path_bytes);
            }
            let value = value.value;

            IIcs23LibraryDispatcher { class_hash: read_raw_key::<'ics23-library'>() }
                .verify_membership(specs, proofs, root, keys, value, 0);
        }

        fn verify_non_membership(
            self: @ComponentState<TContractState>,
            client_sequence: u64,
            paths: Array<ByteArray>,
            proof: StateProof,
            root: StateRoot,
        ) {
            let decoded_proof = IProtobufLibraryDispatcher {
                class_hash: read_raw_key::<'protobuf-library'>(),
            }
                .merkle_proof_decode(proof.proof);
            let specs = self.read_client_state(client_sequence).proof_spec;
            let mut proofs: Array<Proof> = ArrayTrait::new();
            for proof in decoded_proof.proofs {
                proofs.append(proof.proof);
            }
            let root = root.root;
            let mut keys = array![];
            for path in paths {
                let path_bytes = ByteArrayIntoArrayU8::into(path);
                keys.append(path_bytes);
            }

            IIcs23LibraryDispatcher { class_hash: read_raw_key::<'ics23-library'>() }
                .verify_non_membership(specs, proofs, root, keys);
        }

        fn verify_client_message(
            self: @ComponentState<TContractState>,
            client_sequence: u64,
            client_message: Array<felt252>,
        ) {
            let mut span = client_message.span();

            let MessageWithHint {
                client_message, serialized_hints,
            } = Serde::<MessageWithHint>::deserialize(ref span).unwrap();

            let mut serialized_hints = serialized_hints.span();

            match client_message {
                ClientMessage::Update(message) => {
                    let header: CometHeader = CometHeaderImpl::deserialize(message);
                    let hints: Array<Array<felt252>> = Serde::deserialize(ref serialized_hints)
                        .unwrap();
                    self._verify_update_header(client_sequence, header, hints);
                },
                ClientMessage::Misbehaviour(message) => {
                    let misbehaviour: Misbehaviour = MisbehaviourImpl::deserialize(message);
                    let (hints_1, hints_2) = Serde::deserialize(ref serialized_hints).unwrap();
                    misbehaviour.validate_basic();
                    self
                        ._verify_misbehaviour_header(
                            client_sequence, misbehaviour.header_1, hints_1,
                        );
                    self
                        ._verify_misbehaviour_header(
                            client_sequence, misbehaviour.header_2, hints_2,
                        );
                },
            }
        }

        fn verify_misbehaviour(
            self: @ComponentState<TContractState>,
            client_sequence: u64,
            client_message: Array<felt252>,
        ) -> bool {
            let mut span = client_message.span();

            let MessageWithHint {
                client_message, ..,
            } = Serde::<MessageWithHint>::deserialize(ref span).unwrap();

            match client_message {
                ClientMessage::Update(message) => {
                    let header: CometHeader = CometHeaderImpl::deserialize(message);
                    self._verify_misbehaviour_on_update(client_sequence, header)
                },
                ClientMessage::Misbehaviour(message) => {
                    let misbehaviour: Misbehaviour = MisbehaviourImpl::deserialize(message);
                    misbehaviour.verify()
                },
            }
        }

        fn verify_substitute(
            self: @ComponentState<TContractState>, substitute_client_state: Array<felt252>,
        ) {}

        fn verify_upgrade(
            self: @ComponentState<TContractState>,
            client_sequence: u64,
            upgrade_client_state: Array<felt252>,
            upgrade_consensus_state: Array<felt252>,
            proof_upgrade_client: StateProof,
            proof_upgrade_consensus: StateProof,
        ) {
            let comet_client_state: CometClientState = self.read_client_state(client_sequence);
            let latest_height = comet_client_state.latest_height;

            let latest_consensus_state = self
                .read_consensus_state(client_sequence, comet_client_state.latest_height);

            let root = latest_consensus_state.root.clone();

            let upgrade_path = comet_client_state.upgrade_path.clone();

            let status = self._status(comet_client_state, latest_consensus_state, client_sequence);

            assert(status.is_active(), CometErrors::INACTIVE_CLIENT);

            assert(
                upgrade_path.len() == 1 || upgrade_path.len() == 2,
                CometErrors::INVALID_UPGRADE_PATH_LENGTH,
            );

            let (prefix, upgrade_path) = if upgrade_path.len() == 1 {
                ("", upgrade_path[0].clone())
            } else {
                (upgrade_path[0].clone(), upgrade_path[1].clone())
            };

            let base_prefix = BasePrefix { prefix };

            let upgraded_client_path = client_upgrade_path(
                base_prefix.clone(), latest_height.revision_height, upgrade_path.clone(),
            );

            let upgraded_consensus_path = consensus_upgrade_path(
                base_prefix, latest_height.revision_height, upgrade_path,
            );

            let upgraded_client_state = CometClientStateImpl::deserialize(upgrade_client_state);

            let upgraded_consensus_state = CometConsensusStateImpl::deserialize(
                upgrade_consensus_state,
            );

            let upgraded_height = upgraded_client_state.latest_height;

            assert(upgraded_height > latest_height, CometErrors::INVALID_UPGRADE_HEIGHT);

            let upgraded_client_protobuf = StateValue {
                value: upgraded_client_state.protobuf_bytes(),
            };
            let upgraded_consensus_protobuf = StateValue {
                value: upgraded_consensus_state.protobuf_bytes(),
            };

            self
                .verify_membership(
                    client_sequence,
                    upgraded_client_path,
                    upgraded_client_protobuf,
                    proof_upgrade_client,
                    root.clone(),
                );

            self
                .verify_membership(
                    client_sequence,
                    upgraded_consensus_path,
                    upgraded_consensus_protobuf,
                    proof_upgrade_consensus,
                    root,
                );
        }
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

            let comet_client_state_latest_height = comet_client_state.latest_height;

            self
                ._update_state(
                    client_sequence,
                    comet_client_state.latest_height,
                    comet_client_state,
                    comet_consensus_state,
                    get_block_number(),
                    get_block_timestamp(),
                );

            self.write_next_client_sequence(client_sequence + 1);

            let client_id = ClientIdImpl::new(self.client_type(), client_sequence);

            CreateResponseImpl::new(client_id, comet_client_state_latest_height)
        }

        fn update_state(
            ref self: ComponentState<TContractState>,
            client_sequence: u64,
            client_message: Array<felt252>,
        ) -> UpdateResponse {
            let latest_height = self.latest_height(client_sequence);

            let header = {
                let mut span = client_message.span();

                let client_message = Serde::<ClientMessage>::deserialize(ref span).unwrap();

                match client_message {
                    ClientMessage::Update(message) => {
                        let header: CometHeader = CometHeaderImpl::deserialize(message);
                        header
                    },
                    ClientMessage::Misbehaviour(_message) => {
                        core::panic_with_const_felt252::<CometErrors::INVALID_MISBEHAVIOUR>()
                    },
                }
            };

            let tm_header_height = header.clone().signed_header.header.height;

            let header_height = HeightImpl::new(latest_height.revision_number, tm_header_height);

            let update_heights = self.read_update_heights(client_sequence);
            let mut update_heights_span = update_heights.span();

            let mut client_state = self.read_client_state(client_sequence);

            let mut heights_kept = array![];
            let mut check_in_progress = true;
            // Since the Heights are sorted when stored, as soon as we find the first
            // Height which isn't expired we can stop checking the rest and build
            // the new Heights array which are kept.
            while let Option::Some(height) = update_heights_span.pop_front() {
                if check_in_progress {
                    let consensus_state = self.read_consensus_state(client_sequence, *height);

                    if consensus_state
                        .status(client_state.trusting_period, client_state.max_clock_drift)
                        .is_expired() {
                        self.remove_consensus_state(client_sequence, *height);
                    } else {
                        check_in_progress = false;
                        heights_kept.append(*height);
                    }
                } else {
                    heights_kept.append(*height);
                }
            }
            // Write directly since heights_kept is already sorted and is equal or
            // smaller to the previous one
            self.update_heights.write(client_sequence, heights_kept);

            if !self.consensus_state_exists(client_sequence, header_height) {
                client_state.update(header_height);

                let new_consensus_state: CometConsensusState = header.into();

                self
                    ._update_state(
                        client_sequence,
                        header_height,
                        client_state,
                        new_consensus_state,
                        get_block_number(),
                        get_block_timestamp(),
                    );
            }

            array![header_height].into()
        }

        fn update_on_misbehaviour(
            ref self: ComponentState<TContractState>,
            client_sequence: u64,
            client_message: Array<felt252>,
        ) -> UpdateResponse {
            let mut client_state = self.read_client_state(client_sequence);

            // convention from ibc-go
            let frozen_height = HeightImpl::new(0, 1);

            client_state.freeze(frozen_height);

            self.write_client_state(client_sequence, client_state);

            UpdateResponse::Misbehaviour
        }

        fn update_on_recover(
            ref self: ComponentState<TContractState>,
            subject_client_sequence: u64,
            substitute_client_sequence: u64,
            substitute_client_state: Array<felt252>,
            substitute_consensus_state: Array<felt252>,
        ) {
            let update_heights = self.read_update_heights(subject_client_sequence);

            if update_heights.len() > 0 {
                let mut update_heights_span = update_heights.span();

                while let Option::Some(height) = update_heights_span.pop_front() {
                    self.remove_consensus_state(subject_client_sequence, *height);
                }

                self.update_heights.write(subject_client_sequence, array![]);
            }

            let substitute_client_state = CometClientStateImpl::deserialize(
                substitute_client_state,
            );
            let substitute_consensus_state = CometConsensusStateImpl::deserialize(
                substitute_consensus_state,
            );

            let latest_height = substitute_client_state.latest_height;

            let processed_height = self
                .read_client_processed_height(substitute_client_sequence, latest_height);
            let processed_time = self
                .read_client_processed_time(substitute_client_sequence, latest_height);

            self
                ._update_state(
                    subject_client_sequence,
                    substitute_client_state.latest_height,
                    substitute_client_state,
                    substitute_consensus_state,
                    processed_height,
                    processed_time,
                );
        }

        fn update_on_upgrade(
            ref self: ComponentState<TContractState>,
            client_sequence: u64,
            new_client_state: Array<felt252>,
            new_consensus_state: Array<felt252>,
        ) {
            let update_heights = self.read_update_heights(client_sequence);

            if update_heights.len() > 0 {
                let mut update_heights_span = update_heights.span();

                while let Option::Some(height) = update_heights_span.pop_front() {
                    self.remove_consensus_state(client_sequence, *height);
                }

                self.update_heights.write(client_sequence, array![]);
            }

            let new_client_state = CometClientStateImpl::deserialize(new_client_state);
            let new_consensus_state = CometConsensusStateImpl::deserialize(new_consensus_state);

            self
                ._update_state(
                    client_sequence,
                    new_client_state.latest_height,
                    new_client_state,
                    new_consensus_state,
                    get_block_number(),
                    get_block_timestamp(),
                );
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
            processed_height: u64,
            processed_time: u64,
        ) {
            self.write_client_state(client_sequence, client_state);

            self.write_update_height(client_sequence, update_height);

            self
                .write_consensus_state(
                    client_sequence,
                    update_height,
                    consensus_state,
                    processed_height,
                    processed_time,
                );
        }

        fn _verify_update_header(
            self: @ComponentState<TContractState>,
            client_sequence: u64,
            header: CometHeader,
            signature_hints: Array<Array<felt252>>,
        ) {
            let trusted_height = header.trusted_height;

            let ibc_trusted_height = HeightImpl::new(
                trusted_height.revision_number, trusted_height.revision_height,
            );

            let client_state = self.read_client_state(client_sequence);

            let trusted_consensus_state = self
                .read_consensus_state(client_sequence, ibc_trusted_height);

            let trusted_block_state = TrustedBlockState {
                chain_id: client_state.chain_id,
                header_time: trusted_consensus_state.timestamp.try_into().unwrap(),
                height: trusted_height.revision_height,
                next_validators: header.trusted_validator_set.clone(),
                next_validators_hash: trusted_consensus_state.next_validators_hash,
            };

            let untrusted_block_state = UntrustedBlockState {
                signed_header: header.signed_header,
                validators: header.validator_set,
                next_validators: header.trusted_validator_set,
            };

            let trust_threshold = client_state.trust_level;
            let trusting_period = client_state.trusting_period.try_into().unwrap();
            let clock_drift = client_state.max_clock_drift.try_into().unwrap();
            let now = TimestampImpl::host().try_into().unwrap();

            let options = Options { trust_threshold, trusting_period, clock_drift };

            let mut hints_context = array![];
            Serde::serialize(
                @(client_state.attestator_quorum_percentage, client_state.attestator_keys),
                ref hints_context,
            );

            ICometLibraryDispatcher { class_hash: read_raw_key::<'comet-library'>() }
                .verify_update_header(
                    untrusted_block_state,
                    trusted_block_state,
                    options,
                    now,
                    hints_context.span(),
                    signature_hints.span(),
                );
        }


        fn _verify_misbehaviour_header(
            self: @ComponentState<TContractState>,
            client_sequence: u64,
            header: CometHeader,
            signature_hints: Array<Array<felt252>>,
        ) {
            let trusted_height = header.trusted_height;

            let ibc_trusted_height = HeightImpl::new(
                trusted_height.revision_number, trusted_height.revision_height,
            );

            let client_state = self.read_client_state(client_sequence);

            let trusted_consensus_state = self
                .read_consensus_state(client_sequence, ibc_trusted_height);

            let trusted_block_state = TrustedBlockState {
                chain_id: client_state.chain_id,
                header_time: trusted_consensus_state.timestamp.try_into().unwrap(),
                height: trusted_height.revision_height,
                next_validators: header.trusted_validator_set.clone(),
                next_validators_hash: trusted_consensus_state.next_validators_hash,
            };

            let untrusted_block_state = UntrustedBlockState {
                signed_header: header.signed_header,
                validators: header.validator_set,
                next_validators: header.trusted_validator_set,
            };

            let trust_threshold = client_state.trust_level;
            let trusting_period = client_state.trusting_period.try_into().unwrap();
            let clock_drift = client_state.max_clock_drift.try_into().unwrap();
            let now = TimestampImpl::host().try_into().unwrap();

            let options = Options { trust_threshold, trusting_period, clock_drift };

            let mut hints_context = array![];
            Serde::serialize(
                @(client_state.attestator_quorum_percentage, client_state.attestator_keys),
                ref hints_context,
            );

            ICometLibraryDispatcher { class_hash: read_raw_key::<'comet-library'>() }
                .verify_misbehaviour_header(
                    untrusted_block_state,
                    trusted_block_state,
                    options,
                    now,
                    hints_context.span(),
                    signature_hints.span(),
                )
        }

        fn _verify_misbehaviour_on_update(
            self: @ComponentState<TContractState>, client_sequence: u64, header: CometHeader,
        ) -> bool {
            let client_state = self.read_client_state(client_sequence);

            let target_height = HeightImpl::new(
                client_state.latest_height.revision_number, header.signed_header.header.height,
            );

            let target_consensus_state: CometConsensusState = header.into();

            let previous_height = self.update_height_before(client_sequence, target_height);

            if previous_height == Some(target_height) {
                let stored_consensus_state = self
                    .read_consensus_state(client_sequence, target_height);

                // stored consensus state should be the same from target consensus state
                // negation of the correct condition is a misbehaviour case
                if !(stored_consensus_state == target_consensus_state) {
                    return true;
                }
            } else {
                if let Some(previous_height) = previous_height {
                    let previous_consensus_state = self
                        .read_consensus_state(client_sequence, previous_height);

                    // time should be monotonically increasing
                    // negation of the correct condition is a misbehaviour case
                    if !(@previous_consensus_state.timestamp < @target_consensus_state.timestamp) {
                        return true;
                    }
                }

                let next_height = self.update_height_after(client_sequence, target_height);

                if let Some(next_height) = next_height {
                    let next_consensus_state = self
                        .read_consensus_state(client_sequence, next_height);

                    // time should be monotonically increasing
                    // negation of the correct condition is a misbehaviour case
                    if !(@next_consensus_state.timestamp > @target_consensus_state.timestamp) {
                        return true;
                    }
                }
            }

            false
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
            let consensus_state: CometConsensusState = self
                .consensus_states
                .read((client_sequence, height));

            assert(consensus_state.is_non_zero(), CometErrors::MISSING_CONSENSUS_STATE);

            consensus_state
        }

        fn consensus_state_exists(
            self: @ComponentState<TContractState>, client_sequence: u64, height: Height,
        ) -> bool {
            let consensus_state: CometConsensusState = self
                .consensus_states
                .read((client_sequence, height));

            consensus_state.is_non_zero()
        }

        fn read_client_processed_time(
            self: @ComponentState<TContractState>, client_sequence: u64, height: Height,
        ) -> u64 {
            let processed_time = self.client_processed_times.read((client_sequence, height));

            assert(processed_time.is_non_zero(), CometErrors::MISSING_CLIENT_PROCESSED_TIME);

            processed_time
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

            if span_contains(update_heights.span(), @update_height) {
                return;
            }

            let mut len = update_heights.len();

            if len == 100 {
                update_heights.pop_front().unwrap();
                len = update_heights.len();
            }

            // if the new height is bigger than the last one or the first, we can just append it
            if len == 0 || update_heights.at(len - 1) < @update_height {
                update_heights.append(update_height);
                self.update_heights.write(client_sequence, update_heights);
                return;
            }

            // update_heights is already sorted.
            // we only need to insert the new height in the right place

            let mut new_update_heights = array![];
            let mut inserted = false;

            while let Some(height) = update_heights.pop_front() {
                if height > update_height {
                    new_update_heights.append(update_height);
                    inserted = true;
                }
                new_update_heights.append(height);
                if inserted {
                    break;
                }
            }

            if inserted {
                new_update_heights.append_span(update_heights.span());
            } else {
                new_update_heights.append(update_height);
            }
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
            processed_height: u64,
            processed_time: u64,
        ) {
            self.consensus_states.write((client_sequence, height), consensus_state);
            self.client_processed_heights.write((client_sequence, height), processed_height);
            self.client_processed_times.write((client_sequence, height), processed_time);
        }

        fn remove_consensus_state(
            ref self: ComponentState<TContractState>, client_sequence: u64, height: Height,
        ) {
            let consensus_zero = CometConsensusStateZero::zero();
            self.consensus_states.write((client_sequence, height), consensus_zero);
            self.client_processed_times.write((client_sequence, height), 0);
            self.client_processed_heights.write((client_sequence, height), 0);
        }
    }
}
