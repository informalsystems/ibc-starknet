use core::marker::PhantomData;

use hermes_core::chain_components::traits::{
    CanMeasureTime, CanQueryChainStatus, CanQueryClientStateWithLatestHeight,
    CanQueryConsensusStateWithLatestHeight, ClientStatus, ClientStatusQuerier,
    ClientStatusQuerierComponent, HasClientIdType, HasClientStateFields, HasClientStateType,
    HasClientStatusType, HasConsensusStateFields,
};
use hermes_prelude::*;

use crate::types::{ClientId, CometClientState};

pub struct QueryStarknetClientStatus;

#[cgp_provider(ClientStatusQuerierComponent)]
impl<Chain, Counterparty> ClientStatusQuerier<Chain, Counterparty> for QueryStarknetClientStatus
where
    Chain: HasClientIdType<Counterparty, ClientId = ClientId>
        + CanQueryClientStateWithLatestHeight<Counterparty>
        + CanQueryConsensusStateWithLatestHeight<Counterparty>
        + CanQueryChainStatus
        + CanMeasureTime,
    Counterparty: HasClientStateType<Chain, ClientState = CometClientState>
        + HasClientStateFields<Chain>
        + HasConsensusStateFields<Chain>
        + HasClientStatusType<Chain, ClientStatus = ClientStatus>,
{
    async fn query_client_status(
        chain: &Chain,
        _tag: PhantomData<Counterparty>,
        client_id: &ClientId,
    ) -> Result<Counterparty::ClientStatus, Chain::Error> {
        let client_state = chain
            .query_client_state_with_latest_height(PhantomData, client_id)
            .await?;

        if Counterparty::client_state_is_frozen(&client_state) {
            return Ok(ClientStatus::Frozen);
        }

        let client_latest_height = Counterparty::client_state_latest_height(&client_state);

        let latest_consensus_state = chain
            .query_consensus_state_with_latest_height(PhantomData, client_id, &client_latest_height)
            .await?;

        let latest_consensus_state_timestamp =
            Counterparty::consensus_state_timestamp(&latest_consensus_state);

        let chain_status = chain.query_chain_status().await?;

        let current_network_time = Chain::chain_status_time(&chain_status);

        let elapsed =
            Chain::duration_since(&latest_consensus_state_timestamp, current_network_time);

        if elapsed
            .is_some_and(|elapsed| Counterparty::client_state_has_expired(&client_state, elapsed))
        {
            return Ok(ClientStatus::Expired);
        }

        Ok(ClientStatus::Active)
    }
}
