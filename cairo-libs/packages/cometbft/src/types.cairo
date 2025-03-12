use protobuf::types::wkt::Timestamp;
use protobuf::types::message::{
    ProtoMessage, ProtoOneof, ProtoCodecImpl, EncodeContext, DecodeContext, EncodeContextImpl,
    DecodeContextImpl, ProtoName,
};
use protobuf::primitives::array::{BytesAsProtoMessage, ByteArrayAsProtoMessage};
use protobuf::primitives::numeric::{
    I32AsProtoMessage, I64AsProtoMessage, BoolAsProtoMessage, U64AsProtoMessage,
};
use protobuf::types::tag::{WireType, ProtobufTag};

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

    fn decode_raw(ref context: DecodeContext) -> Option<Consensus> {
        let block = context.decode_field(1)?;
        let app = context.decode_field(2)?;
        Option::Some(Consensus { block, app })
    }

    fn wire_type() -> WireType {
        WireType::LengthDelimited
    }
}

impl ConsensusAsProtoName of ProtoName<Consensus> {
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

    fn decode_raw(ref context: DecodeContext) -> Option<PartSetHeader> {
        let total = context.decode_field(1)?;
        let hash = context.decode_field(2)?;
        Option::Some(PartSetHeader { total, hash })
    }

    fn wire_type() -> WireType {
        WireType::LengthDelimited
    }
}

impl PartSetHeaderAsProtoName of ProtoName<PartSetHeader> {
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

    fn decode_raw(ref context: DecodeContext) -> Option<BlockId> {
        let hash = context.decode_field(1)?;
        let part_set_header = context.decode_field(2)?;
        Option::Some(BlockId { hash, part_set_header })
    }

    fn wire_type() -> WireType {
        WireType::LengthDelimited
    }
}

impl BlockIdAsProtoName of ProtoName<BlockId> {
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

    fn decode_raw(ref context: DecodeContext) -> Option<Header> {
        let version = context.decode_field(1)?;
        let chain_id = context.decode_field(2)?;
        let height = context.decode_field(3)?;
        let time = context.decode_field(4)?;
        let last_block_id = context.decode_field(5)?;
        let last_commit_hash = context.decode_field(6)?;
        let data_hash = context.decode_field(7)?;
        let validators_hash = context.decode_field(8)?;
        let next_validators_hash = context.decode_field(9)?;
        let consensus_hash = context.decode_field(10)?;
        let app_hash = context.decode_field(11)?;
        let last_results_hash = context.decode_field(12)?;
        let evidence_hash = context.decode_field(13)?;
        let proposer_address = context.decode_field(14)?;
        Option::Some(
            Header {
                version,
                chain_id,
                height,
                time,
                last_block_id,
                last_commit_hash,
                data_hash,
                validators_hash,
                next_validators_hash,
                consensus_hash,
                app_hash,
                last_results_hash,
                evidence_hash,
                proposer_address,
            },
        )
    }

    fn wire_type() -> WireType {
        WireType::LengthDelimited
    }
}

impl HeaderAsProtoName of ProtoName<Header> {
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

pub impl BlockIdFlagIntoU32 of Into<@BlockIdFlag, u32> {
    fn into(self: @BlockIdFlag) -> u32 {
        match self {
            BlockIdFlag::Unknown => 0,
            BlockIdFlag::Absent => 1,
            BlockIdFlag::Commit => 2,
            BlockIdFlag::Nil => 3,
        }
    }
}

pub impl U32TryIntoBlockIdFlag of TryInto<u32, BlockIdFlag> {
    fn try_into(self: u32) -> Option<BlockIdFlag> {
        match self {
            0 => Option::Some(BlockIdFlag::Unknown),
            1 => Option::Some(BlockIdFlag::Absent),
            2 => Option::Some(BlockIdFlag::Commit),
            3 => Option::Some(BlockIdFlag::Nil),
            _ => Option::None,
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
        context.encode_enum(1, self.block_id_flag);
        context.encode_field(2, self.validator_address);
        context.encode_field(3, self.timestamp);
        context.encode_field(4, self.signature);
    }

    fn decode_raw(ref context: DecodeContext) -> Option<CommitSig> {
        let block_id_flag = context.decode_enum(1)?;
        let validator_address = context.decode_field(2)?;
        let timestamp = context.decode_field(3)?;
        let signature = context.decode_field(4)?;
        Option::Some(CommitSig { block_id_flag, validator_address, timestamp, signature })
    }

    fn wire_type() -> WireType {
        WireType::LengthDelimited
    }
}

impl CommitSigAsProtoName of ProtoName<CommitSig> {
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

    fn decode_raw(ref context: DecodeContext) -> Option<Commit> {
        let height = context.decode_field(1)?;
        let round = context.decode_field(2)?;
        let block_id = context.decode_field(3)?;
        let signatures = context.decode_repeated_field(4)?;
        Option::Some(Commit { height, round, block_id, signatures })
    }

    fn wire_type() -> WireType {
        WireType::LengthDelimited
    }
}

impl CommitAsProtoName of ProtoName<Commit> {
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

    fn decode_raw(ref context: DecodeContext) -> Option<SignedHeader> {
        let header = context.decode_field(1)?;
        let commit = context.decode_field(2)?;
        Option::Some(SignedHeader { header, commit })
    }

    fn wire_type() -> WireType {
        WireType::LengthDelimited
    }
}

impl SignedHeaderAsProtoName of ProtoName<SignedHeader> {
    fn type_url() -> ByteArray {
        "SignedHeader"
    }
}

#[derive(Drop, Default, Debug, Clone, PartialEq, Serde)]
pub struct PublicKey {
    pub sum: Sum,
}

impl PublicKeyAsProtoMessage of ProtoMessage<PublicKey> {
    fn encode_raw(self: @PublicKey, ref context: EncodeContext) {
        context.encode_oneof(self.sum)
    }

    fn decode_raw(ref context: DecodeContext) -> Option<PublicKey> {
        let sum = context.decode_oneof()?;
        Option::Some(PublicKey { sum })
    }

    fn wire_type() -> WireType {
        WireType::LengthDelimited
    }
}

#[derive(Drop, Default, Debug, Clone, PartialEq, Serde)]
pub enum Sum {
    #[default]
    Ed25519: Array<u8>,
    Secp256k1: Array<u8>,
}

impl SumAsProtoOneof of ProtoOneof<Sum> {
    fn encode_raw(self: @Sum, ref context: EncodeContext) -> ProtobufTag {
        match self {
            Sum::Ed25519(k) => {
                k.encode_raw(ref context);
                let wire_type = ProtoMessage::<Array<u8>>::wire_type();
                ProtobufTag { field_number: 1, wire_type }
            },
            Sum::Secp256k1(k) => {
                k.encode_raw(ref context);
                let wire_type = ProtoMessage::<Array<u8>>::wire_type();
                ProtobufTag { field_number: 2, wire_type }
            },
        }
    }

    fn decode_raw(ref context: DecodeContext, tag: u8) -> Option<Sum> {
        match tag {
            0 => Option::None,
            1 => {
                let sum = context.decode_field(1)?;
                Option::Some(Sum::Ed25519(sum))
            },
            2 => {
                let sum = context.decode_field(2)?;
                Option::Some(Sum::Secp256k1(sum))
            },
            _ => Option::None,
        }
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

    fn decode_raw(ref context: DecodeContext) -> Option<Validator> {
        let address = context.decode_field(1)?;
        let pub_key = context.decode_field(2)?;
        let voting_power = context.decode_field(3)?;
        let proposer_priority = context.decode_field(4)?;
        Option::Some(Validator { address, pub_key, voting_power, proposer_priority })
    }

    fn wire_type() -> WireType {
        WireType::LengthDelimited
    }
}

impl ValidatorAsProtoName of ProtoName<Validator> {
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

    fn decode_raw(ref context: DecodeContext) -> Option<ValidatorSet> {
        let validators = context.decode_repeated_field(1)?;
        let proposer = context.decode_field(2)?;
        let total_voting_power = context.decode_field(3)?;
        Option::Some(ValidatorSet { validators, proposer, total_voting_power })
    }

    fn wire_type() -> WireType {
        WireType::LengthDelimited
    }
}

impl ValidatorSetAsProtoName of ProtoName<ValidatorSet> {
    fn type_url() -> ByteArray {
        "ValidatorSet"
    }
}
