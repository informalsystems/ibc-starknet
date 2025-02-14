use protobuf::types::wkt::Timestamp;
use protobuf::types::message::{
    ProtoMessage, ProtoCodecImpl, EncodeContext, DecodeContext, EncodeContextImpl,
    DecodeContextImpl, ProtoName,
};
use protobuf::primitives::array::{ByteArrayAsProtoMessage};
use protobuf::primitives::numeric::{
    I32AsProtoMessage, I64AsProtoMessage, BoolAsProtoMessage, U64AsProtoMessage,
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

    fn decode_raw(ref context: DecodeContext) -> Option<Consensus> {
        let mut consensus = Default::<Consensus>::default();
        if !context.decode_field(1, ref consensus.block) {
            return Option::None;
        }
        if !context.decode_field(2, ref consensus.app) {
            return Option::None;
        }
        Option::Some(consensus)
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
        let mut psh = Default::<PartSetHeader>::default();
        if !context.decode_field(1, ref psh.total) {
            return Option::None;
        }
        if !context.decode_field(2, ref psh.hash) {
            return Option::None;
        }
        Option::Some(psh)
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
        let mut block_id = Default::<BlockId>::default();
        if !context.decode_field(1, ref block_id.hash) {
            return Option::None;
        }
        if !context.decode_field(2, ref block_id.part_set_header) {
            return Option::None;
        }
        Option::Some(block_id)
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
        let mut header = Default::<Header>::default();
        if !context.decode_field(1, ref header.version) {
            return Option::None;
        }
        if !context.decode_field(2, ref header.chain_id) {
            return Option::None;
        }
        if !context.decode_field(3, ref header.height) {
            return Option::None;
        }
        if !context.decode_field(4, ref header.time) {
            return Option::None;
        }
        if !context.decode_field(5, ref header.last_block_id) {
            return Option::None;
        }
        if !context.decode_field(6, ref header.last_commit_hash) {
            return Option::None;
        }
        if !context.decode_field(7, ref header.data_hash) {
            return Option::None;
        }
        if !context.decode_field(8, ref header.validators_hash) {
            return Option::None;
        }
        if !context.decode_field(9, ref header.next_validators_hash) {
            return Option::None;
        }
        if !context.decode_field(10, ref header.consensus_hash) {
            return Option::None;
        }
        if !context.decode_field(11, ref header.app_hash) {
            return Option::None;
        }
        if !context.decode_field(12, ref header.last_results_hash) {
            return Option::None;
        }
        if !context.decode_field(13, ref header.evidence_hash) {
            return Option::None;
        }
        if !context.decode_field(14, ref header.proposer_address) {
            return Option::None;
        }
        Option::Some(header)
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

impl BlockIdFlagAsProtoMessage of ProtoMessage<BlockIdFlag> {
    fn encode_raw(self: @BlockIdFlag, ref context: EncodeContext) {
        match self {
            BlockIdFlag::Unknown => 0_u32.encode_raw(ref context),
            BlockIdFlag::Absent => 1_u32.encode_raw(ref context),
            BlockIdFlag::Commit => 2_u32.encode_raw(ref context),
            BlockIdFlag::Nil => 3_u32.encode_raw(ref context),
        }
    }

    fn decode_raw(ref context: DecodeContext) -> Option<BlockIdFlag> {
        let var: Option<u32> = context.decode_raw();
        if var.is_none() {
            return Option::None;
        }
        let value = match var.unwrap() {
            0 => Option::Some(BlockIdFlag::Unknown),
            1 => Option::Some(BlockIdFlag::Absent),
            2 => Option::Some(BlockIdFlag::Commit),
            3 => Option::Some(BlockIdFlag::Nil),
            _ => Option::None,
        };
        value
    }

    fn wire_type() -> WireType {
        WireType::Varint
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

    fn decode_raw(ref context: DecodeContext) -> Option<CommitSig> {
        let mut commit_sig = Default::<CommitSig>::default();
        if !context.decode_field(1, ref commit_sig.block_id_flag) {
            return Option::None;
        }
        if !context.decode_field(2, ref commit_sig.validator_address) {
            return Option::None;
        }
        if !context.decode_field(3, ref commit_sig.timestamp) {
            return Option::None;
        }
        if !context.decode_field(4, ref commit_sig.signature) {
            return Option::None;
        }
        Option::Some(commit_sig)
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
        let mut commit = Default::<Commit>::default();
        if !context.decode_field(1, ref commit.height) {
            return Option::None;
        }
        if !context.decode_field(2, ref commit.round) {
            return Option::None;
        }
        if !context.decode_field(3, ref commit.block_id) {
            return Option::None;
        }
        if !context.decode_repeated_field(4, ref commit.signatures) {
            return Option::None;
        }
        Option::Some(commit)
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
        let mut signed_header = Default::<SignedHeader>::default();
        if !context.decode_field(1, ref signed_header.header) {
            return Option::None;
        }
        if !context.decode_field(2, ref signed_header.commit) {
            return Option::None;
        }
        Option::Some(signed_header)
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

    fn decode_raw(ref context: DecodeContext) -> Option<PublicKey> {
        let mut public_key = Default::<PublicKey>::default();
        if !context.decode_field(1, ref public_key.ed25519) {
            return Option::None;
        }
        if !context.decode_field(2, ref public_key.secp256k1) {
            return Option::None;
        }
        Option::Some(public_key)
    }

    fn wire_type() -> WireType {
        WireType::LengthDelimited
    }
}

impl PublicKeyAsProtoName of ProtoName<PublicKey> {
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

    fn decode_raw(ref context: DecodeContext) -> Option<Validator> {
        let mut validator = Default::<Validator>::default();
        if !context.decode_field(1, ref validator.address) {
            return Option::None;
        }
        if !context.decode_field(2, ref validator.pub_key) {
            return Option::None;
        }
        if !context.decode_field(3, ref validator.voting_power) {
            return Option::None;
        }
        if !context.decode_field(4, ref validator.proposer_priority) {
            return Option::None;
        }
        Option::Some(validator)
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
        let mut set = Default::<ValidatorSet>::default();
        if !context.decode_repeated_field(1, ref set.validators) {
            return Option::None;
        }
        if !context.decode_field(2, ref set.proposer) {
            return Option::None;
        }
        if !context.decode_field(3, ref set.total_voting_power) {
            return Option::None;
        }
        Option::Some(set)
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
