use cgp_core::error::HasErrorType;
use hermes_relayer_components::chain::traits::send_message::MessageSender;
use hermes_relayer_components::chain::traits::types::event::HasEventType;
use hermes_relayer_components::chain::traits::types::message::HasMessageType;
use hermes_relayer_components::transaction::traits::poll_tx_response::CanPollTxResponse;
use hermes_relayer_components::transaction::traits::submit_tx::CanSubmitTx;
use hermes_relayer_components::transaction::traits::types::tx_response::HasTxResponseType;
use starknet::accounts::Call;
use starknet::core::types::{ExecuteInvocation, OrderedEvent, TransactionTrace};

use crate::types::tx_response::TxResponse;

pub struct SendCallMessages;

impl<Chain> MessageSender<Chain> for SendCallMessages
where
    Chain: HasMessageType<Message = Call>
        + CanSubmitTx<Transaction = Vec<Call>>
        + HasTxResponseType<TxResponse = TxResponse>
        + CanPollTxResponse
        + HasEventType<Event = OrderedEvent>
        + HasErrorType,
{
    async fn send_messages(
        chain: &Chain,
        messages: Vec<Call>,
    ) -> Result<Vec<Vec<Chain::Event>>, Chain::Error> {
        let tx_hash = chain.submit_tx(&messages).await?;

        let tx_response = chain.poll_tx_response(&tx_hash).await?;

        println!("tx response: {:?}", tx_response);

        match tx_response.trace {
            TransactionTrace::Invoke(trace) => match trace.execute_invocation {
                ExecuteInvocation::Success(trace) => {
                    let events = trace.calls.into_iter().map(|call| call.events).collect();

                    println!("events: {:?}", events);

                    Ok(events)
                }
                ExecuteInvocation::Reverted(trace) => {
                    todo!()
                }
            },
            _ => {
                todo!()
            }
        }
    }
}
