use protobuf::types::wkt::Timestamp;
use protobuf::types::message::{ProtoMessage, ProtoCodecImpl};
use protobuf::primitives::array::{ByteArrayAsProtoMessage, ArrayAsProtoMessage};
use protobuf::primitives::numeric::{
    NumberAsProtoMessage, I32AsProtoMessage, I64AsProtoMessage, BoolAsProtoMessage
};
use protobuf::types::tag::WireType;

#[derive(Default, Debug, Copy, Drop, PartialEq, Serde)]
pub struct Consensus {
    pub block: u64,
    pub app: u64,
}

impl ConsensusAsProtoMessage of ProtoMessage<Consensus> {
    fn encode_raw(self: @Consensus, ref output: ByteArray) {
        ProtoCodecImpl::encode_length_delimited_raw(1, self.block, ref output);
        ProtoCodecImpl::encode_length_delimited_raw(2, self.app, ref output);
    }

    fn decode_raw(ref value: Consensus, serialized: @ByteArray, ref index: usize, length: usize) {
        let bound = index + length;

        ProtoCodecImpl::decode_length_delimited_raw(1, ref value.block, serialized, ref index);
        ProtoCodecImpl::decode_length_delimited_raw(2, ref value.app, serialized, ref index);

        assert(index == bound, 'invalid length for Consensus');
    }

    fn wire_type() -> WireType {
        WireType::LengthDelimited
    }
}

#[derive(Default, Debug, Drop, PartialEq, Serde)]
pub struct PartSetHeader {
    pub total: u32,
    pub hash: ByteArray,
}

impl PartSetHeaderAsProtoMessage of ProtoMessage<PartSetHeader> {
    fn encode_raw(self: @PartSetHeader, ref output: ByteArray) {
        ProtoCodecImpl::encode_length_delimited_raw(1, self.total, ref output);
        ProtoCodecImpl::encode_length_delimited_raw(2, self.hash, ref output);
    }

    fn decode_raw(
        ref value: PartSetHeader, serialized: @ByteArray, ref index: usize, length: usize
    ) {
        let bound = index + length;

        ProtoCodecImpl::decode_length_delimited_raw(1, ref value.total, serialized, ref index);
        ProtoCodecImpl::decode_length_delimited_raw(2, ref value.hash, serialized, ref index);

        assert(index == bound, 'invalid length for PSH');
    }

    fn wire_type() -> WireType {
        WireType::LengthDelimited
    }
}

#[derive(Default, Debug, Drop, PartialEq, Serde)]
pub struct BlockId {
    pub hash: ByteArray,
    pub part_set_header: PartSetHeader,
}

impl BlockIdAsProtoMessage of ProtoMessage<BlockId> {
    fn encode_raw(self: @BlockId, ref output: ByteArray) {
        ProtoCodecImpl::encode_length_delimited_raw(1, self.hash, ref output);
        ProtoCodecImpl::encode_length_delimited_raw(2, self.part_set_header, ref output);
    }

    fn decode_raw(ref value: BlockId, serialized: @ByteArray, ref index: usize, length: usize) {
        let bound = index + length;

        ProtoCodecImpl::decode_length_delimited_raw(1, ref value.hash, serialized, ref index);
        ProtoCodecImpl::decode_length_delimited_raw(
            2, ref value.part_set_header, serialized, ref index
        );

        assert(index == bound, 'invalid length for BlockId');
    }

    fn wire_type() -> WireType {
        WireType::LengthDelimited
    }
}

#[derive(Default, Debug, Drop, PartialEq, Serde)]
pub struct Header {
    pub version: Consensus,
    pub chain_id: ByteArray,
    pub height: i64,
    pub time: Timestamp,
    pub last_block_id: BlockId,
    pub last_commit_hash: ByteArray,
    pub data_hash: ByteArray,
    pub validators_hash: ByteArray,
    pub next_validators_hash: ByteArray,
    pub consensus_hash: ByteArray,
    pub app_hash: ByteArray,
    pub last_results_hash: ByteArray,
    pub evidence_hash: ByteArray,
    pub proposer_address: ByteArray,
}

impl HeaderAsProtoMessage of ProtoMessage<Header> {
    fn encode_raw(self: @Header, ref output: ByteArray) {
        ProtoCodecImpl::encode_length_delimited_raw(1, self.version, ref output);
        ProtoCodecImpl::encode_length_delimited_raw(2, self.chain_id, ref output);
        ProtoCodecImpl::encode_length_delimited_raw(3, self.height, ref output);
        ProtoCodecImpl::encode_length_delimited_raw(4, self.time, ref output);
        ProtoCodecImpl::encode_length_delimited_raw(5, self.last_block_id, ref output);
        ProtoCodecImpl::encode_length_delimited_raw(6, self.last_commit_hash, ref output);
        ProtoCodecImpl::encode_length_delimited_raw(7, self.data_hash, ref output);
        ProtoCodecImpl::encode_length_delimited_raw(8, self.validators_hash, ref output);
        ProtoCodecImpl::encode_length_delimited_raw(9, self.next_validators_hash, ref output);
        ProtoCodecImpl::encode_length_delimited_raw(10, self.consensus_hash, ref output);
        ProtoCodecImpl::encode_length_delimited_raw(11, self.app_hash, ref output);
        ProtoCodecImpl::encode_length_delimited_raw(12, self.last_results_hash, ref output);
        ProtoCodecImpl::encode_length_delimited_raw(13, self.evidence_hash, ref output);
        ProtoCodecImpl::encode_length_delimited_raw(14, self.proposer_address, ref output);
    }

    fn decode_raw(ref value: Header, serialized: @ByteArray, ref index: usize, length: usize) {
        let bound = index + length;

        ProtoCodecImpl::decode_length_delimited_raw(1, ref value.version, serialized, ref index);
        ProtoCodecImpl::decode_length_delimited_raw(2, ref value.chain_id, serialized, ref index);
        ProtoCodecImpl::decode_length_delimited_raw(3, ref value.height, serialized, ref index);
        ProtoCodecImpl::decode_length_delimited_raw(4, ref value.time, serialized, ref index);
        ProtoCodecImpl::decode_length_delimited_raw(
            5, ref value.last_block_id, serialized, ref index
        );
        ProtoCodecImpl::decode_length_delimited_raw(
            6, ref value.last_commit_hash, serialized, ref index
        );
        ProtoCodecImpl::decode_length_delimited_raw(7, ref value.data_hash, serialized, ref index);
        ProtoCodecImpl::decode_length_delimited_raw(
            8, ref value.validators_hash, serialized, ref index
        );
        ProtoCodecImpl::decode_length_delimited_raw(
            9, ref value.next_validators_hash, serialized, ref index
        );
        ProtoCodecImpl::decode_length_delimited_raw(
            10, ref value.consensus_hash, serialized, ref index
        );
        ProtoCodecImpl::decode_length_delimited_raw(11, ref value.app_hash, serialized, ref index);
        ProtoCodecImpl::decode_length_delimited_raw(
            12, ref value.last_results_hash, serialized, ref index
        );
        ProtoCodecImpl::decode_length_delimited_raw(
            13, ref value.evidence_hash, serialized, ref index
        );
        ProtoCodecImpl::decode_length_delimited_raw(
            14, ref value.proposer_address, serialized, ref index
        );

        assert(index == bound, 'invalid length for Header');
    }

    fn wire_type() -> WireType {
        WireType::LengthDelimited
    }
}

#[derive(Default, Debug, Copy, Drop, PartialEq, Serde)]
pub enum BlockIdFlag {
    #[default]
    Unknown,
    Absent,
    Commit,
    Nil,
}

impl BlockIdFlagIntoU64 of Into<BlockIdFlag, u64> {
    fn into(self: BlockIdFlag) -> u64 {
        match self {
            BlockIdFlag::Unknown => 0,
            BlockIdFlag::Absent => 1,
            BlockIdFlag::Commit => 2,
            BlockIdFlag::Nil => 3,
        }
    }
}

impl U64IntoBlockIdFlag of Into<u64, BlockIdFlag> {
    fn into(self: u64) -> BlockIdFlag {
        match self {
            0 => BlockIdFlag::Unknown,
            1 => BlockIdFlag::Absent,
            2 => BlockIdFlag::Commit,
            3 => BlockIdFlag::Nil,
            _ => panic!("invalid BlockIdFlag"),
        }
    }
}

#[derive(Default, Debug, Drop, PartialEq, Serde)]
pub struct CommitSig {
    pub block_id_flag: BlockIdFlag,
    pub validator_address: ByteArray,
    pub timestamp: Timestamp,
    pub signature: ByteArray,
}

impl CommitSigAsProtoMessage of ProtoMessage<CommitSig> {
    fn encode_raw(self: @CommitSig, ref output: ByteArray) {
        ProtoCodecImpl::encode_length_delimited_raw(1, self.block_id_flag, ref output);
        ProtoCodecImpl::encode_length_delimited_raw(2, self.validator_address, ref output);
        ProtoCodecImpl::encode_length_delimited_raw(3, self.timestamp, ref output);
        ProtoCodecImpl::encode_length_delimited_raw(4, self.signature, ref output);
    }

    fn decode_raw(ref value: CommitSig, serialized: @ByteArray, ref index: usize, length: usize) {
        let bound = index + length;

        ProtoCodecImpl::decode_length_delimited_raw(
            1, ref value.block_id_flag, serialized, ref index
        );
        ProtoCodecImpl::decode_length_delimited_raw(
            2, ref value.validator_address, serialized, ref index
        );
        ProtoCodecImpl::decode_length_delimited_raw(3, ref value.timestamp, serialized, ref index);
        ProtoCodecImpl::decode_length_delimited_raw(4, ref value.signature, serialized, ref index);

        assert(index == bound, 'invalid length for CommitSig');
    }

    fn wire_type() -> WireType {
        WireType::LengthDelimited
    }
}

#[derive(Default, Debug, Drop, PartialEq, Serde)]
pub struct Commit {
    pub height: i64,
    pub round: i32,
    pub block_id: BlockId,
    pub signatures: Array<CommitSig>,
}

impl CommitAsProtoMessage of ProtoMessage<Commit> {
    fn encode_raw(self: @Commit, ref output: ByteArray) {
        ProtoCodecImpl::encode_length_delimited_raw(1, self.height, ref output);
        ProtoCodecImpl::encode_length_delimited_raw(2, self.round, ref output);
        ProtoCodecImpl::encode_length_delimited_raw(3, self.block_id, ref output);
        ProtoCodecImpl::encode_length_delimited_raw(4, self.signatures, ref output);
    }

    fn decode_raw(ref value: Commit, serialized: @ByteArray, ref index: usize, length: usize) {
        let bound = index + length;

        ProtoCodecImpl::decode_length_delimited_raw(1, ref value.height, serialized, ref index);
        ProtoCodecImpl::decode_length_delimited_raw(2, ref value.round, serialized, ref index);
        ProtoCodecImpl::decode_length_delimited_raw(3, ref value.block_id, serialized, ref index);
        ProtoCodecImpl::decode_length_delimited_raw(4, ref value.signatures, serialized, ref index);

        assert(index == bound, 'invalid length for Commit');
    }

    fn wire_type() -> WireType {
        WireType::LengthDelimited
    }
}

#[derive(Default, Debug, Drop, PartialEq, Serde)]
pub struct SignedHeader {
    pub header: Header,
    pub commit: Commit,
}

impl SignedHeaderAsProtoMessage of ProtoMessage<SignedHeader> {
    fn encode_raw(self: @SignedHeader, ref output: ByteArray) {
        ProtoCodecImpl::encode_length_delimited_raw(1, self.header, ref output);
        ProtoCodecImpl::encode_length_delimited_raw(2, self.commit, ref output);
    }

    fn decode_raw(
        ref value: SignedHeader, serialized: @ByteArray, ref index: usize, length: usize
    ) {
        let bound = index + length;

        ProtoCodecImpl::decode_length_delimited_raw(1, ref value.header, serialized, ref index);
        ProtoCodecImpl::decode_length_delimited_raw(2, ref value.commit, serialized, ref index);

        assert(index == bound, 'invalid length for SignedHeader');
    }

    fn wire_type() -> WireType {
        WireType::LengthDelimited
    }
}

#[derive(Default, Debug, Drop, PartialEq, Serde)]
pub struct PublicKey {
    // this is oneof
    pub ed25519: ByteArray,
    pub secp256k1: ByteArray,
}

impl PublicKeyAsProtoMessage of ProtoMessage<PublicKey> {
    fn encode_raw(self: @PublicKey, ref output: ByteArray) {
        ProtoCodecImpl::encode_length_delimited_raw(1, self.ed25519, ref output);
        ProtoCodecImpl::encode_length_delimited_raw(2, self.secp256k1, ref output);
    }

    fn decode_raw(ref value: PublicKey, serialized: @ByteArray, ref index: usize, length: usize) {
        let bound = index + length;

        ProtoCodecImpl::decode_length_delimited_raw(1, ref value.ed25519, serialized, ref index);
        ProtoCodecImpl::decode_length_delimited_raw(2, ref value.secp256k1, serialized, ref index);

        assert(index == bound, 'invalid length for PublicKey');
    }

    fn wire_type() -> WireType {
        WireType::LengthDelimited
    }
}

#[derive(Default, Debug, Drop, PartialEq, Serde)]
pub struct Validator {
    pub address: ByteArray,
    pub pub_key: PublicKey,
    pub voting_power: i64,
    pub proposer_priority: i64,
}

impl ValidatorAsProtoMessage of ProtoMessage<Validator> {
    fn encode_raw(self: @Validator, ref output: ByteArray) {
        ProtoCodecImpl::encode_length_delimited_raw(1, self.address, ref output);
        ProtoCodecImpl::encode_length_delimited_raw(2, self.pub_key, ref output);
        ProtoCodecImpl::encode_length_delimited_raw(3, self.voting_power, ref output);
        ProtoCodecImpl::encode_length_delimited_raw(4, self.proposer_priority, ref output);
    }

    fn decode_raw(ref value: Validator, serialized: @ByteArray, ref index: usize, length: usize) {
        let bound = index + length;

        ProtoCodecImpl::decode_length_delimited_raw(1, ref value.address, serialized, ref index);
        ProtoCodecImpl::decode_length_delimited_raw(2, ref value.pub_key, serialized, ref index);
        ProtoCodecImpl::decode_length_delimited_raw(
            3, ref value.voting_power, serialized, ref index
        );
        ProtoCodecImpl::decode_length_delimited_raw(
            4, ref value.proposer_priority, serialized, ref index
        );

        assert(index == bound, 'invalid length for Validator');
    }

    fn wire_type() -> WireType {
        WireType::LengthDelimited
    }
}

#[derive(Default, Debug, Drop, PartialEq, Serde)]
pub struct ValidatorSet {
    pub validators: Array<Validator>,
    pub proposer: Validator,
    pub total_voting_power: i64,
}

impl ValidatorSetAsProtoMessage of ProtoMessage<ValidatorSet> {
    fn encode_raw(self: @ValidatorSet, ref output: ByteArray) {
        ProtoCodecImpl::encode_length_delimited_raw(1, self.validators, ref output);
        ProtoCodecImpl::encode_length_delimited_raw(2, self.proposer, ref output);
        ProtoCodecImpl::encode_length_delimited_raw(3, self.total_voting_power, ref output);
    }

    fn decode_raw(
        ref value: ValidatorSet, serialized: @ByteArray, ref index: usize, length: usize
    ) {
        let bound = index + length;

        ProtoCodecImpl::decode_length_delimited_raw(1, ref value.validators, serialized, ref index);
        ProtoCodecImpl::decode_length_delimited_raw(2, ref value.proposer, serialized, ref index);
        ProtoCodecImpl::decode_length_delimited_raw(
            3, ref value.total_voting_power, serialized, ref index
        );

        assert(index == bound, 'invalid length for ValidatorSet');
    }

    fn wire_type() -> WireType {
        WireType::LengthDelimited
    }
}
