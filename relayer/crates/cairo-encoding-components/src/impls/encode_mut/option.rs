use cgp_core::error::HasErrorType;

use crate::impls::encode_mut::variant::SumEncoders;
use crate::impls::encode_mut::with_context::EncodeWithContext;
use crate::traits::encode_mut::{HasEncodeBufferType, MutEncoder};
use crate::types::either::Either;
use crate::types::nat::Z;
use crate::{HList, Sum};

pub struct EncodeOption;

pub type OptionEncoder = SumEncoders<Z, HList![EncodeWithContext, EncodeWithContext]>;

impl<Encoding, Strategy, Value> MutEncoder<Encoding, Strategy, Option<Value>> for EncodeOption
where
    Encoding: HasEncodeBufferType + HasErrorType,
    OptionEncoder: for<'a> MutEncoder<Encoding, Strategy, Sum![&'a Value, ()]>,
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
