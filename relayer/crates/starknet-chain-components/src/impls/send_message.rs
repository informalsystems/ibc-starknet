use cgp_core::error::HasErrorType;
use hermes_relayer_components::chain::traits::send_message::MessageSender;
use hermes_relayer_components::chain::traits::types::event::HasEventType;
use hermes_relayer_components::chain::traits::types::message::HasMessageType;
use hermes_relayer_components::transaction::traits::poll_tx_response::CanPollTxResponse;
use hermes_relayer_components::transaction::traits::submit_tx::CanSubmitTx;
use hermes_relayer_components::transaction::traits::types::tx_response::HasTxResponseType;
use starknet::accounts::Call;
use starknet::core::types::TransactionReceiptWithBlockInfo;

pub struct SendCallMessages;

impl<Chain> MessageSender<Chain> for SendCallMessages
where
    Chain: HasMessageType<Message = Call>
        + CanSubmitTx<Transaction = Vec<Call>>
        + HasTxResponseType<TxResponse = TransactionReceiptWithBlockInfo>
        + CanPollTxResponse
        + HasEventType
        + HasErrorType,
{
    async fn send_messages(
        chain: &Chain,
        messages: Vec<Call>,
    ) -> Result<Vec<Vec<Chain::Event>>, Chain::Error> {
        // stub events
        let events = messages.iter().map(|_| Vec::new()).collect();

        let tx_hash = chain.submit_tx(&messages).await?;

        let receipt = chain.poll_tx_response(&tx_hash).await?;

        println!("tx resceipt: {:?}", receipt);

        Ok(events)
    }
}
