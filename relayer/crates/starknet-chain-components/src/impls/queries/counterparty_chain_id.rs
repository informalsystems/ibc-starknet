use cgp::prelude::HasAsyncErrorType;
use hermes_chain_components::traits::queries::counterparty_chain_id::CounterpartyChainIdQuerier;
use hermes_chain_components::traits::types::chain_id::HasChainIdType;
use hermes_chain_components::traits::types::ibc::{HasChannelIdType, HasPortIdType};

pub struct QueryCosmosChainIdFromStarknetChannelId;

impl<Chain, Counterparty> CounterpartyChainIdQuerier<Chain, Counterparty>
    for QueryCosmosChainIdFromStarknetChannelId
where
    Chain: HasChannelIdType<Counterparty> + HasPortIdType<Counterparty> + HasAsyncErrorType,
    Counterparty: HasChainIdType,
{
    async fn query_counterparty_chain_id_from_channel_id(
        _chain: &Chain,
        _channel_id: &Chain::ChannelId,
        _port_id: &Chain::PortId,
    ) -> Result<Counterparty::ChainId, Chain::Error> {
        todo!()
    }
}
