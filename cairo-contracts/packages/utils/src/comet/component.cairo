use cometbft::types::{CanonicalVote, Header, ValidatorSet};

#[starknet::interface]
pub trait ICometBftFactCheckerQueryTrait<TContractState> {
    fn get_header_hash(self: @TContractState, header: Header) -> u256;
    fn get_validator_set_hash(self: @TContractState, validator_set: ValidatorSet) -> u256;
    fn get_vote_signature(
        self: @TContractState, canonical_vote: CanonicalVote, public_key: u256,
    ) -> [u256; 2];
}

#[starknet::interface]
pub trait ICometBftFactCheckerStoreTrait<TContractState> {
    fn store_header_hash(ref self: TContractState, header: Header);
    fn store_validator_set_hash(ref self: TContractState, validator_set: ValidatorSet);
    fn store_vote_signature(
        ref self: TContractState,
        canonical_vote: CanonicalVote,
        public_key: u256,
        signature: [u256; 2],
    );
}

#[starknet::component]
pub mod CometBftFactCheckerComponent {
    use alexandria_math::ed25519::verify_signature;
    use cometbft::types::{CanonicalVote, Header, SimpleValidator, ValidatorSet};
    use cometbft::utils::MerkleHashImpl;
    use core::poseidon::poseidon_hash_span;
    use ics23::byte_array_to_array_u8;
    use protobuf::primitives::array::{ByteArrayAsProtoMessage, BytesAsProtoMessage};
    use protobuf::primitives::numeric::U64AsProtoMessage;
    use protobuf::types::message::ProtoCodecImpl;
    use starknet::storage::{Map, StorageMapReadAccess, StorageMapWriteAccess, StoragePathEntry};

    #[storage]
    pub struct Storage {
        // commit hash
        header_hash: Map<felt252, u256>,
        // validator set hash
        validator_set_hash: Map<felt252, u256>,
        // valid votes
        vote: Map<felt252, Map<felt252, [u256; 2]>>,
    }

    #[event]
    #[derive(Drop, Debug, starknet::Event)]
    pub enum Event {}

    fn serde_key<T, +Serde<T>>(value: @T) -> felt252 {
        let mut serialized = array![];
        value.serialize(ref serialized);
        let span = serialized.span();
        poseidon_hash_span(span)
    }

    fn u32_8_into_u256(value: [u32; 8]) -> u256 {
        let [n0, n1, n2, n3, n4, n5, n6, n7] = value;

        let mut result: u256 = n0.into();
        result = result * 0x1_0000_0000 + n1.into();
        result = result * 0x1_0000_0000 + n2.into();
        result = result * 0x1_0000_0000 + n3.into();
        result = result * 0x1_0000_0000 + n4.into();
        result = result * 0x1_0000_0000 + n5.into();
        result = result * 0x1_0000_0000 + n6.into();
        result = result * 0x1_0000_0000 + n7.into();

        result
    }

    #[embeddable_as(CometBftFactCheckerQuery)]
    pub impl CometBftFactCheckerQueryImpl<
        TContractState, +HasComponent<TContractState>,
    > of super::ICometBftFactCheckerQueryTrait<ComponentState<TContractState>> {
        fn get_header_hash(self: @ComponentState<TContractState>, header: Header) -> u256 {
            let value = self.validator_set_hash.read(serde_key(@header));
            assert(value != 0, 'Header not stored');
            value
        }

        fn get_validator_set_hash(
            self: @ComponentState<TContractState>, validator_set: ValidatorSet,
        ) -> u256 {
            let value = self.validator_set_hash.read(serde_key(@validator_set));
            assert(value != 0, 'ValSet not stored');
            value
        }

        fn get_vote_signature(
            self: @ComponentState<TContractState>, canonical_vote: CanonicalVote, public_key: u256,
        ) -> [u256; 2] {
            let value = self.vote.entry(serde_key(@canonical_vote)).read(serde_key(@public_key));
            assert(value != [0; 2], 'Vote no stored');
            value
        }
    }

    #[embeddable_as(CometBftFactCheckerStore)]
    pub impl CometBftFactCheckerStoreImpl<
        TContractState, +HasComponent<TContractState>,
    > of super::ICometBftFactCheckerStoreTrait<ComponentState<TContractState>> {
        fn store_header_hash(ref self: ComponentState<TContractState>, header: Header) {
            let header_bytes = array![
                ProtoCodecImpl::encode(@header.version),
                ProtoCodecImpl::encode_as_msg(@header.chain_id),
                ProtoCodecImpl::encode_as_msg(@header.height),
                ProtoCodecImpl::encode(@header.time),
                ProtoCodecImpl::encode(@header.last_block_id),
                ProtoCodecImpl::encode_as_msg(@header.last_commit_hash),
                ProtoCodecImpl::encode_as_msg(@header.data_hash),
                ProtoCodecImpl::encode_as_msg(@header.validators_hash),
                ProtoCodecImpl::encode_as_msg(@header.next_validators_hash),
                ProtoCodecImpl::encode_as_msg(@header.consensus_hash),
                ProtoCodecImpl::encode_as_msg(@header.app_hash),
                ProtoCodecImpl::encode_as_msg(@header.last_results_hash),
                ProtoCodecImpl::encode_as_msg(@header.evidence_hash),
                ProtoCodecImpl::encode_as_msg(@header.proposer_address),
            ];

            let merkle_hash = MerkleHashImpl::hash_byte_vectors(header_bytes.span());

            self.header_hash.write(serde_key(@header), u32_8_into_u256(merkle_hash));
        }

        fn store_validator_set_hash(
            ref self: ComponentState<TContractState>, validator_set: ValidatorSet,
        ) {
            let mut validator_bytes = array![];

            for validator in validator_set.validators.span() {
                let simple_validator: SimpleValidator = validator.clone().into();
                let bytes = ProtoCodecImpl::encode(@simple_validator);
                validator_bytes.append(bytes);
            }

            let merkle_hash = MerkleHashImpl::hash_byte_vectors(validator_bytes.span());

            self.header_hash.write(serde_key(@validator_set), u32_8_into_u256(merkle_hash));
        }

        fn store_vote_signature(
            ref self: ComponentState<TContractState>,
            canonical_vote: CanonicalVote,
            public_key: u256,
            signature: [u256; 2],
        ) {
            let signed_bytes = ProtoCodecImpl::encode_with_length(@canonical_vote);
            let signed_array_u8 = byte_array_to_array_u8(@signed_bytes);

            assert(
                verify_signature(signed_array_u8.span(), signature.span(), public_key),
                'Invalid Vote',
            );

            self.vote.entry(serde_key(@canonical_vote)).write(serde_key(@public_key), signature);
        }
    }
}
