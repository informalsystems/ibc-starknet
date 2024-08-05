use cgp_core::prelude::*;
pub use hermes_relayer_components::chain::traits::send_message::MessageSenderComponent;
pub use hermes_relayer_components::chain::traits::types::event::EventTypeComponent;
pub use hermes_relayer_components::chain::traits::types::message::MessageTypeComponent;
use hermes_relayer_components::error::impls::retry::ReturnRetryable;
pub use hermes_relayer_components::error::traits::retry::RetryableErrorComponent;
pub use hermes_relayer_components::transaction::impls::poll_tx_response::PollTimeoutGetterComponent;
use hermes_relayer_components::transaction::impls::poll_tx_response::PollTxResponse;
pub use hermes_relayer_components::transaction::traits::poll_tx_response::TxResponsePollerComponent;
pub use hermes_relayer_components::transaction::traits::query_tx_response::TxResponseQuerierComponent;
pub use hermes_relayer_components::transaction::traits::submit_tx::TxSubmitterComponent;
pub use hermes_relayer_components::transaction::traits::types::transaction::TransactionTypeComponent;
pub use hermes_relayer_components::transaction::traits::types::tx_hash::TransactionHashTypeComponent;
pub use hermes_relayer_components::transaction::traits::types::tx_response::TxResponseTypeComponent;

use crate::impls::contract::call::CallStarknetContract;
use crate::impls::contract::invoke::InvokeStarknetContract;
use crate::impls::contract::message::BuildInvokeContractCall;
use crate::impls::messages::transfer::BuildTransferErc20TokenMessage;
use crate::impls::queries::token_balance::QueryErc20TokenBalance;
use crate::impls::send_message::SendCallMessages;
use crate::impls::submit_tx::SubmitCallTransaction;
use crate::impls::transfer::TransferErc20Token;
use crate::impls::tx_response::{DefaultPollTimeout, QueryTransactionReceipt};
use crate::impls::types::address::ProvideFeltAddressType;
use crate::impls::types::amount::ProvideU256Amount;
use crate::impls::types::blob::ProvideFeltBlobType;
use crate::impls::types::event::ProvideDummyEvent;
use crate::impls::types::message::ProvideCallMessage;
use crate::impls::types::method::ProvideFeltMethodSelector;
use crate::impls::types::transaction::ProvideCallTransaction;
use crate::impls::types::tx_hash::ProvideFeltTxHash;
use crate::impls::types::tx_response::ProvideTransactionReceipt;
pub use crate::traits::contract::call::ContractCallerComponent;
pub use crate::traits::contract::invoke::ContractInvokerComponent;
pub use crate::traits::contract::message::InvokeContractMessageBuilderComponent;
pub use crate::traits::messages::transfer::TransferTokenMessageBuilderComponent;
pub use crate::traits::queries::token_balance::TokenBalanceQuerierComponent;
pub use crate::traits::transfer::TokenTransferComponent;
pub use crate::traits::types::address::AddressTypeComponent;
pub use crate::traits::types::amount::AmountTypeComponent;
pub use crate::traits::types::blob::BlobTypeComponent;
pub use crate::traits::types::method::MethodSelectorTypeComponent;

define_components! {
    StarknetChainComponents {
        AddressTypeComponent:
            ProvideFeltAddressType,
        BlobTypeComponent:
            ProvideFeltBlobType,
        MessageTypeComponent:
            ProvideCallMessage,
        EventTypeComponent:
            ProvideDummyEvent,
        AmountTypeComponent:
            ProvideU256Amount,
        TransactionTypeComponent:
            ProvideCallTransaction,
        TransactionHashTypeComponent:
            ProvideFeltTxHash,
        TxResponseTypeComponent:
            ProvideTransactionReceipt,
        MethodSelectorTypeComponent:
            ProvideFeltMethodSelector,
        MessageSenderComponent:
            SendCallMessages,
        TxSubmitterComponent:
            SubmitCallTransaction,
        TxResponseQuerierComponent:
            QueryTransactionReceipt,
        TxResponsePollerComponent:
            PollTxResponse,
        PollTimeoutGetterComponent:
            DefaultPollTimeout,
        ContractCallerComponent:
            CallStarknetContract,
        ContractInvokerComponent:
            InvokeStarknetContract,
        InvokeContractMessageBuilderComponent:
            BuildInvokeContractCall,
        RetryableErrorComponent:
            ReturnRetryable<false>,
        TransferTokenMessageBuilderComponent:
            BuildTransferErc20TokenMessage,
        TokenTransferComponent:
            TransferErc20Token,
        TokenBalanceQuerierComponent:
            QueryErc20TokenBalance,
    }
}
