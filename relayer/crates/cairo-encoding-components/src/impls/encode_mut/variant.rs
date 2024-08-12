use std::marker::PhantomData;

use cgp_core::error::HasErrorType;

use crate::traits::encode_mut::{HasEncodeBufferType, MutEncoder};
use crate::types::either::{Either, Void};
use crate::types::nat::{Nat, S};

pub struct SumEncoders<N, Encoders>(pub PhantomData<(N, Encoders)>);

impl<Encoding, Strategy, ValueA, ValueB, N, Encoder, InEncoders>
    MutEncoder<Encoding, Strategy, Either<ValueA, ValueB>> for SumEncoders<N, (Encoder, InEncoders)>
where
    Encoding: HasEncodeBufferType + HasErrorType,
    Encoder: MutEncoder<Encoding, Strategy, ValueA>,
    N: Nat,
    SumEncoders<S<N>, InEncoders>: MutEncoder<Encoding, Strategy, ValueB>,
{
    fn encode_mut(
        encoding: &Encoding,
        value: &Either<ValueA, ValueB>,
        buffer: &mut <Encoding as HasEncodeBufferType>::EncodeBuffer,
    ) -> Result<(), <Encoding as HasErrorType>::Error> {
        match value {
            Either::Left(value) => Encoder::encode_mut(encoding, value, buffer),
            Either::Right(value) => {
                <SumEncoders<S<N>, InEncoders>>::encode_mut(encoding, value, buffer)
            }
        }
    }
}

impl<Encoding, Strategy, N> MutEncoder<Encoding, Strategy, Void> for SumEncoders<N, ()>
where
    Encoding: HasEncodeBufferType + HasErrorType,
{
    fn encode_mut(
        _encoding: &Encoding,
        _value: &Void,
        _buffer: &mut Encoding::EncodeBuffer,
    ) -> Result<(), Encoding::Error> {
        Ok(())
    }
}
