use core::fmt::Debug;

use cgp::core::error::CanRaiseError;
use hermes_relayer_components::chain::traits::send_message::MessageSender;
use hermes_relayer_components::chain::traits::types::event::HasEventType;
use hermes_relayer_components::chain::traits::types::message::HasMessageType;
use hermes_relayer_components::transaction::traits::poll_tx_response::CanPollTxResponse;
use hermes_relayer_components::transaction::traits::submit_tx::CanSubmitTx;
use hermes_relayer_components::transaction::traits::types::tx_response::HasTxResponseType;
use starknet::accounts::Call;
use starknet::core::types::{
    ExecuteInvocation, FunctionInvocation, RevertedInvocation, TransactionTrace,
};
use starknet::macros::selector;

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
                ExecuteInvocation::Success(invocation) => {
                    let events = invocation
                        .calls
                        .into_iter()
                        .map(extract_events_from_function_invocation)
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

pub fn extract_events_from_function_invocation(
    invocation: FunctionInvocation,
) -> Vec<StarknetEvent> {
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

    // We retrofit the result returned from a call as an event,
    // so that we can make it work the same way as Cosmos messages.
    // TODO: use a different type to differentiate result events
    let result_event = StarknetEvent {
        contract_address: invocation.contract_address,
        class_hash: invocation.class_hash,
        selector: Some(selector!("result")),
        keys: Vec::new(),
        data: invocation.result,
    };

    events.push(result_event);

    for inner in invocation.calls {
        let mut in_events = extract_events_from_function_invocation(inner);
        events.append(&mut in_events);
    }

    events
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
