use protobuf::primitives::array::{ByteArrayAsProtoMessage, BytesAsProtoMessage};
use protobuf::types::message::{
    DecodeContext, DecodeContextImpl, EncodeContext, EncodeContextImpl, ProtoCodecImpl,
    ProtoMessage,
};
use protobuf::types::tag::WireType;

#[derive(Default, Debug, Clone, Drop, PartialEq)]
pub struct TendermintByteArray {
    pub inner: Array<u8>,
}

pub impl TendermintByteArrayToArrayU8 of Into<TendermintByteArray, Array<u8>> {
    fn into(self: TendermintByteArray) -> Array<u8> {
        self.inner
    }
}

pub impl ArrayU8ToTendermintByteArray of Into<Array<u8>, TendermintByteArray> {
    fn into(self: Array<u8>) -> TendermintByteArray {
        TendermintByteArray { inner: self }
    }
}

pub impl TendermintByteArraySerde of Serde<TendermintByteArray> {
    fn serialize(self: @TendermintByteArray, ref output: Array<felt252>) {
        let mut byte_array = "";
        let mut span = self.inner.span();
        while let Some(byte) = span.pop_front() {
            byte_array.append_byte(*byte);
        }

        Serde::<ByteArray>::serialize(@byte_array, ref output);
    }

    fn deserialize(ref serialized: Span<felt252>) -> Option<TendermintByteArray> {
        let mut byte_array = Serde::<ByteArray>::deserialize(ref serialized)?;
        let mut data = array![];
        let len = byte_array.len();
        for i in 0..len {
            data.append(byte_array[i]);
        }
        Some(TendermintByteArray { inner: data })
    }
}

impl TendermintByteArrayAsProtoMessage of ProtoMessage<TendermintByteArray> {
    fn encode_raw(self: @TendermintByteArray, ref context: EncodeContext) {
        ProtoMessage::<Array<u8>>::encode_raw(self.inner, ref context);
    }

    fn decode_raw(ref context: DecodeContext) -> Option<TendermintByteArray> {
        let value = ProtoMessage::<Array<u8>>::decode_raw(ref context)?;
        Some(TendermintByteArray { inner: value })
    }

    fn wire_type() -> WireType {
        ProtoMessage::<Array<u8>>::wire_type()
    }
}
