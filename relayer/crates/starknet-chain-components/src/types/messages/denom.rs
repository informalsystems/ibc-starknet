use cgp_core::prelude::*;
use hermes_cairo_encoding_components::impls::encode_mut::combine::CombineEncoders;
use hermes_cairo_encoding_components::impls::encode_mut::field::EncodeField;
use hermes_cairo_encoding_components::impls::encode_mut::variant::SumEncoders;
use hermes_cairo_encoding_components::traits::encode_mut::{HasEncodeBufferType, MutEncoder};
use hermes_cairo_encoding_components::types::either::Either;
use hermes_cairo_encoding_components::types::nat::{S, Z};
use hermes_cairo_encoding_components::{HList, Sum};
use starknet::core::types::Felt;

pub enum Denom {
    Native(Felt),
    Hosted(String),
}

#[derive(HasField)]
pub struct PrefixedDenom {
    pub trace_path: Vec<[String; 2]>,
    pub base: Denom,
}

pub type EncodePrefixedDenom = CombineEncoders<
    HList![
        EncodeField<symbol!("trace_path")>,
        EncodeField<symbol!("base")>,
    ],
>;

pub struct EncodeDenom;

impl<Encoding, Strategy> MutEncoder<Encoding, Strategy, Denom> for EncodeDenom
where
    Encoding: HasEncodeBufferType + HasErrorType,
    SumEncoders<Z, S<Z>>: for<'a> MutEncoder<Encoding, Strategy, Sum![Felt, &'a String]>,
{
    fn encode_mut(
        encoding: &Encoding,
        value: &Denom,
        buffer: &mut Encoding::EncodeBuffer,
    ) -> Result<(), Encoding::Error> {
        let sum = match value {
            Denom::Native(denom) => Either::Left(*denom),
            Denom::Hosted(denom) => Either::Right(Either::Left(denom)),
        };

        <SumEncoders<Z, S<Z>>>::encode_mut(encoding, &sum, buffer)?;

        Ok(())
    }
}
