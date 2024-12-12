use hermes_chain_components::traits::types::height::HasHeightType;
use hermes_chain_components::traits::types::ibc::CounterpartyMessageHeightGetter;
use hermes_chain_components::traits::types::message::HasMessageType;

pub struct GetCounterpartyCosmosHeightFromStarknetMessage;

impl<Chain, Counterparty> CounterpartyMessageHeightGetter<Chain, Counterparty>
    for GetCounterpartyCosmosHeightFromStarknetMessage
where
    Chain: HasMessageType,
    Counterparty: HasHeightType,
{
    fn counterparty_message_height_for_update_client(
        _message: &Chain::Message,
    ) -> Option<Counterparty::Height> {
        // TODO: Define a `StarknetMessage` type that wraps around `Call`
        // and provide counterparty height
        None
    }
}
