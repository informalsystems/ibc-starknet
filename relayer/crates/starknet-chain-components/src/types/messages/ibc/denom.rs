use std::fmt::Display;
use std::str::FromStr;

use cgp::core::component::UseContext;
use cgp::prelude::*;
use hermes_cairo_encoding_components::impls::encode_mut::variant_from::EncodeVariantFrom;
use hermes_encoding_components::impls::encode_mut::combine::CombineEncoders;
use hermes_encoding_components::impls::encode_mut::field::EncodeField;
use hermes_encoding_components::impls::encode_mut::from::DecodeFrom;
use hermes_encoding_components::traits::decode_mut::MutDecoderComponent;
use hermes_encoding_components::traits::encode_mut::MutEncoderComponent;
use hermes_encoding_components::traits::transform::{Transformer, TransformerRef};
use ibc::apps::transfer::types::PrefixedDenom as IbcPrefixedDenom;

use crate::impls::types::address::StarknetAddress;

#[derive(Clone, Debug, PartialEq)]
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

#[derive(Clone, Debug, HasField, PartialEq)]
pub struct PrefixedDenom {
    pub trace_path: Vec<TracePrefix>,
    pub base: Denom,
}

impl Display for PrefixedDenom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for prefix in self.iter_trace_path() {
            write!(f, "{}/{}/", prefix.port_id, prefix.channel_id)?;
        }

        write!(f, "{}", self.base)?;

        Ok(())
    }
}

impl PrefixedDenom {
    /// TracePath is stored in reverse order for appending convenience.
    ///
    /// This function returns the trace path in the correct order.
    ///
    /// PrefixedDenom: transfer/channel-1/transfer/channel-2/coin
    /// .trace_path repr: [(transfer, channel-2), (transfer, channel-1)]
    /// .iter_trace_path(): (transfer, channel-1), (transfer, channel-2)
    pub fn iter_trace_path(&self) -> impl Iterator<Item = &TracePrefix> {
        self.trace_path.iter().rev()
    }
}

impl FromStr for PrefixedDenom {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ibc_prefix_denom = IbcPrefixedDenom::from_str(s)
            .map_err(|e| format!("failed to convert string to IbcPrefixDenom: {e}"))?;
        let trace_path_json = serde_json::to_string(&ibc_prefix_denom.trace_path)
            .map_err(|e| format!("failed to convert IbcPrefixDenom TracePath to string: {e}"))?;

        #[derive(serde::Deserialize)]
        struct DummyTracePath {
            pub port_id: String,
            pub channel_id: String,
        }

        let trace_path: Vec<DummyTracePath> = serde_json::from_str(&trace_path_json)
            .map_err(|e| format!("failed to convert string to DummyTracePath: {e}"))?;

        Ok(Self {
            trace_path: trace_path
                .into_iter()
                .map(
                    |DummyTracePath {
                         port_id,
                         channel_id,
                     }| TracePrefix {
                        port_id,
                        channel_id,
                    },
                )
                .collect(),
            base: Denom::Hosted(ibc_prefix_denom.base_denom.as_str().to_string()),
        })
    }
}

#[derive(Clone, Debug, HasField, PartialEq)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_trace_path() {
        let input = "transfer/channel-0/coin";
        let expected = PrefixedDenom {
            trace_path: vec![TracePrefix {
                port_id: "transfer".to_string(),
                channel_id: "channel-0".to_string(),
            }],
            base: Denom::Hosted("coin".to_string()),
        };

        let result = PrefixedDenom::from_str(input);
        assert!(result.is_ok(), "Parsing failed for single trace path");
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn test_multiple_trace_paths() {
        let input = "transfer2/channel-1/transfer/channel-0/coin";
        let expected = PrefixedDenom {
            trace_path: vec![
                TracePrefix {
                    port_id: "transfer2".to_string(),
                    channel_id: "channel-1".to_string(),
                },
                TracePrefix {
                    port_id: "transfer".to_string(),
                    channel_id: "channel-0".to_string(),
                },
            ],
            base: Denom::Hosted("coin".to_string()),
        };

        let result = PrefixedDenom::from_str(input);
        assert!(result.is_ok(), "Parsing failed for multiple trace paths");
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn test_starknet_trace_paths() {
        let input = "transfer/channel-75/factory/stars16da2uus9zrsy83h23ur42v3lglg5rmyrpqnju4/dust";
        let expected = PrefixedDenom {
            trace_path: vec![TracePrefix {
                port_id: "transfer".to_string(),
                channel_id: "channel-75".to_string(),
            }],
            base: Denom::Hosted(
                "factory/stars16da2uus9zrsy83h23ur42v3lglg5rmyrpqnju4/dust".to_string(),
            ),
        };

        let result = PrefixedDenom::from_str(input);
        assert!(result.is_ok(), "Parsing failed for multiple trace paths");
        assert_eq!(result.unwrap(), expected);
    }
}
