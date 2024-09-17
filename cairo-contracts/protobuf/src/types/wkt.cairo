use protobuf::types::message::{ProtoMessage, ProtoCodecImpl};
use protobuf::types::tag::WireType;
use protobuf::primitives::numeric::{I32AsProtoMessage, I64AsProtoMessage};
use protobuf::primitives::array::ByteArrayAsProtoMessage;

#[derive(Default, Debug, Copy, Drop, PartialEq, Serde)]
pub struct Duration {
    pub seconds: i64,
    pub nanos: i32,
}

impl DuractionAsProtoMessage of ProtoMessage<Duration> {
    fn encode_raw(self: @Duration, ref output: ByteArray) {
        ProtoCodecImpl::encode_length_delimited_raw(1, self.seconds, ref output);
        ProtoCodecImpl::encode_length_delimited_raw(2, self.nanos, ref output);
    }

    fn decode_raw(ref value: Duration, serialized: @ByteArray, ref index: usize, length: usize) {
        let bound = index + length;

        ProtoCodecImpl::decode_length_delimited_raw(1, ref value.seconds, serialized, ref index);
        ProtoCodecImpl::decode_length_delimited_raw(2, ref value.nanos, serialized, ref index);

        assert(index == bound, 'invalid length for Duration');
    }

    fn wire_type() -> WireType {
        WireType::LengthDelimited
    }
}

#[derive(Default, Debug, Copy, Drop, PartialEq, Serde)]
pub struct Timestamp {
    pub seconds: i64,
    pub nanos: i32,
}

impl TimestampAsProtoMessage of ProtoMessage<Timestamp> {
    fn encode_raw(self: @Timestamp, ref output: ByteArray) {
        ProtoCodecImpl::encode_length_delimited_raw(1, self.seconds, ref output);
        ProtoCodecImpl::encode_length_delimited_raw(2, self.nanos, ref output);
    }

    fn decode_raw(ref value: Timestamp, serialized: @ByteArray, ref index: usize, length: usize) {
        let bound = index + length;

        ProtoCodecImpl::decode_length_delimited_raw(1, ref value.seconds, serialized, ref index);
        ProtoCodecImpl::decode_length_delimited_raw(2, ref value.nanos, serialized, ref index);

        assert(index == bound, 'invalid length for Timestamp');
    }

    fn wire_type() -> WireType {
        WireType::LengthDelimited
    }
}

#[derive(Default, Debug, Drop, PartialEq, Serde)]
pub struct Any {
    pub type_url: ByteArray,
    pub value: ByteArray,
}

impl AnyAsProtoMessage of ProtoMessage<Any> {
    fn encode_raw(self: @Any, ref output: ByteArray) {
        ProtoCodecImpl::encode_length_delimited_raw(1, self.type_url, ref output);
        ProtoCodecImpl::encode_length_delimited_raw(2, self.value, ref output);
    }

    fn decode_raw(ref value: Any, serialized: @ByteArray, ref index: usize, length: usize) {
        let bound = index + length;

        ProtoCodecImpl::decode_length_delimited_raw(1, ref value.type_url, serialized, ref index);
        ProtoCodecImpl::decode_length_delimited_raw(2, ref value.value, serialized, ref index);

        assert(index == bound, 'invalid length for Any');
    }

    fn wire_type() -> WireType {
        WireType::LengthDelimited
    }
}
