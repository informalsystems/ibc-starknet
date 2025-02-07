use protobuf::types::message::DecodeContextTrait;
use protobuf::types::message::{
    ProtoMessage, ProtoCodecImpl, EncodeContext, DecodeContext, EncodeContextImpl,
    DecodeContextImpl, ProtoName
};
use protobuf::primitives::array::{
    ByteArrayAsProtoMessage, ArrayAsProtoMessage, BytesAsProtoMessage
};
use protobuf::primitives::numeric::{I32AsProtoMessage, BoolAsProtoMessage};
use protobuf::types::tag::WireType;
use ics23::{ICS23Errors, SliceU32IntoArrayU8, apply_inner, apply_leaf, iavl_spec};

#[derive(Clone, Default, Debug, Drop, PartialEq, Serde)]
pub struct CommitmentProof {
    proof: Proof,
}

impl CommitmentProofAsProtoMessage of ProtoMessage<CommitmentProof> {
    fn encode_raw(self: @CommitmentProof, ref context: EncodeContext) {
        context.encode_field(1, self.proof);
    }

    fn decode_raw(ref self: CommitmentProof, ref context: DecodeContext) {
        context.decode_field(1, ref self.proof)
    }

    fn wire_type() -> WireType {
        WireType::LengthDelimited
    }
}

/// Contains nested proof types within a commitment proof. It currently supports
/// existence and non-existence proofs to meet the core requirements of IBC. Batch
/// and compressed proofs can be added in the future if necessary.
#[derive(Clone, Default, Debug, Drop, PartialEq, Serde)]
pub enum Proof {
    #[default]
    Exist: ExistenceProof,
    NonExist: NonExistenceProof,
}

impl ProofAsProtoMessage of ProtoMessage<Proof> {
    fn encode_raw(self: @Proof, ref context: EncodeContext) {
        match self {
            Proof::Exist(p) => p.encode_raw(ref context),
            Proof::NonExist(p) => p.encode_raw(ref context),
        }
    }

    fn decode_raw(ref self: Proof, ref context: DecodeContext) {
        match self.clone() {
            Proof::Exist(mut p) => {
                p.decode_raw(ref context);
                self = Proof::Exist(p);
            },
            Proof::NonExist(mut p) => {
                p.decode_raw(ref context);
                self = Proof::NonExist(p);
            },
        }
    }

    fn wire_type() -> WireType {
        WireType::LengthDelimited
    }
}


#[derive(Clone, Default, Debug, Drop, PartialEq, Serde)]
pub struct ExistenceProof {
    pub key: Array<u8>,
    pub value: Array<u8>,
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
        for i in 0
            ..self
                .path
                .len() {
                    hash = apply_inner(self.path[i], hash.into());
                    if let Option::Some(s) = spec {
                        // NOTE: Multiplied by 4 since the hash is a u32 array, but the
                        // child size is in u8 bytes.
                        assert(
                            !(hash.span().len()
                                * 4 > *s.inner_spec.child_size && s.inner_spec.child_size >= @32),
                            ICS23Errors::INVALID_INNER_SPEC
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

    fn decode_raw(ref self: ExistenceProof, ref context: DecodeContext) {
        context.decode_field(1, ref self.key);
        context.decode_field(2, ref self.value);
        context.decode_field(3, ref self.leaf);
        context.decode_repeated_field(4, ref self.path);
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

#[derive(Clone, Default, Debug, Drop, PartialEq, Serde)]
pub struct NonExistenceProof {
    pub key: Array<u8>,
    pub left: ExistenceProof,
    pub right: ExistenceProof,
}

#[generate_trait]
pub impl NonExistenceProofImpl of NonExistenceProofTrait {
    fn calculate_root(self: @NonExistenceProof) -> RootBytes {
        self.calculate_root_for_spec(Option::None)
    }

    fn calculate_root_for_spec(self: @NonExistenceProof, spec: Option<ProofSpec>) -> RootBytes {
        [0; 8]
    }
}

impl NonExistenceProofAsProtoMessage of ProtoMessage<NonExistenceProof> {
    fn encode_raw(self: @NonExistenceProof, ref context: EncodeContext) {
        context.encode_field(1, self.key);
        context.encode_field(2, self.left);
        context.encode_field(3, self.right);
    }

    fn decode_raw(ref self: NonExistenceProof, ref context: DecodeContext) {
        context.decode_field(1, ref self.key);
        context.decode_field(2, ref self.left);
        context.decode_field(3, ref self.right);
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
        context.encode_field(1, self.hash);
        context.encode_field(2, self.prefix);
        context.encode_field(3, self.suffix);
    }

    fn decode_raw(ref self: InnerOp, ref context: DecodeContext) {
        context.decode_field(1, ref self.hash);
        context.decode_field(2, ref self.prefix);
        context.decode_field(3, ref self.suffix);
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

impl HashOpAsProtoMessage of ProtoMessage<HashOp> {
    fn encode_raw(self: @HashOp, ref context: EncodeContext) {
        match self {
            HashOp::NoOp => 0_u32.encode_raw(ref context),
            HashOp::Sha256 => 1_u32.encode_raw(ref context),
        }
    }

    fn decode_raw(ref self: HashOp, ref context: DecodeContext) {
        let mut var = Default::<u32>::default();
        var.decode_raw(ref context);
        match var {
            0 => self = HashOp::NoOp,
            1 => self = HashOp::Sha256,
            _ => panic!("invalid hash op")
        }
    }

    fn wire_type() -> WireType {
        WireType::Varint
    }
}

#[derive(Default, Debug, Copy, Drop, PartialEq, Serde)]
pub enum LengthOp {
    #[default]
    NoPrefix,
    VarProto,
}

impl LengthOpAsProtoMessage of ProtoMessage<LengthOp> {
    fn encode_raw(self: @LengthOp, ref context: EncodeContext) {
        match self {
            LengthOp::NoPrefix => 0_u32.encode_raw(ref context),
            LengthOp::VarProto => 1_u32.encode_raw(ref context),
        }
    }

    fn decode_raw(ref self: LengthOp, ref context: DecodeContext) {
        let mut var = Default::<u32>::default();
        var.decode_raw(ref context);
        match var {
            0 => self = LengthOp::NoPrefix,
            1 => self = LengthOp::VarProto,
            _ => panic!("invalid length op"),
        }
    }

    fn wire_type() -> WireType {
        WireType::Varint
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
        context.encode_field(6, self.hash);
    }

    fn decode_raw(ref self: InnerSpec, ref context: DecodeContext) {
        context.decode_field(1, ref self.child_order);
        context.decode_field(2, ref self.child_size);
        context.decode_field(3, ref self.min_prefix_length);
        context.decode_field(4, ref self.max_prefix_length);
        context.decode_field(5, ref self.empty_child);
        context.decode_field(6, ref self.hash);
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
        context.encode_field(1, self.hash);
        context.encode_field(2, self.prehash_key);
        context.encode_field(3, self.prehash_value);
        context.encode_field(4, self.length);
        context.encode_field(5, self.prefix);
    }

    fn decode_raw(ref self: LeafOp, ref context: DecodeContext) {
        context.decode_field(1, ref self.hash);
        context.decode_field(2, ref self.prehash_key);
        context.decode_field(3, ref self.prehash_value);
        context.decode_field(4, ref self.length);
        context.decode_field(5, ref self.prefix);
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
}

impl ProofSpecAsProtoMessage of ProtoMessage<ProofSpec> {
    fn encode_raw(self: @ProofSpec, ref context: EncodeContext) {
        context.encode_field(1, self.leaf_spec);
        context.encode_field(2, self.inner_spec);
        context.encode_field(3, self.max_depth);
        context.encode_field(4, self.min_depth);
        context.encode_field(5, self.prehash_key_before_comparison);
    }

    fn decode_raw(ref self: ProofSpec, ref context: DecodeContext) {
        context.decode_field(1, ref self.leaf_spec);
        context.decode_field(2, ref self.inner_spec);
        context.decode_field(3, ref self.max_depth);
        context.decode_field(4, ref self.min_depth);
        context.decode_field(5, ref self.prehash_key_before_comparison);
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
