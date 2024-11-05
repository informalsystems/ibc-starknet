use std::sync::Arc;

use cgp::core::component::UseDelegate;
use cgp::core::error::{ErrorRaiserComponent, ErrorTypeComponent};
use cgp::prelude::*;
use hermes_cairo_encoding_components::types::as_felt::AsFelt;
use hermes_chain_type_components::traits::types::message_response::HasMessageResponseType;
use hermes_cosmos_chain_components::components::delegate::DelegateCosmosChainComponents;
use hermes_cosmos_chain_components::types::payloads::client::CosmosCreateClientPayload;
use hermes_cosmos_relayer::contexts::chain::CosmosChain;
use hermes_encoding_components::impls::default_encoding::GetDefaultEncoding;
use hermes_encoding_components::traits::has_encoding::{
    DefaultEncodingGetter, EncodingGetterComponent, HasDefaultEncoding, ProvideEncodingType,
};
use hermes_encoding_components::types::AsBytes;
use hermes_error::impls::ProvideHermesError;
use hermes_logging_components::contexts::no_logger::ProvideNoLogger;
use hermes_logging_components::traits::has_logger::{
    GlobalLoggerGetterComponent, HasLogger, LoggerGetterComponent, LoggerTypeComponent,
};
use hermes_relayer_components::chain::traits::message_builders::create_client::CanBuildCreateClientMessage;
use hermes_relayer_components::chain::traits::message_builders::update_client::CanBuildUpdateClientMessage;
use hermes_relayer_components::chain::traits::payload_builders::create_client::CanBuildCreateClientPayload;
use hermes_relayer_components::chain::traits::payload_builders::update_client::CanBuildUpdateClientPayload;
use hermes_relayer_components::chain::traits::queries::chain_status::CanQueryChainStatus;
use hermes_relayer_components::chain::traits::queries::client_state::CanQueryClientState;
use hermes_relayer_components::chain::traits::queries::consensus_state::CanQueryConsensusState;
use hermes_relayer_components::chain::traits::queries::consensus_state_height::CanQueryConsensusStateHeight;
use hermes_relayer_components::chain::traits::send_message::CanSendMessages;
use hermes_relayer_components::chain::traits::types::chain_id::ChainIdGetter;
use hermes_relayer_components::chain::traits::types::client_state::HasClientStateType;
use hermes_relayer_components::chain::traits::types::consensus_state::HasConsensusStateType;
use hermes_relayer_components::chain::traits::types::event::HasEventType;
use hermes_relayer_components::chain::traits::types::ibc::HasClientIdType;
use hermes_relayer_components::chain::traits::types::packet::HasOutgoingPacketType;
use hermes_relayer_components::chain::traits::types::update_client::HasUpdateClientPayloadType;
use hermes_relayer_components::error::traits::retry::HasRetryableError;
use hermes_relayer_components::transaction::traits::poll_tx_response::CanPollTxResponse;
use hermes_relayer_components::transaction::traits::query_tx_response::CanQueryTxResponse;
use hermes_relayer_components::transaction::traits::submit_tx::CanSubmitTx;
use hermes_runtime::types::runtime::HermesRuntime;
use hermes_runtime_components::traits::runtime::{
    HasRuntime, ProvideDefaultRuntimeField, RuntimeGetterComponent, RuntimeTypeComponent,
};
use hermes_starknet_chain_components::components::chain::*;
use hermes_starknet_chain_components::components::starknet_to_cosmos::StarknetToCosmosComponents;
use hermes_starknet_chain_components::impls::account::GetStarknetAccountField;
use hermes_starknet_chain_components::impls::provider::GetStarknetProviderField;
use hermes_starknet_chain_components::traits::account::{
    HasStarknetAccount, StarknetAccountGetterComponent, StarknetAccountTypeComponent,
};
use hermes_starknet_chain_components::traits::client::JsonRpcClientGetter;
use hermes_starknet_chain_components::traits::contract::call::CanCallContract;
use hermes_starknet_chain_components::traits::contract::declare::CanDeclareContract;
use hermes_starknet_chain_components::traits::contract::deploy::CanDeployContract;
use hermes_starknet_chain_components::traits::contract::invoke::CanInvokeContract;
use hermes_starknet_chain_components::traits::provider::{
    HasStarknetProvider, StarknetProviderGetterComponent, StarknetProviderTypeComponent,
};
use hermes_starknet_chain_components::traits::queries::address::CanQueryContractAddress;
use hermes_starknet_chain_components::traits::queries::token_balance::CanQueryTokenBalance;
use hermes_starknet_chain_components::traits::transfer::CanTransferToken;
use hermes_starknet_chain_components::traits::types::blob::HasBlobType;
use hermes_starknet_chain_components::traits::types::method::HasSelectorType;
use hermes_starknet_chain_components::types::client_id::ClientId;
use hermes_starknet_chain_components::types::client_state::WasmStarknetClientState;
use hermes_starknet_chain_components::types::consensus_state::WasmStarknetConsensusState;
use hermes_starknet_chain_components::types::cosmos::client_state::CometClientState;
use hermes_starknet_chain_components::types::cosmos::consensus_state::CometConsensusState;
use hermes_starknet_chain_components::types::event::StarknetEvent;
use hermes_starknet_chain_components::types::message_response::StarknetMessageResponse;
use hermes_starknet_test_components::impls::types::wallet::ProvideStarknetWalletType;
use hermes_test_components::chain::traits::types::address::HasAddressType;
use hermes_test_components::chain::traits::types::wallet::WalletTypeComponent;
use starknet::accounts::SingleOwnerAccount;
use starknet::core::types::Felt;
use starknet::providers::jsonrpc::HttpTransport;
use starknet::providers::JsonRpcClient;
use starknet::signers::LocalWallet;

use crate::contexts::encoding::cairo::StarknetCairoEncoding;
use crate::contexts::encoding::protobuf::StarknetProtobufEncoding;
use crate::impls::error::HandleStarknetChainError;

#[derive(HasField, Clone)]
pub struct StarknetChain {
    pub runtime: HermesRuntime,
    pub chain_id: Felt,
    pub rpc_client: Arc<JsonRpcClient<HttpTransport>>,
    pub account: SingleOwnerAccount<Arc<JsonRpcClient<HttpTransport>>, LocalWallet>,
    pub ibc_client_contract_address: Option<Felt>,
}

pub struct StarknetChainContextComponents;

impl HasComponents for StarknetChain {
    type Components = StarknetChainContextComponents;
}

delegate_components! {
    StarknetChainContextComponents {
        ErrorTypeComponent: ProvideHermesError,
        ErrorRaiserComponent: UseDelegate<HandleStarknetChainError>,
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
        EncodingGetterComponent: GetDefaultEncoding,
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
        WalletTypeComponent:
            ProvideStarknetWalletType,
    }
}

with_starknet_chain_components! {
    delegate_components! {
        StarknetChainContextComponents {
            @StarknetChainComponents: StarknetChainComponents,
        }
    }
}

delegate_components! {
    DelegateCosmosChainComponents {
        StarknetChain: StarknetToCosmosComponents,
    }
}

impl ProvideEncodingType<StarknetChain, AsFelt> for StarknetChainContextComponents {
    type Encoding = StarknetCairoEncoding;
}

impl DefaultEncodingGetter<StarknetChain, AsFelt> for StarknetChainContextComponents {
    fn default_encoding() -> &'static StarknetCairoEncoding {
        &StarknetCairoEncoding
    }
}

impl ProvideEncodingType<StarknetChain, AsBytes> for StarknetChainContextComponents {
    type Encoding = StarknetProtobufEncoding;
}

impl DefaultEncodingGetter<StarknetChain, AsBytes> for StarknetChainContextComponents {
    fn default_encoding() -> &'static StarknetProtobufEncoding {
        &StarknetProtobufEncoding
    }
}

impl JsonRpcClientGetter<StarknetChain> for StarknetChainContextComponents {
    fn json_rpc_client(chain: &StarknetChain) -> &JsonRpcClient<HttpTransport> {
        &chain.rpc_client
    }
}

impl ChainIdGetter<StarknetChain> for StarknetChainContextComponents {
    fn chain_id(chain: &StarknetChain) -> &Felt {
        &chain.chain_id
    }
}

pub trait CanUseStarknetChain:
    HasRuntime
    + HasLogger
    + HasEventType<Event = StarknetEvent>
    + HasMessageResponseType<MessageResponse = StarknetMessageResponse>
    + HasDefaultEncoding<AsBytes, Encoding = StarknetProtobufEncoding>
    + HasDefaultEncoding<AsFelt, Encoding = StarknetCairoEncoding>
    + HasAddressType<Address = Felt>
    + HasSelectorType<Selector = Felt>
    + HasBlobType<Blob = Vec<Felt>>
    + HasClientStateType<CosmosChain, ClientState = WasmStarknetClientState>
    + HasConsensusStateType<CosmosChain, ConsensusState = WasmStarknetConsensusState>
    + HasClientIdType<CosmosChain, ClientId = ClientId>
    + HasOutgoingPacketType<CosmosChain>
    + HasStarknetProvider
    + HasStarknetAccount
    + CanQueryChainStatus
    + CanSendMessages
    + CanSubmitTx
    + CanQueryTxResponse
    + CanPollTxResponse
    + CanCallContract
    + CanInvokeContract
    + CanDeclareContract
    + CanDeployContract
    + CanQueryTokenBalance
    + CanTransferToken
    + HasRetryableError
    + CanBuildCreateClientPayload<CosmosChain>
    // + CanBuildCreateClientMessage<CosmosChain>
    + CanBuildUpdateClientPayload<CosmosChain>
    + CanQueryClientState<CosmosChain>
    + CanQueryConsensusState<CosmosChain>
    + CanQueryContractAddress<symbol!("ibc_client_contract_address")>
where
    CosmosChain: HasClientStateType<Self> + HasConsensusStateType<Self>,
{
}

impl CanUseStarknetChain for StarknetChain {}

pub trait CanUseCosmosChainWithStarknet:
    HasClientStateType<StarknetChain, ClientState = CometClientState>
    + HasConsensusStateType<StarknetChain, ConsensusState = CometConsensusState>
    + HasUpdateClientPayloadType<StarknetChain, UpdateClientPayload = CosmosCreateClientPayload>
    + CanQueryClientState<StarknetChain>
    + CanQueryConsensusState<StarknetChain>
    + CanBuildCreateClientMessage<StarknetChain>
    + CanBuildUpdateClientPayload<StarknetChain>
    + CanBuildUpdateClientMessage<StarknetChain>
    + CanQueryConsensusStateHeight<StarknetChain>
    + CanBuildCreateClientPayload<StarknetChain>
{
}

impl CanUseCosmosChainWithStarknet for CosmosChain {}
