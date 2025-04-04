use core::fmt::Debug;
use std::sync::Arc;

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
use hermes_relayer_components::transaction::traits::types::tx_hash::HasTxHashType;
use hermes_relayer_components::transaction::traits::types::tx_response::HasTxResponseType;
use hermes_starknet_chain_components::impls::types::address::StarknetAddress;
use hermes_starknet_chain_components::impls::types::message::StarknetMessage;
use hermes_starknet_chain_components::traits::account::{
    CanBuildAccountFromSigner, HasStarknetAccountType,
};
use hermes_starknet_chain_components::types::event::{StarknetEvent, StarknetEventFields};
use hermes_starknet_chain_components::types::message_response::StarknetMessageResponse;
use starknet_v13::accounts::{Account, Call};
use starknet_v13::core::types::{
    ExecuteInvocation, Felt, FunctionInvocation, OrderedEvent, RevertedInvocation, TransactionTrace,
};

use crate::traits::CanUseStarknetAccount;
use crate::types::TxResponse;

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
        + HasTxHashType<TxHash = Felt>
        + CanPollTxResponse
        + CanRaiseAsyncError<&'static str>
        + CanUseStarknetAccount,
{
    async fn send_messages_with_signer_and_nonce(
        chain: &Chain,
        signer: &Chain::Signer,
        nonce: &Felt,
        messages: &[StarknetMessage],
    ) -> Result<Chain::TxResponse, Chain::Error> {
        let calls: Vec<Call> = messages
            .iter()
            .map(|message| Call {
                to: message.to,
                selector: message.selector,
                calldata: message.calldata.clone(),
            })
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
            from_ordered_event(
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

pub fn from_ordered_event(
    contract_address: StarknetAddress,
    class_hash: Felt,
    event: OrderedEvent,
) -> StarknetEvent {
    let mut keys = event.keys.into_iter();
    let data = event.data;

    let selector = keys.next();

    StarknetEvent {
        fields: Arc::new(StarknetEventFields {
            contract_address,
            class_hash: Some(class_hash),
            selector,
            keys: keys.collect(),
            data,
        }),
    }
}
