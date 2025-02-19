use core::marker::PhantomData;

use cgp::prelude::*;
use hermes_encoding_components::traits::decode_mut::{MutDecoder, MutDecoderComponent};
use hermes_encoding_components::traits::encode_mut::{MutEncoder, MutEncoderComponent};
use hermes_encoding_components::traits::transform::{Transformer, TransformerRef};
use hermes_encoding_components::traits::types::decode_buffer::HasDecodeBufferType;
use hermes_encoding_components::traits::types::encode_buffer::HasEncodeBufferType;

use crate::impls::encode_mut::variant::SumEncoders;
use crate::traits::size::HasSize;
use crate::types::nat::{S, Z};

pub struct EncodeVariantFrom<Transform>(pub PhantomData<Transform>);

#[cgp_provider(MutEncoderComponent)]
impl<Encoding, Strategy, N, Transform> MutEncoder<Encoding, Strategy, Transform::From>
    for EncodeVariantFrom<Transform>
where
    Encoding: HasEncodeBufferType + HasAsyncErrorType,
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

#[cgp_provider(MutDecoderComponent)]
impl<Encoding, Strategy, N, Transform, Source, Target> MutDecoder<Encoding, Strategy, Target>
    for EncodeVariantFrom<Transform>
where
    Encoding: HasDecodeBufferType + HasAsyncErrorType,
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
