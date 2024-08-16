use cgp_core::prelude::*;
use hermes_cairo_encoding_components::impls::encode_mut::combine::CombineEncoders;
use hermes_cairo_encoding_components::impls::encode_mut::field::EncodeField;
use hermes_cairo_encoding_components::impls::encode_mut::from::DecodeFrom;
use hermes_cairo_encoding_components::impls::encode_mut::variant::EncodeVariants;
use hermes_cairo_encoding_components::traits::decode_mut::MutDecoderComponent;
use hermes_cairo_encoding_components::traits::encode_mut::{
    HasEncodeBufferType, MutEncoder, MutEncoderComponent,
};
use hermes_cairo_encoding_components::traits::transform::Transformer;
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
    pub trace_path: Vec<TracePrefix>,
    pub base: Denom,
}

pub type EncodePrefixedDenom = CombineEncoders<
    HList![
        EncodeField<symbol!("trace_path")>,
        EncodeField<symbol!("base")>,
    ],
>;

#[derive(HasField)]
pub struct TracePrefix {
    pub port_id: String,
    pub channel_id: String,
}

pub struct EncodeTracePrefix;

delegate_components! {
    EncodeTracePrefix {
        MutEncoderComponent: CombineEncoders<
            HList![
                EncodeField<symbol!("port_id")>,
                EncodeField<symbol!("channel_id")>,
            ],
        >,
        MutDecoderComponent: DecodeFrom<EncodeTracePrefix, (String, String)>,
    }
}

impl Transformer<(String, String), TracePrefix> for EncodeTracePrefix {
    fn transform((port_id, channel_id): (String, String)) -> TracePrefix {
        TracePrefix {
            port_id,
            channel_id,
        }
    }
}

pub struct EncodeDenom;

impl<Encoding, Strategy> MutEncoder<Encoding, Strategy, Denom> for EncodeDenom
where
    Encoding: HasEncodeBufferType + HasErrorType,
    EncodeVariants<S<Z>>: for<'a> MutEncoder<Encoding, Strategy, Sum![Felt, &'a String]>,
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

        <EncodeVariants<S<Z>>>::encode_mut(encoding, &sum, buffer)?;

        Ok(())
    }
}
