use core::fmt::Debug;

use cgp::core::error::CanRaiseError;
use hermes_chain_type_components::traits::types::message_response::HasMessageResponseType;
use hermes_relayer_components::chain::traits::send_message::MessageSender;
use hermes_relayer_components::chain::traits::types::message::HasMessageType;
use hermes_relayer_components::transaction::traits::poll_tx_response::CanPollTxResponse;
use hermes_relayer_components::transaction::traits::submit_tx::CanSubmitTx;
use hermes_relayer_components::transaction::traits::types::tx_response::HasTxResponseType;
use starknet::accounts::Call;
use starknet::core::types::{
    ExecuteInvocation, FunctionInvocation, RevertedInvocation, TransactionTrace,
};

use crate::impls::types::message::StarknetMessage;
use crate::types::event::StarknetEvent;
use crate::types::message_response::StarknetMessageResponse;
use crate::types::tx_response::TxResponse;

pub struct SendCallMessages;

pub struct UnexpectedTransactionTraceType {
    pub trace: TransactionTrace,
}

impl<Chain> MessageSender<Chain> for SendCallMessages
where
    Chain: HasMessageType<Message = StarknetMessage>
        + CanSubmitTx<Transaction = Vec<Call>>
        + HasTxResponseType<TxResponse = TxResponse>
        + HasMessageResponseType<MessageResponse = StarknetMessageResponse>
        + CanPollTxResponse
        + CanRaiseError<RevertedInvocation>
        + CanRaiseError<UnexpectedTransactionTraceType>,
{
    async fn send_messages(
        chain: &Chain,
        messages: Vec<StarknetMessage>,
    ) -> Result<Vec<StarknetMessageResponse>, Chain::Error> {
        let calls: Vec<Call> = messages
            .iter()
            .map(|message| message.call.clone())
            .collect();
        let tx_hash = chain.submit_tx(&calls).await?;

        let tx_response = chain.poll_tx_response(&tx_hash).await?;

        match tx_response.trace {
            TransactionTrace::Invoke(trace) => match trace.execute_invocation {
                ExecuteInvocation::Success(invocation) => {
                    let message_responses = invocation
                        .calls
                        .into_iter()
                        .map(extract_events_from_function_invocation)
                        .collect();

                    Ok(message_responses)
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

pub fn extract_events_from_function_invocation(
    invocation: FunctionInvocation,
) -> StarknetMessageResponse {
    let mut events: Vec<StarknetEvent> = invocation
        .events
        .into_iter()
        .map(|event| {
            StarknetEvent::from_ordered_event(
                invocation.contract_address,
                invocation.class_hash,
                event,
            )
        })
        .collect();

    for inner in invocation.calls {
        let mut message_response = extract_events_from_function_invocation(inner);
        events.append(&mut message_response.events);
    }

    StarknetMessageResponse {
        result: invocation.result,
        events,
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
