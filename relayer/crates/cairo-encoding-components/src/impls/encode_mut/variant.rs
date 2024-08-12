use std::marker::PhantomData;

use cgp_core::error::HasErrorType;

use crate::traits::encode_mut::{CanEncodeMut, HasEncodeBufferType, MutEncoder};
use crate::types::either::{Either, Void};
use crate::types::nat::{Nat, S, Z};

pub struct SumEncoders<Index, Remain>(pub PhantomData<(Index, Remain)>);

impl<Encoding, Strategy, ValueA, ValueB, I, N>
    MutEncoder<Encoding, Strategy, Either<ValueA, ValueB>> for SumEncoders<I, S<N>>
where
    Encoding: CanEncodeMut<Strategy, ValueA>,
    I: Nat,
    SumEncoders<S<I>, N>: MutEncoder<Encoding, Strategy, ValueB>,
{
    fn encode_mut(
        encoding: &Encoding,
        value: &Either<ValueA, ValueB>,
        buffer: &mut <Encoding as HasEncodeBufferType>::EncodeBuffer,
    ) -> Result<(), <Encoding as HasErrorType>::Error> {
        match value {
            Either::Left(value) => encoding.encode_mut(value, buffer),
            Either::Right(value) => <SumEncoders<S<I>, N>>::encode_mut(encoding, value, buffer),
        }
    }
}

impl<Encoding, Strategy, Value, I> MutEncoder<Encoding, Strategy, Either<Value, Void>>
    for SumEncoders<I, Z>
where
    Encoding: CanEncodeMut<Strategy, Value>,
{
    fn encode_mut(
        encoding: &Encoding,
        value: &Either<Value, Void>,
        buffer: &mut Encoding::EncodeBuffer,
    ) -> Result<(), Encoding::Error> {
        match value {
            Either::Left(value) => encoding.encode_mut(value, buffer),
            Either::Right(value) => match *value {},
        }
    }
}
