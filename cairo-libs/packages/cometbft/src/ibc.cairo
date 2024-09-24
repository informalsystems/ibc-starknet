use protobuf::types::message::{
    ProtoMessage, ProtoCodecImpl, EncodeContext, DecodeContext, EncodeContextImpl,
    DecodeContextImpl, ProtoName
};
use protobuf::primitives::array::ByteArrayAsProtoMessage;
use protobuf::primitives::numeric::UnsignedAsProtoMessage;
use protobuf::types::tag::WireType;

#[derive(Default, Debug, Copy, Drop, PartialEq, Serde)]
pub struct Height {
    pub revision_number: u64,
    pub revision_height: u64,
}

impl HeightAsProtoMessage of ProtoMessage<Height> {
    fn encode_raw(self: @Height, ref context: EncodeContext) {
        context.encode_field(1, self.revision_number);
        context.encode_field(2, self.revision_height);
    }

    fn decode_raw(ref self: Height, ref context: DecodeContext) {
        context.decode_field(1, ref self.revision_number);
        context.decode_field(2, ref self.revision_height);
    }

    fn wire_type() -> WireType {
        WireType::LengthDelimited
    }
}

impl HeightAsProtoName of ProtoName<Height> {
    fn type_url() -> ByteArray {
        "ibc.core.client.v1.Height"
    }
}

#[derive(Default, Debug, Clone, Drop, PartialEq, Serde)]
pub struct MerkleRoot {
    pub hash: ByteArray,
}

impl MerkleRootAsProtoMessage of ProtoMessage<MerkleRoot> {
    fn encode_raw(self: @MerkleRoot, ref context: EncodeContext) {
        context.encode_field(1, self.hash);
    }

    fn decode_raw(ref self: MerkleRoot, ref context: DecodeContext) {
        context.decode_field(1, ref self.hash);
    }

    fn wire_type() -> WireType {
        WireType::LengthDelimited
    }
}

impl MerkleRootAsProtoName of ProtoName<MerkleRoot> {
    fn type_url() -> ByteArray {
        "ibc.core.commitment.v1.MerkleRoot"
    }
}