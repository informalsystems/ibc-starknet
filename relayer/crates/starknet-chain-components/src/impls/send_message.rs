use core::fmt::Debug;

use cgp::core::error::CanRaiseError;
use hermes_relayer_components::chain::traits::send_message::MessageSender;
use hermes_relayer_components::chain::traits::types::event::HasEventType;
use hermes_relayer_components::chain::traits::types::message::HasMessageType;
use hermes_relayer_components::transaction::traits::poll_tx_response::CanPollTxResponse;
use hermes_relayer_components::transaction::traits::submit_tx::CanSubmitTx;
use hermes_relayer_components::transaction::traits::types::tx_response::HasTxResponseType;
use starknet::accounts::Call;
use starknet::core::types::{ExecuteInvocation, RevertedInvocation, TransactionTrace};

use crate::types::event::StarknetEvent;
use crate::types::tx_response::TxResponse;

pub struct SendCallMessages;

pub struct UnexpectedTransactionTraceType {
    pub trace: TransactionTrace,
}

impl<Chain> MessageSender<Chain> for SendCallMessages
where
    Chain: HasMessageType<Message = Call>
        + CanSubmitTx<Transaction = Vec<Call>>
        + HasTxResponseType<TxResponse = TxResponse>
        + HasEventType<Event = StarknetEvent>
        + CanPollTxResponse
        + CanRaiseError<RevertedInvocation>
        + CanRaiseError<UnexpectedTransactionTraceType>,
{
    async fn send_messages(
        chain: &Chain,
        messages: Vec<Call>,
    ) -> Result<Vec<Vec<Chain::Event>>, Chain::Error> {
        let tx_hash = chain.submit_tx(&messages).await?;

        let tx_response = chain.poll_tx_response(&tx_hash).await?;

        match tx_response.trace {
            TransactionTrace::Invoke(trace) => match trace.execute_invocation {
                ExecuteInvocation::Success(trace) => {
                    let events = trace
                        .calls
                        .into_iter()
                        .map(|call| {
                            call.events
                                .into_iter()
                                .map(|event| {
                                    StarknetEvent::from_ordered_event(
                                        call.contract_address,
                                        call.class_hash,
                                        event,
                                    )
                                })
                                .collect()
                        })
                        .collect();

                    Ok(events)
                }
                ExecuteInvocation::Reverted(trace) => Err(Chain::raise_error(trace)),
            },
            trace => {
                // The transaction for sending Call messages should always return an Invoke trace.
                // The other type of transactions such as Declare would have to be submitted as non message.
                Err(Chain::raise_error(UnexpectedTransactionTraceType { trace }))
            }
        }
    }
}

impl Debug for UnexpectedTransactionTraceType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "Expected transaction trace to be of type Invoke, but instead got: {:?}",
            self.trace
        )
    }
}
