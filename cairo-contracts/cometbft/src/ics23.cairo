use protobuf::types::message::{
    ProtoMessage, ProtoCodecImpl, EncodeContext, DecodeContext, EncodeContextImpl,
    DecodeContextImpl, Name
};
use protobuf::primitives::array::{ByteArrayAsProtoMessage};
use protobuf::primitives::numeric::{UnsignedAsProtoMessage, I32AsProtoMessage, BoolAsProtoMessage};
use protobuf::types::tag::WireType;

#[derive(Default, Debug, Copy, Drop, PartialEq, Serde)]
pub enum HashOp {
    #[default]
    NoOp,
    Sha256,
    Sha512,
    Keccak256,
    Ripemd160,
    Bitcoin,
    Sha512_256,
    Blake2b_512,
    Blake2b_256,
    Blake3,
}

impl HashOpIntoU64 of Into<HashOp, u64> {
    fn into(self: HashOp) -> u64 {
        match self {
            HashOp::NoOp => 0,
            HashOp::Sha256 => 1,
            HashOp::Sha512 => 2,
            HashOp::Keccak256 => 3,
            HashOp::Ripemd160 => 4,
            HashOp::Bitcoin => 5,
            HashOp::Sha512_256 => 6,
            HashOp::Blake2b_512 => 7,
            HashOp::Blake2b_256 => 8,
            HashOp::Blake3 => 9,
        }
    }
}

impl U64IntoHashOp of Into<u64, HashOp> {
    fn into(self: u64) -> HashOp {
        match self {
            0 => HashOp::NoOp,
            1 => HashOp::Sha256,
            2 => HashOp::Sha512,
            3 => HashOp::Keccak256,
            4 => HashOp::Ripemd160,
            5 => HashOp::Bitcoin,
            6 => HashOp::Sha512_256,
            7 => HashOp::Blake2b_512,
            8 => HashOp::Blake2b_256,
            9 => HashOp::Blake3,
            _ => panic!("invalid HashOp"),
        }
    }
}


#[derive(Default, Debug, Copy, Drop, PartialEq, Serde)]
pub enum LengthOp {
    #[default]
    NoPrefix,
    VarProto,
    VarRlp,
    Fixed32Big,
    Fixed32Little,
    Fixed64Big,
    Fixed64Little,
    Require32Bytes,
    Require64Bytes,
}

impl LengthOpIntoU64 of Into<LengthOp, u64> {
    fn into(self: LengthOp) -> u64 {
        match self {
            LengthOp::NoPrefix => 0,
            LengthOp::VarProto => 1,
            LengthOp::VarRlp => 2,
            LengthOp::Fixed32Big => 3,
            LengthOp::Fixed32Little => 4,
            LengthOp::Fixed64Big => 5,
            LengthOp::Fixed64Little => 6,
            LengthOp::Require32Bytes => 7,
            LengthOp::Require64Bytes => 8,
        }
    }
}

impl U64IntoLengthOp of Into<u64, LengthOp> {
    fn into(self: u64) -> LengthOp {
        match self {
            0 => LengthOp::NoPrefix,
            1 => LengthOp::VarProto,
            2 => LengthOp::VarRlp,
            3 => LengthOp::Fixed32Big,
            4 => LengthOp::Fixed32Little,
            5 => LengthOp::Fixed64Big,
            6 => LengthOp::Fixed64Little,
            7 => LengthOp::Require32Bytes,
            8 => LengthOp::Require64Bytes,
            _ => panic!("invalid length op"),
        }
    }
}

#[derive(Default, Debug, Clone, Drop, PartialEq, Serde)]
pub struct InnerSpec {
    pub child_order: Array<i32>,
    pub child_size: i32,
    pub min_prefix_length: i32,
    pub max_prefix_length: i32,
    pub empty_child: ByteArray,
    pub hash: HashOp,
}

impl InnerSpecAsProtoMessage of ProtoMessage<InnerSpec> {
    fn encode_raw(self: @InnerSpec, ref context: EncodeContext) {
        context.encode_repeated_field(1, self.child_order);
        context.encode_field(2, self.child_size);
        context.encode_field(3, self.min_prefix_length);
        context.encode_field(4, self.max_prefix_length);
        context.encode_field(5, self.empty_child);
        context.encode_field(6, self.hash);
    }

    fn decode_raw(ref self: InnerSpec, ref context: DecodeContext, length: usize) {
        context.init_branch(length);
        context.decode_repeated_field(1, ref self.child_order);
        context.decode_field(2, ref self.child_size);
        context.decode_field(3, ref self.min_prefix_length);
        context.decode_field(4, ref self.max_prefix_length);
        context.decode_field(5, ref self.empty_child);
        context.decode_field(6, ref self.hash);
        context.end_branch();
    }

    fn wire_type() -> WireType {
        WireType::LengthDelimited
    }
}

impl InnerSpecAsName of Name<InnerSpec> {
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
    pub prefix: ByteArray,
}

impl LeafOpAsProtoMessage of ProtoMessage<LeafOp> {
    fn encode_raw(self: @LeafOp, ref context: EncodeContext) {
        context.encode_field(1, self.hash);
        context.encode_field(2, self.prehash_key);
        context.encode_field(3, self.prehash_value);
        context.encode_field(4, self.length);
        context.encode_field(5, self.prefix);
    }

    fn decode_raw(ref self: LeafOp, ref context: DecodeContext, length: usize) {
        context.init_branch(length);
        context.decode_field(1, ref self.hash);
        context.decode_field(2, ref self.prehash_key);
        context.decode_field(3, ref self.prehash_value);
        context.decode_field(4, ref self.length);
        context.decode_field(5, ref self.prefix);
        context.end_branch();
    }

    fn wire_type() -> WireType {
        WireType::LengthDelimited
    }
}

impl LeafOpAsName of Name<LeafOp> {
    fn type_url() -> ByteArray {
        "LeafOp"
    }
}

#[derive(Default, Debug, Clone, Drop, PartialEq, Serde)]
pub struct ProofSpec {
    pub leaf_spec: LeafOp,
    pub inner_spec: InnerSpec,
    pub max_depth: i32,
    pub min_depth: i32,
    pub prehash_key_before_comparison: bool,
}

impl ProofSpecAsProtoMessage of ProtoMessage<ProofSpec> {
    fn encode_raw(self: @ProofSpec, ref context: EncodeContext) {
        context.encode_field(1, self.leaf_spec);
        context.encode_field(2, self.inner_spec);
        context.encode_field(3, self.max_depth);
        context.encode_field(4, self.min_depth);
        context.encode_field(5, self.prehash_key_before_comparison);
    }

    fn decode_raw(ref self: ProofSpec, ref context: DecodeContext, length: usize) {
        context.init_branch(length);
        context.decode_field(1, ref self.leaf_spec);
        context.decode_field(2, ref self.inner_spec);
        context.decode_field(3, ref self.max_depth);
        context.decode_field(4, ref self.min_depth);
        context.decode_field(5, ref self.prehash_key_before_comparison);
        context.end_branch();
    }

    fn wire_type() -> WireType {
        WireType::LengthDelimited
    }
}

impl ProofSpecAsName of Name<ProofSpec> {
    fn type_url() -> ByteArray {
        "ProofSpec"
    }
}
