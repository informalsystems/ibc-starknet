use core::fmt::Debug;

use cgp::prelude::*;
use hermes_chain_type_components::traits::types::message_response::HasMessageResponseType;
use hermes_relayer_components::chain::traits::types::message::HasMessageType;
use hermes_relayer_components::transaction::traits::parse_events::{
    TxMessageResponseParser, TxMessageResponseParserComponent,
};
use hermes_relayer_components::transaction::traits::poll_tx_response::CanPollTxResponse;
use hermes_relayer_components::transaction::traits::send_messages_with_signer_and_nonce::{
    MessagesWithSignerAndNonceSender, MessagesWithSignerAndNonceSenderComponent,
};
use hermes_relayer_components::transaction::traits::types::nonce::HasNonceType;
use hermes_relayer_components::transaction::traits::types::signer::HasSignerType;
use hermes_relayer_components::transaction::traits::types::tx_hash::HasTransactionHashType;
use hermes_relayer_components::transaction::traits::types::tx_response::HasTxResponseType;
use starknet::accounts::{Account, Call};
use starknet::core::types::{
    ExecuteInvocation, Felt, FunctionInvocation, RevertedInvocation, TransactionTrace,
};

use crate::impls::types::message::StarknetMessage;
use crate::traits::account::{
    CanBuildAccountFromSigner, CanRaiseAccountErrors, HasStarknetAccountType,
};
use crate::types::event::StarknetEvent;
use crate::types::message_response::StarknetMessageResponse;
use crate::types::tx_response::TxResponse;

pub struct UnexpectedTransactionTraceType {
    pub trace: TransactionTrace,
}

#[cgp_new_provider(MessagesWithSignerAndNonceSenderComponent)]
impl<Chain> MessagesWithSignerAndNonceSender<Chain> for SendStarknetMessages
where
    Chain: HasStarknetAccountType
        + HasSignerType
        + CanBuildAccountFromSigner
        + HasNonceType<Nonce = Felt>
        + HasMessageType<Message = StarknetMessage>
        + HasTransactionHashType<TxHash = Felt>
        + CanPollTxResponse
        + CanRaiseAccountErrors,
{
    async fn send_messages_with_signer_and_nonce(
        chain: &Chain,
        signer: &Chain::Signer,
        nonce: &Felt,
        messages: &[StarknetMessage],
    ) -> Result<Chain::TxResponse, Chain::Error> {
        let calls: Vec<Call> = messages
            .iter()
            .map(|message| message.call.clone())
            .collect();

        let account = chain.build_account_from_signer(signer);

        let execution = account.execute_v3(calls).nonce(*nonce);

        let tx_hash = execution
            .send()
            .await
            .map_err(Chain::raise_error)?
            .transaction_hash;

        chain.poll_tx_response(&tx_hash).await
    }
}

#[cgp_provider(TxMessageResponseParserComponent)]
impl<Chain> TxMessageResponseParser<Chain> for SendStarknetMessages
where
    Chain: HasTxResponseType<TxResponse = TxResponse>
        + HasMessageResponseType<MessageResponse = StarknetMessageResponse>
        + CanRaiseAsyncError<RevertedInvocation>
        + CanRaiseAsyncError<UnexpectedTransactionTraceType>,
{
    fn parse_tx_message_response(
        tx_response: TxResponse,
    ) -> Result<Vec<StarknetMessageResponse>, Chain::Error> {
        match tx_response.trace {
            TransactionTrace::Invoke(trace) => match trace.execute_invocation {
                ExecuteInvocation::Success(invocation) => {
                    let message_responses = invocation
                        .calls
                        .into_iter()
                        .map(extract_message_response_from_function_invocation)
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

pub fn extract_message_response_from_function_invocation(
    invocation: FunctionInvocation,
) -> StarknetMessageResponse {
    let result = invocation.result.clone();
    let events = extract_events_from_function_invocation(invocation);

    StarknetMessageResponse { result, events }
}

pub fn extract_events_from_function_invocation(
    invocation: FunctionInvocation,
) -> Vec<StarknetEvent> {
    let mut events: Vec<StarknetEvent> = invocation
        .events
        .into_iter()
        .map(|event| {
            StarknetEvent::from_ordered_event(
                invocation.contract_address.into(),
                invocation.class_hash,
                event,
            )
        })
        .collect();

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
