use std::sync::Arc;

use cgp_core::error::{DelegateErrorRaiser, ErrorRaiserComponent, ErrorTypeComponent};
use cgp_core::prelude::*;
use hermes_error::impls::ProvideHermesError;
use hermes_logging_components::contexts::no_logger::ProvideNoLogger;
use hermes_logging_components::traits::has_logger::{
    GlobalLoggerGetterComponent, HasLogger, LoggerGetterComponent, LoggerTypeComponent,
};
use hermes_relayer_components::chain::traits::send_message::CanSendMessages;
use hermes_relayer_components::error::traits::retry::HasRetryableError;
use hermes_relayer_components::transaction::traits::poll_tx_response::CanPollTxResponse;
use hermes_relayer_components::transaction::traits::query_tx_response::CanQueryTxResponse;
use hermes_relayer_components::transaction::traits::submit_tx::CanSubmitTx;
use hermes_runtime::types::runtime::HermesRuntime;
use hermes_runtime_components::traits::runtime::{
    HasRuntime, ProvideDefaultRuntimeField, RuntimeGetterComponent, RuntimeTypeComponent,
};
use hermes_starknet_chain_components::components::*;
use hermes_starknet_chain_components::impls::account::GetStarknetAccountField;
use hermes_starknet_chain_components::impls::provider::GetStarknetProviderField;
use hermes_starknet_chain_components::traits::account::{
    HasStarknetAccount, StarknetAccountGetterComponent, StarknetAccountTypeComponent,
};
use hermes_starknet_chain_components::traits::client::JsonRpcClientGetter;
use hermes_starknet_chain_components::traits::contract::call::CanCallContract;
use hermes_starknet_chain_components::traits::contract::invoke::CanInvokeContract;
use hermes_starknet_chain_components::traits::provider::{
    HasStarknetProvider, StarknetProviderGetterComponent, StarknetProviderTypeComponent,
};
use hermes_starknet_chain_components::traits::queries::token_balance::CanQueryTokenBalance;
use hermes_starknet_chain_components::traits::transfer::CanTransferToken;
use hermes_starknet_chain_components::traits::types::address::HasAddressType;
use hermes_starknet_chain_components::traits::types::blob::HasBlobType;
use hermes_starknet_chain_components::traits::types::method::HasMethodSelectorType;
use starknet::accounts::SingleOwnerAccount;
use starknet::core::types::Felt;
use starknet::providers::jsonrpc::HttpTransport;
use starknet::providers::JsonRpcClient;
use starknet::signers::LocalWallet;

use crate::impls::error::HandleStarknetError;

#[derive(HasField)]
pub struct StarknetChain {
    pub runtime: HermesRuntime,
    pub rpc_client: Arc<JsonRpcClient<HttpTransport>>,
    pub account: SingleOwnerAccount<Arc<JsonRpcClient<HttpTransport>>, LocalWallet>,
}

pub struct StarknetChainContextComponents;

impl HasComponents for StarknetChain {
    type Components = StarknetChainContextComponents;
}

delegate_components! {
    StarknetChainContextComponents {
        ErrorTypeComponent: ProvideHermesError,
        ErrorRaiserComponent: DelegateErrorRaiser<HandleStarknetError>,
        [
            RuntimeTypeComponent,
            RuntimeGetterComponent,
        ]:
            ProvideDefaultRuntimeField,
        [
            LoggerTypeComponent,
            LoggerGetterComponent,
            GlobalLoggerGetterComponent,
        ]:
            ProvideNoLogger,
        [
            StarknetProviderTypeComponent,
            StarknetProviderGetterComponent,
        ]:
            GetStarknetProviderField<symbol!("rpc_client")>,
        [
            StarknetAccountTypeComponent,
            StarknetAccountGetterComponent,
        ]:
            GetStarknetAccountField<symbol!("account")>,
    }
}

with_starknet_chain_components! {
    delegate_components! {
        StarknetChainContextComponents {
            @StarknetChainComponents: StarknetChainComponents,
        }
    }
}

impl JsonRpcClientGetter<StarknetChain> for StarknetChainContextComponents {
    fn json_rpc_client(chain: &StarknetChain) -> &JsonRpcClient<HttpTransport> {
        &chain.rpc_client
    }
}

pub trait CanUseStarknetChain:
    HasRuntime
    + HasLogger
    + HasAddressType<Address = Felt>
    + HasMethodSelectorType<MethodSelector = Felt>
    + HasBlobType<Blob = Vec<Felt>>
    + HasStarknetProvider
    + HasStarknetAccount
    + CanSendMessages
    + CanSubmitTx
    + CanQueryTxResponse
    + CanPollTxResponse
    + CanCallContract
    + CanInvokeContract
    + CanQueryTokenBalance
    + CanTransferToken
    + HasRetryableError
{
}

impl CanUseStarknetChain for StarknetChain {}
