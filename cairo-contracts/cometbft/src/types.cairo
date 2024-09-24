use protobuf::types::wkt::Timestamp;
use protobuf::types::message::{
    ProtoMessage, ProtoCodecImpl, EncodeContext, DecodeContext, EncodeContextImpl,
    DecodeContextImpl, Name
};
use protobuf::primitives::array::{ByteArrayAsProtoMessage};
use protobuf::primitives::numeric::{
    UnsignedAsProtoMessage, I32AsProtoMessage, I64AsProtoMessage, BoolAsProtoMessage
};
use protobuf::types::tag::WireType;

#[derive(Default, Debug, Copy, Drop, PartialEq, Serde)]
pub struct Consensus {
    pub block: u64,
    pub app: u64,
}

impl ConsensusAsProtoMessage of ProtoMessage<Consensus> {
    fn encode_raw(self: @Consensus, ref context: EncodeContext) {
        context.encode_field(1, self.block);
        context.encode_field(2, self.app);
    }

    fn decode_raw(ref self: Consensus, ref context: DecodeContext, length: usize) {
        context.init_branch(length);
        context.decode_field(1, ref self.block);
        context.decode_field(2, ref self.app);
        context.end_branch();
    }

    fn wire_type() -> WireType {
        WireType::LengthDelimited
    }
}

impl ConsensusAsName of Name<Consensus> {
    fn type_url() -> ByteArray {
        "Consensus"
    }
}

#[derive(Default, Debug, Clone, Drop, PartialEq, Serde)]
pub struct PartSetHeader {
    pub total: u32,
    pub hash: ByteArray,
}

impl PartSetHeaderAsProtoMessage of ProtoMessage<PartSetHeader> {
    fn encode_raw(self: @PartSetHeader, ref context: EncodeContext) {
        context.encode_field(1, self.total);
        context.encode_field(2, self.hash);
    }

    fn decode_raw(ref self: PartSetHeader, ref context: DecodeContext, length: usize) {
        context.init_branch(length);
        context.decode_field(1, ref self.total);
        context.decode_field(2, ref self.hash);
        context.end_branch();
    }

    fn wire_type() -> WireType {
        WireType::LengthDelimited
    }
}

impl PartSetHeaderAsName of Name<PartSetHeader> {
    fn type_url() -> ByteArray {
        "PartSetHeader"
    }
}

#[derive(Default, Debug, Clone, Drop, PartialEq, Serde)]
pub struct BlockId {
    pub hash: ByteArray,
    pub part_set_header: PartSetHeader,
}

impl BlockIdAsProtoMessage of ProtoMessage<BlockId> {
    fn encode_raw(self: @BlockId, ref context: EncodeContext) {
        context.encode_field(1, self.hash);
        context.encode_field(2, self.part_set_header);
    }

    fn decode_raw(ref self: BlockId, ref context: DecodeContext, length: usize) {
        context.init_branch(length);
        context.decode_field(1, ref self.hash);
        context.decode_field(2, ref self.part_set_header);
        context.end_branch();
    }

    fn wire_type() -> WireType {
        WireType::LengthDelimited
    }
}

impl BlockIdAsName of Name<BlockId> {
    fn type_url() -> ByteArray {
        "BlockId"
    }
}

#[derive(Default, Debug, Clone, Drop, PartialEq, Serde)]
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
    fn encode_raw(self: @Header, ref context: EncodeContext) {
        context.encode_field(1, self.version);
        context.encode_field(2, self.chain_id);
        context.encode_field(3, self.height);
        context.encode_field(4, self.time);
        context.encode_field(5, self.last_block_id);
        context.encode_field(6, self.last_commit_hash);
        context.encode_field(7, self.data_hash);
        context.encode_field(8, self.validators_hash);
        context.encode_field(9, self.next_validators_hash);
        context.encode_field(10, self.consensus_hash);
        context.encode_field(11, self.app_hash);
        context.encode_field(12, self.last_results_hash);
        context.encode_field(13, self.evidence_hash);
        context.encode_field(14, self.proposer_address);
    }

    fn decode_raw(ref self: Header, ref context: DecodeContext, length: usize) {
        context.init_branch(length);
        context.decode_field(1, ref self.version);
        context.decode_field(2, ref self.chain_id);
        context.decode_field(3, ref self.height);
        context.decode_field(4, ref self.time);
        context.decode_field(5, ref self.last_block_id);
        context.decode_field(6, ref self.last_commit_hash);
        context.decode_field(7, ref self.data_hash);
        context.decode_field(8, ref self.validators_hash);
        context.decode_field(9, ref self.next_validators_hash);
        context.decode_field(10, ref self.consensus_hash);
        context.decode_field(11, ref self.app_hash);
        context.decode_field(12, ref self.last_results_hash);
        context.decode_field(13, ref self.evidence_hash);
        context.decode_field(14, ref self.proposer_address);
        context.end_branch();
    }

    fn wire_type() -> WireType {
        WireType::LengthDelimited
    }
}

impl HeaderAsName of Name<Header> {
    fn type_url() -> ByteArray {
        "Header"
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

#[derive(Default, Debug, Clone, Drop, PartialEq, Serde)]
pub struct CommitSig {
    pub block_id_flag: BlockIdFlag,
    pub validator_address: ByteArray,
    pub timestamp: Timestamp,
    pub signature: ByteArray,
}

impl CommitSigAsProtoMessage of ProtoMessage<CommitSig> {
    fn encode_raw(self: @CommitSig, ref context: EncodeContext) {
        context.encode_field(1, self.block_id_flag);
        context.encode_field(2, self.validator_address);
        context.encode_field(3, self.timestamp);
        context.encode_field(4, self.signature);
    }

    fn decode_raw(ref self: CommitSig, ref context: DecodeContext, length: usize) {
        context.init_branch(length);
        context.decode_field(1, ref self.block_id_flag);
        context.decode_field(2, ref self.validator_address);
        context.decode_field(3, ref self.timestamp);
        context.decode_field(4, ref self.signature);
        context.end_branch();
    }

    fn wire_type() -> WireType {
        WireType::LengthDelimited
    }
}

impl CommitSigAsName of Name<CommitSig> {
    fn type_url() -> ByteArray {
        "CommitSig"
    }
}

#[derive(Default, Debug, Clone, Drop, PartialEq, Serde)]
pub struct Commit {
    pub height: i64,
    pub round: i32,
    pub block_id: BlockId,
    pub signatures: Array<CommitSig>,
}

impl CommitAsProtoMessage of ProtoMessage<Commit> {
    fn encode_raw(self: @Commit, ref context: EncodeContext) {
        context.encode_field(1, self.height);
        context.encode_field(2, self.round);
        context.encode_field(3, self.block_id);
        context.encode_repeated_field(4, self.signatures);
    }

    fn decode_raw(ref self: Commit, ref context: DecodeContext, length: usize) {
        context.init_branch(length);
        context.decode_field(1, ref self.height);
        context.decode_field(2, ref self.round);
        context.decode_field(3, ref self.block_id);
        context.decode_repeated_field(4, ref self.signatures);
        context.end_branch();
    }

    fn wire_type() -> WireType {
        WireType::LengthDelimited
    }
}

impl CommitAsName of Name<Commit> {
    fn type_url() -> ByteArray {
        "Commit"
    }
}

#[derive(Default, Debug, Clone, Drop, PartialEq, Serde)]
pub struct SignedHeader {
    pub header: Header,
    pub commit: Commit,
}

impl SignedHeaderAsProtoMessage of ProtoMessage<SignedHeader> {
    fn encode_raw(self: @SignedHeader, ref context: EncodeContext) {
        context.encode_field(1, self.header);
        context.encode_field(2, self.commit);
    }

    fn decode_raw(ref self: SignedHeader, ref context: DecodeContext, length: usize) {
        context.init_branch(length);
        context.decode_field(1, ref self.header);
        context.decode_field(2, ref self.commit);
        context.end_branch();
    }

    fn wire_type() -> WireType {
        WireType::LengthDelimited
    }
}

impl SignedHeaderAsName of Name<SignedHeader> {
    fn type_url() -> ByteArray {
        "SignedHeader"
    }
}

#[derive(Default, Debug, Clone, Drop, PartialEq, Serde)]
pub struct PublicKey {
    // TODO(rano): this is oneof
    pub ed25519: ByteArray,
    pub secp256k1: ByteArray,
}

impl PublicKeyAsProtoMessage of ProtoMessage<PublicKey> {
    fn encode_raw(self: @PublicKey, ref context: EncodeContext) {
        context.encode_field(1, self.ed25519);
        context.encode_field(2, self.secp256k1);
    }

    fn decode_raw(ref self: PublicKey, ref context: DecodeContext, length: usize) {
        context.init_branch(length);
        context.decode_field(1, ref self.ed25519);
        context.decode_field(2, ref self.secp256k1);
        context.end_branch();
    }

    fn wire_type() -> WireType {
        WireType::LengthDelimited
    }
}

impl PublicKeyAsName of Name<PublicKey> {
    fn type_url() -> ByteArray {
        "PublicKey"
    }
}

#[derive(Default, Debug, Clone, Drop, PartialEq, Serde)]
pub struct Validator {
    pub address: ByteArray,
    pub pub_key: PublicKey,
    pub voting_power: i64,
    pub proposer_priority: i64,
}

impl ValidatorAsProtoMessage of ProtoMessage<Validator> {
    fn encode_raw(self: @Validator, ref context: EncodeContext) {
        context.encode_field(1, self.address);
        context.encode_field(2, self.pub_key);
        context.encode_field(3, self.voting_power);
        context.encode_field(4, self.proposer_priority);
    }

    fn decode_raw(ref self: Validator, ref context: DecodeContext, length: usize) {
        context.init_branch(length);
        context.decode_field(1, ref self.address);
        context.decode_field(2, ref self.pub_key);
        context.decode_field(3, ref self.voting_power);
        context.decode_field(4, ref self.proposer_priority);
        context.end_branch();
    }

    fn wire_type() -> WireType {
        WireType::LengthDelimited
    }
}

impl ValidatorAsName of Name<Validator> {
    fn type_url() -> ByteArray {
        "Validator"
    }
}

#[derive(Default, Debug, Clone, Drop, PartialEq, Serde)]
pub struct ValidatorSet {
    pub validators: Array<Validator>,
    pub proposer: Validator,
    pub total_voting_power: i64,
}

impl ValidatorSetAsProtoMessage of ProtoMessage<ValidatorSet> {
    fn encode_raw(self: @ValidatorSet, ref context: EncodeContext) {
        context.encode_repeated_field(1, self.validators);
        context.encode_field(2, self.proposer);
        context.encode_field(3, self.total_voting_power);
    }

    fn decode_raw(ref self: ValidatorSet, ref context: DecodeContext, length: usize) {
        context.init_branch(length);
        context.decode_repeated_field(1, ref self.validators);
        context.decode_field(2, ref self.proposer);
        context.decode_field(3, ref self.total_voting_power);
        context.end_branch();
    }

    fn wire_type() -> WireType {
        WireType::LengthDelimited
    }
}

impl ValidatorSetAsName of Name<ValidatorSet> {
    fn type_url() -> ByteArray {
        "ValidatorSet"
    }
}
