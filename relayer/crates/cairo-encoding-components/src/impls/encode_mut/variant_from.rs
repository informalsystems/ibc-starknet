use core::marker::PhantomData;

use cgp_core::error::HasErrorType;

use crate::impls::encode_mut::variant::SumEncoders;
use crate::traits::decode_mut::{HasDecodeBufferType, MutDecoder};
use crate::traits::encode_mut::{HasEncodeBufferType, MutEncoder};
use crate::traits::size::HasSize;
use crate::traits::transform::{Transformer, TransformerRef};
use crate::types::nat::Z;

pub struct EncodeVariantFrom<Transform>(pub PhantomData<Transform>);

impl<Encoding, Strategy, N, Transform> MutEncoder<Encoding, Strategy, Transform::From>
    for EncodeVariantFrom<Transform>
where
    Encoding: HasEncodeBufferType + HasErrorType,
    SumEncoders<Z, N>: for<'a> MutEncoder<Encoding, Strategy, Transform::To<'a>>,
    Transform: TransformerRef,
    for<'a> Transform::To<'a>: HasSize<Size = N>,
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
    Source: HasSize<Size = N>,
{
    fn decode_mut(
        encoding: &Encoding,
        buffer: &mut Encoding::DecodeBuffer<'_>,
    ) -> Result<Target, Encoding::Error> {
        let source = SumEncoders::decode_mut(encoding, buffer)?;
        Ok(Transform::transform(source))
    }
}
