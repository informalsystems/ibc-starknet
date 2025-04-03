use protobuf::primitives::array::ByteArrayAsProtoMessage;
use protobuf::primitives::numeric::{I32AsProtoMessage, I64AsProtoMessage};
use protobuf::types::message::{
    DecodeContext, DecodeContextImpl, EncodeContext, EncodeContextImpl, ProtoCodecImpl,
    ProtoMessage, ProtoName,
};
use protobuf::types::tag::WireType;

#[derive(Default, Debug, Copy, Drop, PartialEq, Serde)]
pub struct Duration {
    pub seconds: i64,
    pub nanos: i32,
}

impl DurationAsProtoMessage of ProtoMessage<Duration> {
    fn encode_raw(self: @Duration, ref context: EncodeContext) {
        context.encode_field(1, self.seconds);
        context.encode_field(2, self.nanos);
    }

    fn decode_raw(ref context: DecodeContext) -> Option<Duration> {
        let seconds = context.decode_field(1)?;
        let nanos = context.decode_field(2)?;
        Option::Some(Duration { seconds, nanos })
    }

    fn wire_type() -> WireType {
        WireType::LengthDelimited
    }
}

impl DurationAsProtoName of ProtoName<Duration> {
    fn type_url() -> ByteArray {
        "type.googleapis.com/google.protobuf.Duration"
    }
}

#[derive(Default, Debug, Copy, Drop, PartialEq, Serde)]
pub struct Timestamp {
    pub seconds: i64,
    pub nanos: i32,
}

impl TimestampPartialOrd of PartialOrd<Timestamp> {
    fn lt(lhs: Timestamp, rhs: Timestamp) -> bool {
        (lhs.seconds < rhs.seconds) || (lhs.seconds == rhs.seconds && lhs.nanos < rhs.nanos)
    }
    fn le(lhs: Timestamp, rhs: Timestamp) -> bool {
        (lhs.seconds < rhs.seconds) || (lhs.seconds == rhs.seconds && lhs.nanos <= rhs.nanos)
    }
    fn gt(lhs: Timestamp, rhs: Timestamp) -> bool {
        (lhs.seconds > rhs.seconds) || (lhs.seconds == rhs.seconds && lhs.nanos > rhs.nanos)
    }
    fn ge(lhs: Timestamp, rhs: Timestamp) -> bool {
        (lhs.seconds > rhs.seconds) || (lhs.seconds == rhs.seconds && lhs.nanos >= rhs.nanos)
    }
}

impl TimestampAsProtoMessage of ProtoMessage<Timestamp> {
    fn encode_raw(self: @Timestamp, ref context: EncodeContext) {
        context.encode_field(1, self.seconds);
        context.encode_field(2, self.nanos);
    }

    fn decode_raw(ref context: DecodeContext) -> Option<Timestamp> {
        let seconds = context.decode_field(1)?;
        let nanos = context.decode_field(2)?;
        Option::Some(Timestamp { seconds, nanos })
    }

    fn wire_type() -> WireType {
        WireType::LengthDelimited
    }
}

impl TimestampAsProtoName of ProtoName<Timestamp> {
    fn type_url() -> ByteArray {
        "type.googleapis.com/google.protobuf.Timestamp"
    }
}

#[derive(Default, Debug, Clone, Drop, PartialEq, Serde)]
pub struct Any {
    pub type_url: ByteArray,
    pub value: ByteArray,
}

impl AnyAsProtoMessage of ProtoMessage<Any> {
    fn encode_raw(self: @Any, ref context: EncodeContext) {
        context.encode_field(1, self.type_url);
        context.encode_field(2, self.value);
    }

    fn decode_raw(ref context: DecodeContext) -> Option<Any> {
        let type_url = context.decode_field(1)?;
        let value = context.decode_field(2)?;
        Option::Some(Any { type_url, value })
    }

    fn wire_type() -> WireType {
        WireType::LengthDelimited
    }
}


impl AnyAsProtoName of ProtoName<Any> {
    fn type_url() -> ByteArray {
        "type.googleapis.com/google.protobuf.Any"
    }
}

pub impl ProtoMessageIntoAny<T, +ProtoName<T>, +ProtoMessage<T>, +Drop<T>> of Into<T, Any> {
    fn into(self: T) -> Any {
        Any { type_url: ProtoName::<T>::type_url(), value: ProtoCodecImpl::encode(@self) }
    }
}

pub impl AnyTryIntoProtoMessage<
    T, +ProtoName<T>, +ProtoMessage<T>, +Drop<T>, +Default<T>,
> of TryInto<Any, T> {
    fn try_into(self: Any) -> Option<T> {
        if self.type_url == ProtoName::<T>::type_url() {
            ProtoCodecImpl::decode::<T>(@self.value)
        } else {
            Option::None
        }
    }
}
