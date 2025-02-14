use protobuf::types::message::{
    ProtoMessage, ProtoName, ProtoCodecImpl, EncodeContext, DecodeContext, EncodeContextImpl,
    DecodeContextImpl,
};
use protobuf::types::tag::WireType;
use protobuf::primitives::numeric::{I32AsProtoMessage, I64AsProtoMessage};
use protobuf::primitives::array::ByteArrayAsProtoMessage;

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
        let mut duration = Default::<Duration>::default();
        if !context.decode_field(1, ref duration.seconds) {
            return Option::None;
        }
        if !context.decode_field(2, ref duration.nanos) {
            return Option::None;
        }
        Option::Some(duration)
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

impl TimestampAsProtoMessage of ProtoMessage<Timestamp> {
    fn encode_raw(self: @Timestamp, ref context: EncodeContext) {
        context.encode_field(1, self.seconds);
        context.encode_field(2, self.nanos);
    }

    fn decode_raw(ref context: DecodeContext) -> Option<Timestamp> {
        let mut timestamp = Default::<Timestamp>::default();
        if !context.decode_field(1, ref timestamp.seconds) {
            return Option::None;
        }
        if !context.decode_field(2, ref timestamp.nanos) {
            return Option::None;
        }
        Option::Some(timestamp)
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
        let mut any = Default::<Any>::default();
        if !context.decode_field(1, ref any.type_url) {
            return Option::None;
        }
        if !context.decode_field(2, ref any.value) {
            return Option::None;
        }
        Option::Some(any)
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
