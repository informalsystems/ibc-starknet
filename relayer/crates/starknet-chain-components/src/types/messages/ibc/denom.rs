use std::fmt::Display;
use std::str::FromStr;

use cgp::prelude::*;
use ibc::apps::transfer::types::PrefixedDenom as IbcPrefixedDenom;

use crate::impls::types::address::StarknetAddress;

#[derive(Clone, Debug, PartialEq, HasFields)]
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

#[derive(Clone, Debug, HasField, HasFields, PartialEq)]
pub struct PrefixedDenom {
    pub trace_path: Vec<TracePrefix>,
    pub base: Denom,
}

impl Display for PrefixedDenom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for prefix in &self.trace_path {
            write!(f, "{}/{}/", prefix.port_id, prefix.channel_id)?;
        }

        write!(f, "{}", self.base)?;

        Ok(())
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
                // ibc-rs and cairo has different order of trace path
                .rev()
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

#[derive(Clone, Debug, HasField, HasFields, PartialEq)]
pub struct TracePrefix {
    pub port_id: String,
    pub channel_id: String,
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
        assert_eq!(expected.to_string(), input);
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
        assert_eq!(expected.to_string(), input);
    }
}
