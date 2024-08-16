use core::marker::PhantomData;

use cgp_core::error::HasErrorType;

use crate::impls::encode_mut::variant::SumEncoders;
use crate::traits::decode_mut::{HasDecodeBufferType, MutDecoder};
use crate::traits::encode_mut::{HasEncodeBufferType, MutEncoder};
use crate::traits::transform::Transformer;
use crate::types::nat::Z;

pub struct EncodeVariantFrom<N, Transform, Source>(pub PhantomData<(N, Transform, Source)>);

impl<Encoding, Strategy, N, Transform, Source> MutEncoder<Encoding, Strategy, Source>
    for EncodeVariantFrom<N, Transform, Source>
where
    Encoding: HasEncodeBufferType + HasErrorType,
    SumEncoders<Z, N>:
        for<'a> MutEncoder<Encoding, Strategy, <Transform as Transformer<&'a Source>>::To>,
    Transform: for<'a> Transformer<&'a Source>,
{
    fn encode_mut(
        encoding: &Encoding,
        source: &Source,
        buffer: &mut Encoding::EncodeBuffer,
    ) -> Result<(), Encoding::Error> {
        let target = Transform::transform(source);
        SumEncoders::encode_mut(encoding, &target, buffer)
    }
}

impl<Encoding, Strategy, N, Transform, Source, Target> MutDecoder<Encoding, Strategy, Target>
    for EncodeVariantFrom<N, Transform, Source>
where
    Encoding: HasDecodeBufferType + HasErrorType,
    SumEncoders<Z, N>: MutDecoder<Encoding, Strategy, Source>,
    Transform: Transformer<Source, To = Target>,
{
    fn decode_mut(
        encoding: &Encoding,
        buffer: &mut Encoding::DecodeBuffer<'_>,
    ) -> Result<Target, Encoding::Error> {
        let source = SumEncoders::decode_mut(encoding, buffer)?;
        Ok(Transform::transform(source))
    }
}
