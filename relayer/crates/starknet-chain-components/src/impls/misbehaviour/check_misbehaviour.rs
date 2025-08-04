use hermes_core::chain_components::traits::{
    HasClientStateType, HasEvidenceType, HasUpdateClientEvent, MisbehaviourChecker,
    MisbehaviourCheckerComponent,
};
use hermes_prelude::*;

#[cgp_new_provider(MisbehaviourCheckerComponent)]
impl<Chain, Counterparty> MisbehaviourChecker<Chain, Counterparty> for CheckStarknetMisbehaviour
where
    Chain: HasClientStateType<Counterparty> + HasAsyncErrorType,
    Counterparty: HasUpdateClientEvent + HasEvidenceType + Async,
{
    async fn check_misbehaviour(
        chain: &Chain,
        update_client_event: &Counterparty::UpdateClientEvent,
        client_state: &Chain::ClientState,
    ) -> Result<Option<Counterparty::Evidence>, Chain::Error> {
        Ok(None)
    }
}
