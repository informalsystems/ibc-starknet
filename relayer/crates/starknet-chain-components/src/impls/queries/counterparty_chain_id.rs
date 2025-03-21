use core::marker::PhantomData;

use cgp::prelude::*;
use hermes_chain_components::traits::queries::chain_status::CanQueryChainHeight;
use hermes_chain_components::traits::queries::channel_end::CanQueryChannelEnd;
use hermes_chain_components::traits::queries::client_state::CanQueryClientState;
use hermes_chain_components::traits::queries::connection_end::CanQueryConnectionEnd;
use hermes_chain_components::traits::queries::counterparty_chain_id::{
    CounterpartyChainIdQuerier, CounterpartyChainIdQuerierComponent,
};
use hermes_chain_components::traits::types::channel::HasChannelEndType;
use hermes_chain_components::traits::types::client_state::HasClientStateFields;
use hermes_chain_components::traits::types::connection::HasConnectionEndType;
use hermes_chain_components::traits::types::ibc::{
    HasChannelIdType, HasClientIdType, HasConnectionIdType, HasPortIdType,
};

use crate::types::channel_id::ChannelEnd;
use crate::types::client_id::ClientId;
use crate::types::connection_id::{ConnectionEnd, ConnectionId};

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
