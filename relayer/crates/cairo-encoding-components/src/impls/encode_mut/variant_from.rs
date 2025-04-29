use core::marker::PhantomData;

use hermes_encoding_components::traits::{
    HasDecodeBufferType, HasEncodeBufferType, MutDecoder, MutDecoderComponent, MutEncoder,
    MutEncoderComponent, Transformer, TransformerRef,
};
use hermes_prelude::*;

use crate::impls::encode_mut::variant::SumEncoders;
use crate::types::nat::Z;

pub struct EncodeVariantFrom<Transform>(pub PhantomData<Transform>);

#[cgp_provider(MutEncoderComponent)]
impl<Encoding, Strategy, Transform> MutEncoder<Encoding, Strategy, Transform::From>
    for EncodeVariantFrom<Transform>
where
    Encoding: HasEncodeBufferType + HasAsyncErrorType,
    SumEncoders<Z>: for<'a> MutEncoder<Encoding, Strategy, Transform::To<'a>>,
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

#[cgp_provider(MutDecoderComponent)]
impl<Encoding, Strategy, Transform, Source, Target> MutDecoder<Encoding, Strategy, Target>
    for EncodeVariantFrom<Transform>
where
    Encoding: HasDecodeBufferType + HasAsyncErrorType,
    SumEncoders<Z>: MutDecoder<Encoding, Strategy, Source>,
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
