#[starknet::component]
pub mod CometClientComponent {
    use alexandria_data_structures::array_ext::ArrayTraitExt;
    use alexandria_data_structures::byte_array_ext::SpanU8IntoBytearray;
    use alexandria_sorting::MergeSort;
    use cometbft::types::{Options, TrustedBlockState, UntrustedBlockState};
    use cometbft::verifier::verify_update_header;
    use core::num::traits::Zero;
    use ics23::{
        MerkleProof, Proof, array_u8_to_byte_array, byte_array_to_array_u8,
        verify_membership as ics23_verify_membership,
        verify_non_membership as ics23_verify_non_membership,
    };
    use openzeppelin_access::ownable::OwnableComponent;
    use openzeppelin_access::ownable::interface::IOwnable;
    use protobuf::types::message::ProtoCodecImpl;
    use starknet::storage::{
        Map, StorageMapReadAccess, StorageMapWriteAccess, StoragePointerReadAccess,
        StoragePointerWriteAccess,
    };
    use starknet::{ContractAddress, get_block_number, get_block_timestamp, get_caller_address};
    use starknet_ibc_clients::cometbft::{
        CometClientState, CometClientStateImpl, CometConsensusState, CometConsensusStateImpl,
        CometConsensusStateStore, CometConsensusStateToStore, CometConsensusStateZero, CometErrors,
        CometHeader, CometHeaderImpl, CometHeaderIntoConsensusState, StoreToCometConsensusState,
    };
    use starknet_ibc_core::client::{
        CreateResponse, CreateResponseImpl, Height, HeightImpl, HeightPartialOrd, HeightZero,
        IClientHandler, IClientQuery, IClientStateExecution, IClientStateValidation,
        MsgCreateClient, MsgRecoverClient, MsgUpdateClient, MsgUpgradeClient, Status, StatusTrait,
        StoreHeightArray, Timestamp, TimestampImpl, TimestampToProto, UpdateResponse,
    };
    use starknet_ibc_core::commitment::{StateProof, StateRoot, StateValue};
    use starknet_ibc_core::host::ClientIdImpl;
    use starknet_ibc_utils::ValidateBasic;

    #[storage]
    pub struct Storage {
        next_client_sequence: u64,
        update_heights: Map<u64, Array<Height>>,
        client_states: Map<u64, CometClientState>,
        consensus_states: Map<(u64, Height), CometConsensusStateStore>,
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
            self.assert_owner();
            self.recover_validate(msg.clone());
            self.recover_execute(msg)
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

            let mut height = target_height;

            let mut update_heights_span = update_heights.span();

            while let Option::Some(update_height) = update_heights_span.pop_back() {
                if @target_height >= update_height {
                    height = *update_height;
                    break;
                }
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
            let mut serialized: Array<felt252> = ArrayTrait::new();

            let consensus_state: CometConsensusState = self
                .read_consensus_state(client_sequence, height)
                .into();

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
                .read_consensus_state(client_sequence, comet_client_state.latest_height.clone());

            let status = self._status(comet_client_state, latest_consensus_state, client_sequence);

            assert(status.is_active(), CometErrors::INACTIVE_CLIENT);

            self.verify_client_message(client_sequence, msg.client_message.clone());
        }

        fn update_execute(
            ref self: ComponentState<TContractState>, msg: MsgUpdateClient,
        ) -> UpdateResponse {
            let client_sequence = msg.client_id.sequence;

            if self.verify_misbehaviour(client_sequence, msg.client_message.clone()) {
                return self.update_on_misbehaviour(client_sequence, msg.client_message.clone());
            }

            self.update_state(client_sequence, msg.client_message)
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
                .read_consensus_state(
                    subject_client_sequence, subject_client_state.latest_height.clone(),
                );
            let substitute_consensus_state = self
                .read_consensus_state(
                    substitute_client_sequence, substitute_client_state.latest_height.clone(),
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
                    substitute_client_sequence, substitute_client_state.latest_height.clone(),
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
            paths: Array<ByteArray>,
            value: StateValue,
            proof: StateProof,
            root: StateRoot,
        ) {
            let byte_array_proof = array_u8_to_byte_array(@proof.proof);
            let decoded_proof = ProtoCodecImpl::decode::<MerkleProof>(@byte_array_proof).unwrap();
            let specs = self.read_client_state(client_sequence).proof_spec;
            let mut proofs: Array<Proof> = ArrayTrait::new();
            for proof in decoded_proof.proofs {
                proofs.append(proof.proof);
            }
            let root = root.root;
            let mut keys = array![];
            for path in paths {
                let path_bytes = byte_array_to_array_u8(@path);
                keys.append(path_bytes);
            }
            let value = value.value;
            ics23_verify_membership(specs, @proofs, root, keys, value, 0);
        }

        fn verify_non_membership(
            self: @ComponentState<TContractState>,
            client_sequence: u64,
            paths: Array<ByteArray>,
            proof: StateProof,
            root: StateRoot,
        ) {
            let byte_array_proof = array_u8_to_byte_array(@proof.proof);
            let decoded_proof = ProtoCodecImpl::decode::<MerkleProof>(@byte_array_proof).unwrap();
            let specs = self.read_client_state(client_sequence).proof_spec;
            let mut proofs: Array<Proof> = ArrayTrait::new();
            for proof in decoded_proof.proofs {
                proofs.append(proof.proof);
            }
            let root = root.root;
            let mut keys = array![];
            for path in paths {
                let path_bytes = byte_array_to_array_u8(@path);
                keys.append(path_bytes);
            }
            ics23_verify_non_membership(specs, @proofs, root, keys);
        }

        fn verify_client_message(
            self: @ComponentState<TContractState>,
            client_sequence: u64,
            client_message: Array<felt252>,
        ) {
            let header: CometHeader = CometHeaderImpl::deserialize(client_message);
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
                next_validators: header.trusted_validator_set,
                next_validators_hash: trusted_consensus_state.next_validators_hash,
            };

            let untrusted_block_state = UntrustedBlockState {
                signed_header: header.signed_header, validators: header.validator_set,
            };

            let trust_threshold = client_state.trust_level;
            let trusting_period = client_state.trusting_period.try_into().unwrap();
            let clock_drift = client_state.max_clock_drift.try_into().unwrap();
            let now = TimestampImpl::host().try_into().unwrap();

            let options = Options { trust_threshold, trusting_period, clock_drift };
            verify_update_header(untrusted_block_state, trusted_block_state, options, now)
        }

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

            let comet_client_state_latest_height = comet_client_state.latest_height.clone();

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

            let header: CometHeader = CometHeaderImpl::deserialize(client_message);

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
                    let consensus_state = self
                        .read_consensus_state(client_sequence, height.clone());

                    if consensus_state
                        .status(client_state.trusting_period, client_state.max_clock_drift)
                        .is_expired() {
                        self.remove_consensus_state(client_sequence, height.clone());
                    } else {
                        check_in_progress = false;
                        heights_kept.append(height.clone());
                    }
                } else {
                    heights_kept.append(height.clone());
                }
            }
            // Write directly since heights_kept is already sorted and is equal or
            // smaller to the previous one
            self.update_heights.write(client_sequence, heights_kept);

            if !self.consensus_state_exists(client_sequence, header_height.clone()) {
                client_state.update(header_height.clone());

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
            let header = CometHeaderImpl::deserialize(client_message);

            let mut client_state = self.read_client_state(client_sequence);

            let frozen_height = Height {
                revision_number: header.trusted_height.revision_number,
                revision_height: header.trusted_height.revision_height,
            };

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
            assert(update_heights.len() > 0, CometErrors::ZERO_UPDATE_HEIGHTS);

            let mut update_heights_span = update_heights.span();

            while let Option::Some(height) = update_heights_span.pop_front() {
                self.remove_consensus_state(subject_client_sequence, height.clone());
            }

            self.update_heights.write(subject_client_sequence, array![]);

            let substitute_client_state = CometClientStateImpl::deserialize(
                substitute_client_state,
            );
            let substitute_consensus_state = CometConsensusStateImpl::deserialize(
                substitute_consensus_state,
            );

            let latest_height = substitute_client_state.latest_height.clone();

            let processed_height = self
                .read_client_processed_height(substitute_client_sequence, latest_height.clone());
            let processed_time = self
                .read_client_processed_time(substitute_client_sequence, latest_height.clone());

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
            processed_height: u64,
            processed_time: u64,
        ) {
            self.write_client_state(client_sequence, client_state);

            self.write_update_height(client_sequence, update_height.clone());

            self
                .write_consensus_state(
                    client_sequence,
                    update_height.clone(),
                    consensus_state,
                    processed_height,
                    processed_time,
                );
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
                .read((client_sequence, height))
                .into();

            assert(consensus_state.is_non_zero(), CometErrors::MISSING_CONSENSUS_STATE);

            consensus_state
        }

        fn consensus_state_exists(
            self: @ComponentState<TContractState>, client_sequence: u64, height: Height,
        ) -> bool {
            let consensus_state: CometConsensusState = self
                .consensus_states
                .read((client_sequence, height))
                .into();

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
            processed_height: u64,
            processed_time: u64,
        ) {
            self.consensus_states.write((client_sequence, height.clone()), consensus_state.into());
            self
                .client_processed_heights
                .write((client_sequence, height.clone()), processed_height);
            self.client_processed_times.write((client_sequence, height), processed_time);
        }

        fn remove_consensus_state(
            ref self: ComponentState<TContractState>, client_sequence: u64, height: Height,
        ) {
            let consensus_zero = CometConsensusStateZero::zero();
            self.consensus_states.write((client_sequence, height), consensus_zero.into());
            self.client_processed_times.write((client_sequence, height), 0);
            self.client_processed_heights.write((client_sequence, height), 0);
        }
    }
}
