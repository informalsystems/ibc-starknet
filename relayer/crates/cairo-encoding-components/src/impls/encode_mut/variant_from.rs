use core::marker::PhantomData;

use cgp::core::error::HasErrorType;
use hermes_encoding_components::traits::decode_mut::MutDecoder;
use hermes_encoding_components::traits::encode_mut::MutEncoder;
use hermes_encoding_components::traits::types::decode_buffer::HasDecodeBufferType;
use hermes_encoding_components::traits::types::encode_buffer::HasEncodeBufferType;

use crate::impls::encode_mut::variant::SumEncoders;
use crate::traits::size::HasSize;
use crate::traits::transform::{Transformer, TransformerRef};
use crate::types::nat::{S, Z};

pub struct EncodeVariantFrom<Transform>(pub PhantomData<Transform>);

impl<Encoding, Strategy, N, Transform> MutEncoder<Encoding, Strategy, Transform::From>
    for EncodeVariantFrom<Transform>
where
    Encoding: HasEncodeBufferType + HasErrorType,
    SumEncoders<Z, N>: for<'a> MutEncoder<Encoding, Strategy, Transform::To<'a>>,
    Transform: TransformerRef,
    for<'a> Transform::To<'a>: HasSize<Size = S<N>>,
{
    fn encode_mut(
        encoding: &Encoding,
        source: &Transform::From,
        buffer: &mut Encoding::EncodeBuffer,
    ) -> Result<(), Encoding::Error> {
        let target = Transform::transform(source);
        SumEncoders::encode_mut(encoding, &target, buffer)
    }
}

impl<Encoding, Strategy, N, Transform, Source, Target> MutDecoder<Encoding, Strategy, Target>
    for EncodeVariantFrom<Transform>
where
    Encoding: HasDecodeBufferType + HasErrorType,
    SumEncoders<Z, N>: MutDecoder<Encoding, Strategy, Source>,
    Transform: Transformer<From = Source, To = Target>,
    Source: HasSize<Size = S<N>>,
{
    fn decode_mut(
        encoding: &Encoding,
        buffer: &mut Encoding::DecodeBuffer<'_>,
    ) -> Result<Target, Encoding::Error> {
        let source = SumEncoders::decode_mut(encoding, buffer)?;
        Ok(Transform::transform(source))
    }
}
