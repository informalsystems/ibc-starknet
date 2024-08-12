use cgp_core::error::HasErrorType;

use crate::impls::encode_mut::variant::{Either, SumEncoders, Void, Z};
use crate::impls::encode_mut::with_context::EncodeWithContext;
use crate::traits::encode_mut::{HasEncodeBufferType, MutEncoder};
use crate::HList;

pub struct EncodeOption;

pub type OptionEncoder = SumEncoders<Z, HList![EncodeWithContext, EncodeWithContext]>;

impl<Encoding, Strategy, Value> MutEncoder<Encoding, Strategy, Option<Value>> for EncodeOption
where
    Encoding: HasEncodeBufferType + HasErrorType,
    OptionEncoder: for<'a> MutEncoder<Encoding, Strategy, Either<&'a Value, Either<(), Void>>>,
{
    fn encode_mut(
        encoding: &Encoding,
        value: &Option<Value>,
        buffer: &mut Encoding::EncodeBuffer,
    ) -> Result<(), Encoding::Error> {
        let sum = match value {
            Some(value) => Either::Left(value),
            None => Either::Right(Either::Left(())),
        };

        OptionEncoder::encode_mut(encoding, &sum, buffer)?;

        Ok(())
    }
}
