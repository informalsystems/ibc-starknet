use cgp::prelude::*;
use hermes_cairo_encoding_components::impls::encode_mut::field::EncodeField;
use hermes_cairo_encoding_components::impls::encode_mut::variant_from::EncodeVariantFrom;
use hermes_cairo_encoding_components::traits::transform::TransformerRef;
use hermes_cairo_encoding_components::types::either::Either;
use hermes_cairo_encoding_components::{HList, Sum};
use hermes_encoding_components::impls::encode_mut::combine::CombineEncoders;
use hermes_encoding_components::impls::encode_mut::from::DecodeFrom;
use hermes_encoding_components::impls::with_context::EncodeWithContext;
use hermes_encoding_components::traits::decode_mut::MutDecoderComponent;
use hermes_encoding_components::traits::encode_mut::MutEncoderComponent;
use hermes_encoding_components::traits::transform::Transformer;
use starknet::core::types::Felt;

#[derive(Debug)]
pub enum Denom {
    Native(Felt),
    Hosted(String),
}

#[derive(Debug, HasField)]
pub struct PrefixedDenom {
    pub trace_path: Vec<TracePrefix>,
    pub base: Denom,
}

#[derive(Debug, HasField)]
pub struct TracePrefix {
    pub port_id: String,
    pub channel_id: String,
}

pub struct EncodePrefixedDenom;

delegate_components! {
    EncodePrefixedDenom {
        MutEncoderComponent: CombineEncoders<
            HList![
                EncodeField<symbol!("trace_path")>,
                EncodeField<symbol!("base")>,
            ],
        >,
        MutDecoderComponent: DecodeFrom<Self, EncodeWithContext>,
    }
}

impl Transformer for EncodePrefixedDenom {
    type From = (Vec<TracePrefix>, Denom);
    type To = PrefixedDenom;

    fn transform((trace_path, base): (Vec<TracePrefix>, Denom)) -> PrefixedDenom {
        PrefixedDenom { trace_path, base }
    }
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
        MutDecoderComponent: DecodeFrom<Self, EncodeWithContext>,
    }
}

impl Transformer for EncodeTracePrefix {
    type From = (String, String);
    type To = TracePrefix;

    fn transform((port_id, channel_id): (String, String)) -> TracePrefix {
        TracePrefix {
            port_id,
            channel_id,
        }
    }
}

pub struct EncodeDenom;

delegate_components! {
    EncodeDenom {
        [
            MutEncoderComponent,
            MutDecoderComponent,
        ]: EncodeVariantFrom<EncodeDenom>,
    }
}

impl TransformerRef for EncodeDenom {
    type From = Denom;
    type To<'a> = Sum![Felt, &'a String];

    fn transform<'a>(from: &'a Denom) -> Sum![Felt, &'a String] {
        match from {
            Denom::Native(denom) => Either::Left(*denom),
            Denom::Hosted(denom) => Either::Right(Either::Left(denom)),
        }
    }
}

impl Transformer for EncodeDenom {
    type From = Sum![Felt, String];
    type To = Denom;

    fn transform(value: Sum![Felt, String]) -> Denom {
        match value {
            Either::Left(value) => Denom::Native(value),
            Either::Right(Either::Left(value)) => Denom::Hosted(value),
            Either::Right(Either::Right(value)) => match value {},
        }
    }
}
