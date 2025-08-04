use hermes_core::chain_components::traits::{
    HasEvidenceType, HasMessageType, MisbehaviourMessageBuilder,
    MisbehaviourMessageBuilderComponent,
};
use hermes_prelude::*;

#[cgp_new_provider(MisbehaviourMessageBuilderComponent)]
impl<Chain, Counterparty> MisbehaviourMessageBuilder<Chain, Counterparty>
    for CosmosFromStarknetMisbehaviourMessageBuilder
where
    Chain: HasEvidenceType + HasMessageType + HasAsyncErrorType,
{
    async fn build_misbehaviour_message(
        _chain: &Chain,
        evidence: &Chain::Evidence,
    ) -> Result<Chain::Message, Chain::Error> {
        // TODO
        todo!();
    }
}
