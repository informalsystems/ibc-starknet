use cgp::prelude::*;
use hermes_core::chain_components::traits::{
    CounterpartyMessageHeightGetter, CounterpartyMessageHeightGetterComponent, HasHeightType,
    HasMessageType,
};
use ibc::core::client::types::Height;

use crate::impls::types::message::StarknetMessage;

pub struct GetCounterpartyCosmosHeightFromStarknetMessage;

#[cgp_provider(CounterpartyMessageHeightGetterComponent)]
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
