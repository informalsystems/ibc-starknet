use core::marker::PhantomData;

use cgp::core::error::{CanRaiseError, HasErrorType};
use hermes_encoding_components::traits::decode_mut::{
    CanDecodeMut, CanPeekDecodeBuffer, MutDecoder,
};
use hermes_encoding_components::traits::encode_mut::{CanEncodeMut, MutEncoder};
use hermes_encoding_components::traits::types::encode_buffer::HasEncodeBufferType;
use starknet::core::types::Felt;

use crate::impls::encode_mut::felt::UnexpectedEndOfBuffer;
use crate::impls::encode_mut::u128::felt_to_u128;
use crate::types::either::{Either, Void};
use crate::types::nat::{Nat, S, Z};

pub type EncodeVariants<N> = SumEncoders<Z, N>;

#[derive(Debug)]
pub struct VariantIndexOutOfBound {
    pub index: usize,
}

pub struct SumEncoders<Index, Remain>(pub PhantomData<(Index, Remain)>);

impl<Encoding, Strategy, ValueA, ValueB, I, N>
    MutEncoder<Encoding, Strategy, Either<ValueA, ValueB>> for SumEncoders<I, S<N>>
where
    Encoding: CanEncodeMut<Strategy, ValueA> + CanEncodeMut<Strategy, usize>,
    I: Nat,
    SumEncoders<S<I>, N>: MutEncoder<Encoding, Strategy, ValueB>,
{
    fn encode_mut(
        encoding: &Encoding,
        value: &Either<ValueA, ValueB>,
        buffer: &mut <Encoding as HasEncodeBufferType>::EncodeBuffer,
    ) -> Result<(), <Encoding as HasErrorType>::Error> {
        match value {
            Either::Left(value) => {
                encoding.encode_mut(&I::N, buffer)?;

                encoding.encode_mut(value, buffer)
            }
            Either::Right(value) => SumEncoders::encode_mut(encoding, value, buffer),
        }
    }
}

impl<Encoding, Strategy, Value, I> MutEncoder<Encoding, Strategy, Either<Value, Void>>
    for SumEncoders<I, Z>
where
    Encoding: CanEncodeMut<Strategy, Value> + CanEncodeMut<Strategy, usize>,
    I: Nat,
{
    fn encode_mut(
        encoding: &Encoding,
        value: &Either<Value, Void>,
        buffer: &mut Encoding::EncodeBuffer,
    ) -> Result<(), Encoding::Error> {
        match value {
            Either::Left(value) => {
                encoding.encode_mut(&I::N, buffer)?;
                encoding.encode_mut(value, buffer)
            }
            Either::Right(value) => match *value {},
        }
    }
}

impl<Encoding, Strategy, ValueA, ValueB, I, N>
    MutDecoder<Encoding, Strategy, Either<ValueA, ValueB>> for SumEncoders<I, S<N>>
where
    Encoding: CanDecodeMut<Strategy, ValueA>
        + CanDecodeMut<Strategy, usize>
        + CanPeekDecodeBuffer<Felt>
        + CanRaiseError<UnexpectedEndOfBuffer>,
    I: Nat,
    SumEncoders<S<I>, N>: MutDecoder<Encoding, Strategy, ValueB>,
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

impl<Encoding, Strategy, Value, I> MutDecoder<Encoding, Strategy, Either<Value, Void>>
    for SumEncoders<I, Z>
where
    Encoding: CanDecodeMut<Strategy, Value>
        + CanDecodeMut<Strategy, usize>
        + CanPeekDecodeBuffer<Felt>
        + CanRaiseError<VariantIndexOutOfBound>,
    I: Nat,
{
    fn decode_mut(
        encoding: &Encoding,
        buffer: &mut Encoding::DecodeBuffer<'_>,
    ) -> Result<Either<Value, Void>, Encoding::Error> {
        let index: usize = encoding.decode_mut(buffer)?;

        if index != I::N {
            Err(Encoding::raise_error(VariantIndexOutOfBound { index }))
        } else {
            let decoded = encoding.decode_mut(buffer)?;

            Ok(Either::Left(decoded))
        }
    }
}
