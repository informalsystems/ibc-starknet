use hermes_core::chain_components::traits::{
    HasClientStateType, HasEvidenceType, HasUpdateClientEvent, MisbehaviourChecker,
    MisbehaviourCheckerComponent,
};
use hermes_core::logging_components::traits::CanLog;
use hermes_core::logging_components::types::LevelWarn;
use hermes_prelude::*;

#[cgp_new_provider(MisbehaviourCheckerComponent)]
impl<Chain, Counterparty> MisbehaviourChecker<Chain, Counterparty> for CheckStarknetMisbehaviour
where
    Chain: HasClientStateType<Counterparty> + CanLog<LevelWarn> + HasAsyncErrorType,
    Counterparty: HasUpdateClientEvent + HasEvidenceType + Async,
{
    async fn check_misbehaviour(
        chain: &Chain,
        update_client_event: &Counterparty::UpdateClientEvent,
        client_state: &Chain::ClientState,
    ) -> Result<Option<Counterparty::Evidence>, Chain::Error> {
        chain
            .log("CheckStarknetMisbehaviour check_misbehaviour", &LevelWarn)
            .await;
        Ok(None)
    }
}
