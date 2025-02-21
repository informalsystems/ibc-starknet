use protobuf::types::message::{
    ProtoMessage, ProtoOneof, ProtoCodecImpl, EncodeContext, DecodeContext, EncodeContextImpl,
    DecodeContextImpl, ProtoName, EncodeContextTrait, DecodeContextTrait,
};
use protobuf::primitives::array::{
    ByteArrayAsProtoMessage, ArrayAsProtoMessage, BytesAsProtoMessage,
};
use protobuf::primitives::numeric::{I32AsProtoMessage, BoolAsProtoMessage};
use protobuf::types::tag::{WireType, ProtobufTag};
use ics23::{ICS23Errors, SliceU32IntoArrayU8, apply_inner, apply_leaf, iavl_spec, hash_u32_array};

#[derive(Default, Debug, Drop, PartialEq, Serde)]
pub struct CommitmentProof {
    pub proof: Proof,
}

impl CommitmentProofAsProtoMessage of ProtoMessage<CommitmentProof> {
    fn encode_raw(self: @CommitmentProof, ref context: EncodeContext) {
        context.encode_oneof(self.proof)
    }

    fn decode_raw(ref context: DecodeContext) -> Option<CommitmentProof> {
        let proof = context.decode_oneof()?;
        Option::Some(CommitmentProof { proof })
    }

    fn wire_type() -> WireType {
        WireType::LengthDelimited
    }
}

/// Contains nested proof types within a commitment proof. It currently supports
/// existence and non-existence proofs to meet the core requirements of IBC. Batch
/// and compressed proofs can be added in the future if necessary.
#[derive(Default, Debug, Drop, PartialEq, Serde)]
pub enum Proof {
    #[default]
    Exist: ExistenceProof,
    NonExist: NonExistenceProof,
}

impl ProofAsProtoOneof of ProtoOneof<Proof> {
    fn encode_raw(self: @Proof, ref context: EncodeContext) -> ProtobufTag {
        match self {
            Proof::Exist(p) => {
                p.encode_raw(ref context);
                let wire_type = ProtoMessage::<ExistenceProof>::wire_type();
                ProtobufTag { field_number: 1, wire_type }
            },
            Proof::NonExist(p) => {
                p.encode_raw(ref context);
                let wire_type = ProtoMessage::<NonExistenceProof>::wire_type();
                ProtobufTag { field_number: 2, wire_type }
            },
        }
    }

    fn decode_raw(ref context: DecodeContext, tag: u8) -> Option<Proof> {
        match tag {
            0 => Option::None,
            1 => {
                let proof = context.decode_field(1)?;
                Option::Some(Proof::Exist(proof))
            },
            2 => {
                let proof = context.decode_field(2)?;
                Option::Some(Proof::NonExist(proof))
            },
            _ => Option::None,
        }
    }
}

#[derive(Clone, Default, Debug, Drop, PartialEq, Serde)]
pub struct ExistenceProof {
    pub key: KeyBytes,
    pub value: ValueBytes,
    pub leaf: LeafOp,
    pub path: Array<InnerOp>,
}

#[generate_trait]
pub impl ExistenceProofImpl of ExistenceProofTrait {
    fn calculate_root(self: @ExistenceProof) -> RootBytes {
        self.calculate_root_for_spec(Option::None)
    }

    fn calculate_root_for_spec(self: @ExistenceProof, spec: Option<@ProofSpec>) -> RootBytes {
        assert(self.key.len() > 0, ICS23Errors::MISSING_KEY);
        assert(self.value.len() > 0, ICS23Errors::MISSING_VALUE);
        let mut hash = apply_leaf(self.leaf, self.key.clone(), self.value.clone());
        for i in 0..self.path.len() {
            hash = apply_inner(self.path[i], hash.into());
            if let Option::Some(s) = spec {
                // NOTE: Multiplied by 4 since the hash is a u32 array, but the
                // child size is in u8 bytes.
                assert(
                    !(hash.span().len()
                        * 4 > *s.inner_spec.child_size && s.inner_spec.child_size >= @32),
                    ICS23Errors::INVALID_INNER_SPEC,
                );
            }
        };
        hash
    }
}

impl ExistenceProofAsProtoMessage of ProtoMessage<ExistenceProof> {
    fn encode_raw(self: @ExistenceProof, ref context: EncodeContext) {
        context.encode_field(1, self.key);
        context.encode_field(2, self.value);
        context.encode_field(3, self.leaf);
        context.encode_repeated_field(4, self.path);
    }

    fn decode_raw(ref context: DecodeContext) -> Option<ExistenceProof> {
        let key = context.decode_field(1)?;
        let value = context.decode_field(2)?;
        let leaf = context.decode_field(3)?;
        let path = context.decode_repeated_field(4)?;
        Option::Some(ExistenceProof { key, value, leaf, path })
    }

    fn wire_type() -> WireType {
        WireType::LengthDelimited
    }
}

impl ExistenceProofAsProtoName of ProtoName<ExistenceProof> {
    fn type_url() -> ByteArray {
        "ExistenceProof"
    }
}

#[derive(Default, Debug, Drop, PartialEq, Serde)]
pub struct NonExistenceProof {
    pub key: Array<u8>,
    pub left: Option<ExistenceProof>,
    pub right: Option<ExistenceProof>,
}

#[generate_trait]
pub impl NonExistenceProofImpl of NonExistenceProofTrait {
    fn calculate_root(self: @NonExistenceProof) -> RootBytes {
        if let Option::Some(left) = self.left {
            left.calculate_root()
        } else if let Option::Some(right) = self.right {
            right.calculate_root()
        } else {
            panic!("{}", ICS23Errors::MISSING_EXISTENCE_PROOFS)
        }
    }
}

impl NonExistenceProofAsProtoMessage of ProtoMessage<NonExistenceProof> {
    fn encode_raw(self: @NonExistenceProof, ref context: EncodeContext) {
        context.encode_field(1, self.key);
        context.encode_optional_field(2, self.left);
        context.encode_optional_field(3, self.right);
    }

    fn decode_raw(ref context: DecodeContext) -> Option<NonExistenceProof> {
        let key = context.decode_field(1)?;
        let left = context.decode_optional_field(2);
        let right = context.decode_optional_field(3);
        Option::Some(NonExistenceProof { key, left, right })
    }

    fn wire_type() -> WireType {
        WireType::LengthDelimited
    }
}

impl NonExistenceProofAsProtoName of ProtoName<NonExistenceProof> {
    fn type_url() -> ByteArray {
        "NonExistenceProof"
    }
}

#[derive(Clone, Default, Debug, Drop, PartialEq, Serde)]
pub struct InnerOp {
    pub hash: HashOp,
    pub prefix: Array<u8>,
    pub suffix: Array<u8>,
}

impl InnerOpAsProtoMessage of ProtoMessage<InnerOp> {
    fn encode_raw(self: @InnerOp, ref context: EncodeContext) {
        context.encode_enum(1, self.hash);
        context.encode_field(2, self.prefix);
        context.encode_field(3, self.suffix);
    }

    fn decode_raw(ref context: DecodeContext) -> Option<InnerOp> {
        let hash = context.decode_enum(1)?;
        let prefix = context.decode_field(2)?;
        let suffix = context.decode_field(3)?;
        Option::Some(InnerOp { hash, prefix, suffix })
    }

    fn wire_type() -> WireType {
        WireType::LengthDelimited
    }
}

impl InnerOpAsProtoName of ProtoName<InnerOp> {
    fn type_url() -> ByteArray {
        "InnerOp"
    }
}

#[derive(Default, Debug, Copy, Drop, PartialEq, Serde)]
pub enum HashOp {
    #[default]
    NoOp,
    Sha256,
}

pub impl HashOpIntoU32 of Into<@HashOp, u32> {
    fn into(self: @HashOp) -> u32 {
        match self {
            HashOp::NoOp => 0,
            HashOp::Sha256 => 1,
        }
    }
}

pub impl U32TryIntoHashOp of TryInto<u32, HashOp> {
    fn try_into(self: u32) -> Option<HashOp> {
        match self {
            0 => Option::Some(HashOp::NoOp),
            1 => Option::Some(HashOp::Sha256),
            _ => Option::None,
        }
    }
}

#[derive(Default, Debug, Copy, Drop, PartialEq, Serde)]
pub enum LengthOp {
    #[default]
    NoPrefix,
    VarProto,
}

pub impl LengthOpIntoU32 of Into<@LengthOp, u32> {
    fn into(self: @LengthOp) -> u32 {
        match self {
            LengthOp::NoPrefix => 0,
            LengthOp::VarProto => 1,
        }
    }
}

pub impl U32TryIntoLegthOp of TryInto<u32, LengthOp> {
    fn try_into(self: u32) -> Option<LengthOp> {
        match self {
            0 => Option::Some(LengthOp::NoPrefix),
            1 => Option::Some(LengthOp::VarProto),
            _ => Option::None,
        }
    }
}

#[derive(Default, Debug, Clone, Drop, PartialEq, Serde)]
pub struct InnerSpec {
    pub child_order: Array<u32>,
    pub child_size: u32,
    pub min_prefix_length: u32,
    pub max_prefix_length: u32,
    pub empty_child: ByteArray, // TODO: determine the correct type!
    pub hash: HashOp,
}

impl InnerSpecAsProtoMessage of ProtoMessage<InnerSpec> {
    fn encode_raw(self: @InnerSpec, ref context: EncodeContext) {
        context.encode_field(1, self.child_order);
        context.encode_field(2, self.child_size);
        context.encode_field(3, self.min_prefix_length);
        context.encode_field(4, self.max_prefix_length);
        context.encode_field(5, self.empty_child);
        context.encode_enum(6, self.hash);
    }

    fn decode_raw(ref context: DecodeContext) -> Option<InnerSpec> {
        let child_order = context.decode_field(1)?;
        let child_size = context.decode_field(2)?;
        let min_prefix_length = context.decode_field(3)?;
        let max_prefix_length = context.decode_field(4)?;
        let empty_child = context.decode_field(5)?;
        let hash = context.decode_enum(6)?;
        Option::Some(
            InnerSpec {
                child_order, child_size, min_prefix_length, max_prefix_length, empty_child, hash,
            },
        )
    }

    fn wire_type() -> WireType {
        WireType::LengthDelimited
    }
}

impl InnerSpecAsProtoName of ProtoName<InnerSpec> {
    fn type_url() -> ByteArray {
        "InnerSpec"
    }
}

#[derive(Default, Debug, Clone, Drop, PartialEq, Serde)]
pub struct LeafOp {
    pub hash: HashOp,
    pub prehash_key: HashOp,
    pub prehash_value: HashOp,
    pub length: LengthOp,
    pub prefix: Array<u8>,
}

impl LeafOpAsProtoMessage of ProtoMessage<LeafOp> {
    fn encode_raw(self: @LeafOp, ref context: EncodeContext) {
        context.encode_enum(1, self.hash);
        context.encode_enum(2, self.prehash_key);
        context.encode_enum(3, self.prehash_value);
        context.encode_enum(4, self.length);
        context.encode_field(5, self.prefix);
    }

    fn decode_raw(ref context: DecodeContext) -> Option<LeafOp> {
        let hash = context.decode_enum(1)?;
        let prehash_key = context.decode_enum(2)?;
        let prehash_value = context.decode_enum(3)?;
        let length = context.decode_enum(4)?;
        let prefix = context.decode_field(5)?;
        Option::Some(LeafOp { hash, prehash_key, prehash_value, length, prefix })
    }

    fn wire_type() -> WireType {
        WireType::LengthDelimited
    }
}

impl LeafOpAsProtoName of ProtoName<LeafOp> {
    fn type_url() -> ByteArray {
        "LeafOp"
    }
}

#[derive(Default, Debug, Clone, Drop, PartialEq, Serde)]
pub struct ProofSpec {
    pub leaf_spec: LeafOp,
    pub inner_spec: InnerSpec,
    pub max_depth: u32,
    pub min_depth: u32,
    pub prehash_key_before_comparison: bool,
}

#[generate_trait]
pub impl ProofSpecImpl of ProofSpecTrait {
    fn validate(self: @ProofSpec) {
        assert(self.max_depth < @0, ICS23Errors::INVALID_DEPTH_RANGE);
        assert(self.min_depth < @0, ICS23Errors::INVALID_DEPTH_RANGE);
        assert(self.max_depth > self.min_depth, ICS23Errors::INVALID_DEPTH_RANGE);
    }

    fn is_iavl(self: @ProofSpec) -> bool {
        let iavl_spec = iavl_spec();
        @iavl_spec.leaf_spec == self.leaf_spec && @iavl_spec.inner_spec == self.inner_spec
    }

    fn key_for_comparison(self: @ProofSpec, key: Array<u8>) -> Array<u8> {
        match self.prehash_key_before_comparison {
            true => hash_u32_array(self.leaf_spec.prehash_key, key),
            false => key,
        }
    }
}

impl ProofSpecAsProtoMessage of ProtoMessage<ProofSpec> {
    fn encode_raw(self: @ProofSpec, ref context: EncodeContext) {
        context.encode_field(1, self.leaf_spec);
        context.encode_field(2, self.inner_spec);
        context.encode_field(3, self.max_depth);
        context.encode_field(4, self.min_depth);
        context.encode_field(5, self.prehash_key_before_comparison);
    }

    fn decode_raw(ref context: DecodeContext) -> Option<ProofSpec> {
        let leaf_spec = context.decode_field(1)?;
        let inner_spec = context.decode_field(2)?;
        let max_depth = context.decode_field(3)?;
        let min_depth = context.decode_field(4)?;
        let prehash_key_before_comparison = context.decode_field(5)?;
        Option::Some(
            ProofSpec {
                leaf_spec, inner_spec, max_depth, min_depth, prehash_key_before_comparison,
            },
        )
    }

    fn wire_type() -> WireType {
        WireType::LengthDelimited
    }
}

impl ProofSpecAsProtoName of ProtoName<ProofSpec> {
    fn type_url() -> ByteArray {
        "ProofSpec"
    }
}

pub type RootBytes = [u32; 8];
pub type KeyBytes = Array<u8>;
pub type ValueBytes = Array<u8>;
