use core::str::FromStr;

use hermes_core::chain_components::traits::{EvidenceTypeProvider, EvidenceTypeProviderComponent};
use hermes_prelude::*;
use ibc::core::host::types::error::DecodingError;
use ibc::core::host::types::identifiers::ClientId;
use ibc_client_tendermint::types::proto::v1::Misbehaviour;
use ibc_proto::ibc::lightclients::tendermint::v1::Header;
use ibc_proto::Protobuf;
use prost_types::Any;

#[derive(Clone, Debug)]
pub struct CosmosStarknetMisbehaviour {
    pub client_id: ClientId,
    pub evidence_1: Header,
    pub evidence_2: Header,
}

#[cgp_new_provider(EvidenceTypeProviderComponent)]
impl<Chain> EvidenceTypeProvider<Chain> for ProvideStarknetEvidenceType
where
    Chain: Async,
{
    type Evidence = Any;
}

impl Protobuf<Misbehaviour> for CosmosStarknetMisbehaviour {}

impl From<CosmosStarknetMisbehaviour> for Misbehaviour {
    fn from(value: CosmosStarknetMisbehaviour) -> Self {
        #[allow(deprecated)]
        Self {
            client_id: value.client_id.to_string(),
            header_1: Some(value.evidence_1),
            header_2: Some(value.evidence_2),
        }
    }
}

impl TryFrom<Misbehaviour> for CosmosStarknetMisbehaviour {
    type Error = DecodingError;

    fn try_from(raw: Misbehaviour) -> Result<Self, Self::Error> {
        #[allow(deprecated)]
        let evidence = Self {
            client_id: ClientId::from_str(&raw.client_id).map_err(|e| {
                DecodingError::InvalidRawData {
                    description: format!("invalid client id `{}`: {e}", raw.client_id),
                }
            })?,
            evidence_1: raw.header_1.ok_or_else(|| DecodingError::MissingRawData {
                description: "missing Header 1 from `Misbehaviour`".into(),
            })?,
            evidence_2: raw.header_2.ok_or_else(|| DecodingError::MissingRawData {
                description: "missing Header 1 from `Misbehaviour`".into(),
            })?,
        };

        Ok(evidence)
    }
}
