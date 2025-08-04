use hermes_core::chain_components::traits::{
    EvidenceFieldsGetter, EvidenceFieldsGetterComponent, EvidenceTypeProvider,
    EvidenceTypeProviderComponent, HasClientIdType, HasEvidenceType,
};
use hermes_prelude::*;
use ibc::core::host::types::identifiers::ClientId;

#[derive(Clone, Debug)]
pub struct StarknetMisbehaviour {
    pub client_id: ClientId,
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
