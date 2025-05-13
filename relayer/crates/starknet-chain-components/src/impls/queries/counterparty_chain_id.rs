use core::marker::PhantomData;

use hermes_core::chain_components::traits::{
    CanQueryChainHeight, CanQueryChannelEnd, CanQueryClientState, CanQueryConnectionEnd,
    CounterpartyChainIdQuerier, CounterpartyChainIdQuerierComponent, HasChannelEndType,
    HasChannelIdType, HasClientIdType, HasClientStateFields, HasConnectionEndType,
    HasConnectionIdType, HasPortIdType,
};
use hermes_prelude::*;

use crate::types::{ChannelEnd, ClientId, ConnectionEnd, ConnectionId};

pub struct QueryCosmosChainIdFromStarknetChannelId;

#[cgp_provider(CounterpartyChainIdQuerierComponent)]
impl<Chain, Counterparty> CounterpartyChainIdQuerier<Chain, Counterparty>
    for QueryCosmosChainIdFromStarknetChannelId
where
    Chain: HasChannelIdType<Counterparty>
        + HasPortIdType<Counterparty>
        + HasClientIdType<Counterparty, ClientId = ClientId>
        + HasConnectionIdType<Counterparty, ConnectionId = ConnectionId>
        + HasChannelEndType<Counterparty, ChannelEnd = ChannelEnd>
        + HasConnectionEndType<Counterparty, ConnectionEnd = ConnectionEnd>
        + CanQueryChainHeight
        + CanQueryChannelEnd<Counterparty>
        + CanQueryConnectionEnd<Counterparty>
        + CanQueryClientState<Counterparty>,
    Counterparty: HasClientStateFields<Chain>,
{
    async fn query_counterparty_chain_id_from_channel_id(
        chain: &Chain,
        channel_id: &Chain::ChannelId,
        port_id: &Chain::PortId,
    ) -> Result<Counterparty::ChainId, Chain::Error> {
        let height = chain.query_chain_height().await?;

        let channel_end = chain
            .query_channel_end(channel_id, port_id, &height)
            .await?;

        let connection_end = chain
            .query_connection_end(&channel_end.connection_hops[0], &height)
            .await?;

        let client_state = chain
            .query_client_state(PhantomData, connection_end.client_id(), &height)
            .await?;

        let chain_id = Counterparty::client_state_chain_id(&client_state);

        Ok(chain_id)
    }
}
