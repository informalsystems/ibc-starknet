use core::marker::PhantomData;

use hermes_encoding_components::traits::{
    CanDecodeMut, CanEncodeMut, CanPeekDecodeBuffer, HasEncodeBufferType, MutDecoder,
    MutDecoderComponent, MutEncoder, MutEncoderComponent,
};
use hermes_prelude::*;
use starknet::core::types::Felt;

use crate::impls::encode_mut::felt::UnexpectedEndOfBuffer;
use crate::impls::encode_mut::u128::felt_to_u128;
use crate::types::nat::{Nat, S};

#[derive(Debug)]
pub struct VariantIndexOutOfBound {
    pub index: usize,
}

pub struct SumEncoders<I>(pub PhantomData<I>);

#[cgp_provider(MutEncoderComponent)]
impl<Encoding, Strategy, ValueA, ValueB, I> MutEncoder<Encoding, Strategy, Either<ValueA, ValueB>>
    for SumEncoders<I>
where
    Encoding: CanEncodeMut<Strategy, ValueA> + CanEncodeMut<Strategy, usize>,
    I: Nat,
    SumEncoders<S<I>>: MutEncoder<Encoding, Strategy, ValueB>,
{
    fn encode_mut(
        encoding: &Encoding,
        value: &Either<ValueA, ValueB>,
        buffer: &mut <Encoding as HasEncodeBufferType>::EncodeBuffer,
    ) -> Result<(), Encoding::Error> {
        match value {
            Either::Left(value) => {
                encoding.encode_mut(&I::N, buffer)?;

                encoding.encode_mut(value, buffer)
            }
            Either::Right(value) => SumEncoders::encode_mut(encoding, value, buffer),
        }
    }
}

#[cgp_provider(MutEncoderComponent)]
impl<Encoding, Strategy, I> MutEncoder<Encoding, Strategy, Void> for SumEncoders<I>
where
    Encoding: HasEncodeBufferType + HasAsyncErrorType,
    I: Nat,
{
    fn encode_mut(
        encoding: &Encoding,
        value: &Void,
        buffer: &mut Encoding::EncodeBuffer,
    ) -> Result<(), Encoding::Error> {
        match *value {}
    }
}

#[cgp_provider(MutDecoderComponent)]
impl<Encoding, Strategy, ValueA, ValueB, I> MutDecoder<Encoding, Strategy, Either<ValueA, ValueB>>
    for SumEncoders<I>
where
    Encoding: CanDecodeMut<Strategy, ValueA>
        + CanDecodeMut<Strategy, usize>
        + CanPeekDecodeBuffer<Felt>
        + CanRaiseAsyncError<UnexpectedEndOfBuffer>,
    I: Nat,
    SumEncoders<S<I>>: MutDecoder<Encoding, Strategy, ValueB>,
{
    fn decode_mut(
        encoding: &Encoding,
        buffer: &mut Encoding::DecodeBuffer<'_>,
    ) -> Result<Either<ValueA, ValueB>, Encoding::Error> {
        let felt = Encoding::peek_decode_buffer(buffer)
            .ok_or_else(|| Encoding::raise_error(UnexpectedEndOfBuffer))?;

        let i = felt_to_u128(*felt);

        if i == I::N as u128 {
            let _: usize = encoding.decode_mut(buffer)?;
            let decoded = encoding.decode_mut(buffer)?;
            Ok(Either::Left(decoded))
        } else {
            let decoded = SumEncoders::decode_mut(encoding, buffer)?;
            Ok(Either::Right(decoded))
        }
    }
}

#[cgp_provider(MutDecoderComponent)]
impl<Encoding, Strategy, I> MutDecoder<Encoding, Strategy, Void> for SumEncoders<I>
where
    Encoding: CanDecodeMut<Strategy, usize> + CanRaiseAsyncError<VariantIndexOutOfBound>,
    I: Nat,
{
    fn decode_mut(
        encoding: &Encoding,
        buffer: &mut Encoding::DecodeBuffer<'_>,
    ) -> Result<Void, Encoding::Error> {
        let index: usize = encoding.decode_mut(buffer)?;

        Err(Encoding::raise_error(VariantIndexOutOfBound { index }))
    }
}
