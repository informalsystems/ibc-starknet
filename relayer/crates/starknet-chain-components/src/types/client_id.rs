use core::fmt::{Display, Formatter, Result as FmtResult};
use core::str::FromStr;

use cgp::core::component::UseContext;
use cgp::prelude::*;
use hermes_encoding_components::impls::encode_mut::combine::CombineEncoders;
use hermes_encoding_components::impls::encode_mut::field::EncodeField;
use hermes_encoding_components::impls::encode_mut::from::DecodeFrom;
use hermes_encoding_components::traits::transform::Transformer;
use hermes_wasm_encoding_components::components::{MutDecoderComponent, MutEncoderComponent};
use starknet::core::types::Felt;

#[derive(Debug, PartialEq, Clone, HasField)]
pub struct ClientId {
    pub client_type: Felt,
    pub sequence: u64,
}

pub struct EncodeClientId;

delegate_components! {
    EncodeClientId {
        MutEncoderComponent: CombineEncoders<Product![
            EncodeField<symbol!("client_type"), UseContext>,
            EncodeField<symbol!("sequence"), UseContext>,
        ]>,
        MutDecoderComponent: DecodeFrom<Self, UseContext>,
    }
}

impl Transformer for EncodeClientId {
    type From = (Felt, u64);
    type To = ClientId;

    fn transform((client_type, sequence): Self::From) -> ClientId {
        ClientId {
            client_type,
            sequence,
        }
    }
}

impl Display for ClientId {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let felt_as_be_bytes = self.client_type.to_bytes_be();
        let felt_as_string = String::from_utf8_lossy(&felt_as_be_bytes);
        let trimmed_client_type = felt_as_string.trim_start_matches('\0');
        write!(f, "{}-{}", trimmed_client_type, self.sequence)
    }
}

impl FromStr for ClientId {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // TODO: Verify that the parsing from str is correct
        let values: Vec<&str> = s.split('-').collect();
        let felt_str = values.first().ok_or_else(|| format!("client ID doesn't have correct format, expecting `<client_type>-<sequence>, got `{s}`"))?;
        let sequence_str = values.get(1).ok_or_else(|| format!("client ID doesn't have correct format, expecting `<client_type>-<sequence>, got `{s}`"))?;
        let client_type = Felt::from_str(felt_str)
            .map_err(|e| format!("failed to parse {felt_str} to Felt. Cause: {e}"))?;
        let sequence = sequence_str
            .parse::<u64>()
            .map_err(|e| format!("failed to parse {felt_str} to Felt. Cause: {e}"))?;
        Ok(Self {
            client_type,
            sequence,
        })
    }
}

#[cfg(test)]
mod test {
    use starknet::macros::short_string;

    use super::*;

    #[test]
    fn test_client_id_display() {
        let client_id = ClientId {
            client_type: short_string!("07-tendermint"),
            sequence: 1,
        };
        assert_eq!(client_id.to_string(), "07-tendermint-1");
    }
}
