use std::marker::PhantomData;

use cgp_core::error::HasErrorType;
use cgp_core::prelude::DelegateComponent;

use crate::traits::decode_mut::{HasDecodeBufferType, MutDecoder};
use crate::traits::encode_mut::{HasEncodeBufferType, MutEncoder};

pub struct DelegateEncodeMutComponents<Components>(pub PhantomData<Components>);

impl<Encoding, Strategy, Value, Components, Delegate> MutEncoder<Encoding, Strategy, Value>
    for DelegateEncodeMutComponents<Components>
where
    Encoding: HasEncodeBufferType + HasErrorType,
    Components: DelegateComponent<(Strategy, Value), Delegate = Delegate>,
    Delegate: MutEncoder<Encoding, Strategy, Value>,
{
    fn encode_mut(
        encoding: &Encoding,
        value: &Value,
        buffer: &mut Encoding::EncodeBuffer,
    ) -> Result<(), Encoding::Error> {
        Delegate::encode_mut(encoding, value, buffer)
    }
}

impl<Encoding, Strategy, Value, Components, Delegate> MutDecoder<Encoding, Strategy, Value>
    for DelegateEncodeMutComponents<Components>
where
    Encoding: HasDecodeBufferType + HasErrorType,
    Components: DelegateComponent<(Strategy, Value), Delegate = Delegate>,
    Delegate: MutDecoder<Encoding, Strategy, Value>,
{
    fn decode_mut(
        encoding: &Encoding,
        buffer: &mut Encoding::DecodeBuffer<'_>,
    ) -> Result<Value, Encoding::Error> {
        Delegate::decode_mut(encoding, buffer)
    }
}
