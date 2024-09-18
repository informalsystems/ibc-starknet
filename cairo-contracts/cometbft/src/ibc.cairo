use protobuf::types::message::{ProtoMessage, ProtoCodecImpl};
use protobuf::primitives::array::ByteArrayAsProtoMessage;
use protobuf::primitives::numeric::NumberAsProtoMessage;
use protobuf::types::tag::WireType;

#[derive(Default, Debug, Copy, Drop, PartialEq, Serde)]
pub struct Height {
    pub revision_number: u64,
    pub revision_height: u64,
}

impl HeightAsProtoMessage of ProtoMessage<Height> {
    fn encode_raw(self: @Height, ref output: ByteArray) {
        ProtoCodecImpl::encode_field(1, self.revision_number, ref output);
        ProtoCodecImpl::encode_field(2, self.revision_height, ref output);
    }

    fn decode_raw(ref value: Height, serialized: @ByteArray, ref index: usize, length: usize) {
        let bound = index + length;
        ProtoCodecImpl::decode_field(1, ref value.revision_number, serialized, ref index, bound);
        ProtoCodecImpl::decode_field(2, ref value.revision_height, serialized, ref index, bound);
        assert(index == bound, 'invalid length for Height');
    }

    fn wire_type() -> WireType {
        WireType::LengthDelimited
    }

    fn type_url() -> ByteArray {
        "Height"
    }
}

#[derive(Default, Debug, Clone, Drop, PartialEq, Serde)]
pub struct MerkleRoot {
    pub hash: ByteArray,
}

impl MerkleRootAsProtoMessage of ProtoMessage<MerkleRoot> {
    fn encode_raw(self: @MerkleRoot, ref output: ByteArray) {
        ProtoCodecImpl::encode_field(1, self.hash, ref output);
    }

    fn decode_raw(ref value: MerkleRoot, serialized: @ByteArray, ref index: usize, length: usize) {
        let bound = index + length;
        ProtoCodecImpl::decode_field(1, ref value.hash, serialized, ref index, bound);
        assert(index == bound, 'invalid length for MerkleRoot');
    }

    fn wire_type() -> WireType {
        WireType::LengthDelimited
    }

    fn type_url() -> ByteArray {
        "MerkleRoot"
    }
}
