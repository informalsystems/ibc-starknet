use hermes_chain_components::traits::types::height::HasHeightType;
use hermes_chain_components::traits::types::ibc::CounterpartyMessageHeightGetter;
use hermes_chain_components::traits::types::message::HasMessageType;
use ibc::core::client::types::Height;

use crate::impls::types::message::StarknetMessage;

pub struct GetCounterpartyCosmosHeightFromStarknetMessage;

impl<Chain, Counterparty> CounterpartyMessageHeightGetter<Chain, Counterparty>
    for GetCounterpartyCosmosHeightFromStarknetMessage
where
    Chain: HasMessageType<Message = StarknetMessage>,
    Counterparty: HasHeightType<Height = Height>,
{
    fn counterparty_message_height_for_update_client(
        message: &Chain::Message,
    ) -> Option<Counterparty::Height> {
        message.counterparty_height
    }
}
