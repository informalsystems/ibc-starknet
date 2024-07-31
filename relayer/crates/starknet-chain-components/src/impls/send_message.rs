use hermes_relayer_components::chain::traits::send_message::MessageSender;
use hermes_relayer_components::chain::traits::types::event::HasEventType;
use hermes_relayer_components::chain::traits::types::message::HasMessageType;
use starknet::accounts::{Account, Call};

use crate::traits::account::{CanRaiseAccountErrors, HasStarknetAccount};
use crate::traits::provider::HasStarknetProvider;

pub struct SendCallMessages;

impl<Chain> MessageSender<Chain> for SendCallMessages
where
    Chain: HasMessageType<Message = Call>
        + HasEventType
        + HasStarknetProvider
        + HasStarknetAccount
        + CanRaiseAccountErrors,
{
    async fn send_messages(
        chain: &Chain,
        messages: Vec<Call>,
    ) -> Result<Vec<Vec<Chain::Event>>, Chain::Error> {
        let account = chain.account();

        let execution = account.execute_v3(messages);

        let _tx_hash = execution
            .send()
            .await
            .map_err(Chain::raise_error)?
            .transaction_hash;

        Ok(Vec::new())
    }
}
