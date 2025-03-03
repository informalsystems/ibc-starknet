use std::fmt::Display;

use cgp::core::component::UseContext;
use cgp::prelude::*;
use hermes_cairo_encoding_components::impls::encode_mut::variant_from::EncodeVariantFrom;
use hermes_encoding_components::impls::encode_mut::combine::CombineEncoders;
use hermes_encoding_components::impls::encode_mut::field::EncodeField;
use hermes_encoding_components::impls::encode_mut::from::DecodeFrom;
use hermes_encoding_components::traits::decode_mut::MutDecoderComponent;
use hermes_encoding_components::traits::encode_mut::MutEncoderComponent;
use hermes_encoding_components::traits::transform::{Transformer, TransformerRef};

use crate::impls::types::address::StarknetAddress;

#[derive(Clone, Debug)]
pub enum Denom {
    Native(StarknetAddress),
    Hosted(String),
}

impl Display for Denom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Native(denom) => write!(f, "{denom}"),
            Self::Hosted(denom) => write!(f, "{denom}"),
        }
    }
}

#[derive(Clone, Debug, HasField)]
pub struct PrefixedDenom {
    pub trace_path: Vec<TracePrefix>,
    pub base: Denom,
}

impl Display for PrefixedDenom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for prefix in self.trace_path.iter().rev() {
            write!(f, "{}/{}/", prefix.port_id, prefix.channel_id)?;
        }

        write!(f, "{}", self.base)?;

        Ok(())
    }
}

#[derive(Clone, Debug, HasField)]
pub struct TracePrefix {
    pub port_id: String,
    pub channel_id: String,
}

pub struct EncodePrefixedDenom;

delegate_components! {
    EncodePrefixedDenom {
        MutEncoderComponent: CombineEncoders<
            Product![
                EncodeField<symbol!("trace_path"), UseContext>,
                EncodeField<symbol!("base"), UseContext>,
            ],
        >,
        MutDecoderComponent: DecodeFrom<Self, UseContext>,
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
            Product![
                EncodeField<symbol!("port_id"), UseContext>,
                EncodeField<symbol!("channel_id"), UseContext>,
            ],
        >,
        MutDecoderComponent: DecodeFrom<Self, UseContext>,
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
    type To<'a> = Sum![StarknetAddress, &'a String];

    fn transform<'a>(from: &'a Denom) -> Sum![StarknetAddress, &'a String] {
        match from {
            Denom::Native(denom) => Either::Left(*denom),
            Denom::Hosted(denom) => Either::Right(Either::Left(denom)),
        }
    }
}

impl Transformer for EncodeDenom {
    type From = Sum![StarknetAddress, String];
    type To = Denom;

    fn transform(value: Sum![StarknetAddress, String]) -> Denom {
        match value {
            Either::Left(value) => Denom::Native(value),
            Either::Right(Either::Left(value)) => Denom::Hosted(value),
            Either::Right(Either::Right(value)) => match value {},
        }
    }
}
