use protobuf::types::message::{ProtoMessage, ProtoCodecImpl};
use protobuf::primitives::array::{ByteArrayAsProtoMessage};
use protobuf::primitives::numeric::{NumberAsProtoMessage, I32AsProtoMessage, BoolAsProtoMessage};
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
    fn encode_raw(self: @InnerSpec, ref output: ByteArray) {
        ProtoCodecImpl::encode_repeated_field(1, self.child_order, ref output);
        ProtoCodecImpl::encode_field(2, self.child_size, ref output);
        ProtoCodecImpl::encode_field(3, self.min_prefix_length, ref output);
        ProtoCodecImpl::encode_field(4, self.max_prefix_length, ref output);
        ProtoCodecImpl::encode_field(5, self.empty_child, ref output);
        ProtoCodecImpl::encode_field(6, self.hash, ref output);
    }

    fn decode_raw(ref value: InnerSpec, serialized: @ByteArray, ref index: usize, length: usize) {
        let bound = index + length;
        ProtoCodecImpl::decode_repeated_field(
            1, ref value.child_order, serialized, ref index, bound
        );
        ProtoCodecImpl::decode_field(2, ref value.child_size, serialized, ref index, bound);
        ProtoCodecImpl::decode_field(3, ref value.min_prefix_length, serialized, ref index, bound);
        ProtoCodecImpl::decode_field(4, ref value.max_prefix_length, serialized, ref index, bound);
        ProtoCodecImpl::decode_field(5, ref value.empty_child, serialized, ref index, bound);
        ProtoCodecImpl::decode_field(6, ref value.hash, serialized, ref index, bound);
        assert(index == bound, 'invalid length for InnerSpec');
    }

    fn wire_type() -> WireType {
        WireType::LengthDelimited
    }

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
    fn encode_raw(self: @LeafOp, ref output: ByteArray) {
        ProtoCodecImpl::encode_field(1, self.hash, ref output);
        ProtoCodecImpl::encode_field(2, self.prehash_key, ref output);
        ProtoCodecImpl::encode_field(3, self.prehash_value, ref output);
        ProtoCodecImpl::encode_field(4, self.length, ref output);
        ProtoCodecImpl::encode_field(5, self.prefix, ref output);
    }

    fn decode_raw(ref value: LeafOp, serialized: @ByteArray, ref index: usize, length: usize) {
        let bound = index + length;
        ProtoCodecImpl::decode_field(1, ref value.hash, serialized, ref index, bound);
        ProtoCodecImpl::decode_field(2, ref value.prehash_key, serialized, ref index, bound);
        ProtoCodecImpl::decode_field(3, ref value.prehash_value, serialized, ref index, bound);
        ProtoCodecImpl::decode_field(4, ref value.length, serialized, ref index, bound);
        ProtoCodecImpl::decode_field(5, ref value.prefix, serialized, ref index, bound);
        assert(index == bound, 'invalid length for LeafOp');
    }

    fn wire_type() -> WireType {
        WireType::LengthDelimited
    }

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
    fn encode_raw(self: @ProofSpec, ref output: ByteArray) {
        ProtoCodecImpl::encode_field(1, self.leaf_spec, ref output);
        ProtoCodecImpl::encode_field(2, self.inner_spec, ref output);
        ProtoCodecImpl::encode_field(3, self.max_depth, ref output);
        ProtoCodecImpl::encode_field(4, self.min_depth, ref output);
        ProtoCodecImpl::encode_field(5, self.prehash_key_before_comparison, ref output);
    }

    fn decode_raw(ref value: ProofSpec, serialized: @ByteArray, ref index: usize, length: usize) {
        let bound = index + length;
        ProtoCodecImpl::decode_field(1, ref value.leaf_spec, serialized, ref index, bound);
        ProtoCodecImpl::decode_field(2, ref value.inner_spec, serialized, ref index, bound);
        ProtoCodecImpl::decode_field(3, ref value.max_depth, serialized, ref index, bound);
        ProtoCodecImpl::decode_field(4, ref value.min_depth, serialized, ref index, bound);
        ProtoCodecImpl::decode_field(
            5, ref value.prehash_key_before_comparison, serialized, ref index, bound
        );
        assert(index == bound, 'invalid length for ProofSpec');
    }

    fn wire_type() -> WireType {
        WireType::LengthDelimited
    }

    fn type_url() -> ByteArray {
        "ProofSpec"
    }
}
