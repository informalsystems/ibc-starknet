use hermes_core::chain_components::traits::{
    HasChainId, HasEvidenceType, HasMessageType, MisbehaviourMessageBuilder,
    MisbehaviourMessageBuilderComponent,
};
use hermes_core::logging_components::traits::CanLog;
use hermes_core::logging_components::types::LevelWarn;
use hermes_prelude::*;

#[cgp_new_provider(MisbehaviourMessageBuilderComponent)]
impl<Chain, Counterparty> MisbehaviourMessageBuilder<Chain, Counterparty>
    for CosmosFromStarknetMisbehaviourMessageBuilder
where
    Chain: HasEvidenceType + HasChainId + CanLog<LevelWarn> + HasMessageType + HasAsyncErrorType,
{
    async fn build_misbehaviour_message(
        chain: &Chain,
        evidence: &Chain::Evidence,
    ) -> Result<Chain::Message, Chain::Error> {
        chain
            .log(
                &format!(
                    "CosmosFromStarknetMisbehaviourMessageBuilder {}",
                    chain.chain_id()
                ),
                &LevelWarn,
            )
            .await;
        // TODO
        todo!();
    }
}
