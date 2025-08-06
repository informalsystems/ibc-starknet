use core::str::FromStr;

use hermes_core::chain_components::traits::{
    EvidenceFieldsGetter, EvidenceFieldsGetterComponent, EvidenceTypeProvider,
    EvidenceTypeProviderComponent, HasClientIdType, HasEvidenceType,
};
use hermes_prelude::*;
use ibc::core::host::types::{error::DecodingError, identifiers::ClientId};
use ibc_proto::{ibc::lightclients::tendermint::v1::Header, Protobuf};
use ibc_client_tendermint::types::proto::v1::Misbehaviour;

#[derive(Clone, Debug)]
pub struct StarknetMisbehaviour {
    pub client_id: ClientId,
    pub evidence_1: Header,
    pub evidence_2: Header,
}

#[cgp_new_provider(EvidenceTypeProviderComponent)]
impl<Chain> EvidenceTypeProvider<Chain> for ProvideStarknetEvidenceType
where
    Chain: Async,
{
    type Evidence = StarknetMisbehaviour;
}

#[cgp_provider(EvidenceFieldsGetterComponent)]
impl<Chain, Counterparty> EvidenceFieldsGetter<Chain, Counterparty> for ProvideStarknetEvidenceType
where
    Chain: HasEvidenceType<Evidence = StarknetMisbehaviour>
        + HasClientIdType<Counterparty, ClientId = ClientId>,
{
    fn evidence_client_id(evidence: &StarknetMisbehaviour) -> ClientId {
        evidence.client_id.clone()
    }
}

impl Protobuf<Misbehaviour> for StarknetMisbehaviour {}

impl From<StarknetMisbehaviour> for Misbehaviour {
    fn from(value: StarknetMisbehaviour) -> Self {
        #[allow(deprecated)]
        Self {
            client_id: value.client_id.to_string(),
            header_1: Some(value.evidence_1),
            header_2: Some(value.evidence_2),
        }
    }
}

impl TryFrom<Misbehaviour> for StarknetMisbehaviour {
    type Error = DecodingError;

    fn try_from(raw: Misbehaviour) -> Result<Self, Self::Error> {
        #[allow(deprecated)]
        let evidence = Self {
            client_id: ClientId::from_str(&raw.client_id).unwrap(),
            evidence_1: raw.header_1.unwrap(),
            evidence_2: raw.header_2.unwrap(),
        };

        Ok(evidence)
    }
}
