use core::marker::PhantomData;

use cgp_core::error::HasErrorType;

use crate::impls::encode_mut::variant::SumEncoders;
use crate::traits::decode_mut::{HasDecodeBufferType, MutDecoder};
use crate::traits::encode_mut::{HasEncodeBufferType, MutEncoder};
use crate::traits::transform::{Transformer, TransformerRef};
use crate::types::nat::Z;

pub struct EncodeVariantFrom<N, Transform>(pub PhantomData<(N, Transform)>);

impl<Encoding, Strategy, N, Transform> MutEncoder<Encoding, Strategy, Transform::From>
    for EncodeVariantFrom<N, Transform>
where
    Encoding: HasEncodeBufferType + HasErrorType,
    SumEncoders<Z, N>: for<'a> MutEncoder<Encoding, Strategy, Transform::To<'a>>,
    Transform: TransformerRef,
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
    for EncodeVariantFrom<N, Transform>
where
    Encoding: HasDecodeBufferType + HasErrorType,
    SumEncoders<Z, N>: MutDecoder<Encoding, Strategy, Source>,
    Transform: Transformer<From = Source, To = Target>,
{
    fn decode_mut(
        encoding: &Encoding,
        buffer: &mut Encoding::DecodeBuffer<'_>,
    ) -> Result<Target, Encoding::Error> {
        let source = SumEncoders::decode_mut(encoding, buffer)?;
        Ok(Transform::transform(source))
    }
}
