use core::ops::Deref;
use core::time::Duration;
use std::sync::{Arc, OnceLock};

use cgp::core::component::UseDelegate;
use cgp::core::error::{ErrorRaiserComponent, ErrorTypeProviderComponent, ErrorWrapperComponent};
use cgp::core::field::WithField;
use futures::lock::Mutex;
use hermes_cairo_encoding_components::types::as_felt::AsFelt;
use hermes_cairo_encoding_components::types::as_starknet_event::AsStarknetEvent;
use hermes_core::chain_components::traits::{
    BlockTimeQuerierComponent, ChainStatusQuerierComponent, MessageSenderComponent,
    PollIntervalGetterComponent,
};
use hermes_core::chain_type_components::traits::ChainIdGetterComponent;
use hermes_core::encoding_components::traits::{
    DefaultEncodingGetter, DefaultEncodingGetterComponent, EncodingGetter, EncodingGetterComponent,
    EncodingTypeProviderComponent,
};
use hermes_core::encoding_components::types::AsBytes;
use hermes_core::logging_components::traits::LoggerComponent;
use hermes_core::relayer_components::transaction::impls::{
    GetGlobalNonceMutex, GetGlobalSignerMutex, SignerWithIndexGetter,
};
use hermes_core::relayer_components::transaction::traits::{
    ClientRefreshRateGetterComponent, DefaultSignerGetterComponent,
    NonceAllocationMutexGetterComponent, NonceQuerierComponent, SignerGetterComponent,
    SignerMutexGetterComponent,
};
use hermes_core::runtime_components::traits::{
    RuntimeGetterComponent, RuntimeTypeProviderComponent,
};
use hermes_cosmos::chain_components::impls::GetFirstSignerAsDefault;
use hermes_cosmos::chain_preset::delegate::DelegateCosmosChainComponents;
use hermes_cosmos::error::impls::UseHermesError;
use hermes_cosmos::runtime::types::runtime::HermesRuntime;
use hermes_cosmos::tracing_logging_components::contexts::TracingLogger;
use hermes_prelude::*;
use hermes_starknet_chain_components::components::{
    StarknetChainComponents, StarknetToCosmosComponents,
};
use hermes_starknet_chain_components::impls::{
    GetStarknetClientRefreshRate, QueryStarknetStorageProof, SendJsonRpcRequestWithReqwest,
    StarknetAddress, VerifyStarknetMerkleProof, VerifyStarknetStorageProof,
};
use hermes_starknet_chain_components::traits::{
    AccountFromSignerBuilderComponent, ContractCallerComponent, ContractDeclarerComponent,
    ContractDeployerComponent, ContractInvokerComponent, Ed25519AttestatorAddressesGetterComponent,
    FeederGatewayUrlGetterComponent, InvokeContractMessageBuilderComponent,
    JsonRpcRequestSenderComponent, JsonRpcUrlGetterComponent, MerkleProofTypeProviderComponent,
    ReqwestClientGetterComponent, StarknetAccountTypeProviderComponent,
    StarknetClientGetterComponent, StarknetClientTypeProviderComponent,
    StarknetMerkleProofVerifierComponent, StarknetStorageProofVerifierComponent,
    StorageKeyTypeProviderComponent, StorageProofQuerierComponent,
    StorageProofTypeProviderComponent,
};
use hermes_starknet_chain_components::types::StarknetWallet;
use ibc::core::host::types::identifiers::ChainId;
use indexmap::IndexMap;
use starknet::core::types::{Felt, MerkleNode, StorageProof};
use starknet::providers::jsonrpc::HttpTransport;
use starknet::providers::JsonRpcClient;
use ureq::Agent;
use url::Url;

use crate::contexts::{StarknetEventEncoding, StarknetProtobufEncoding, UseStarknetCairoEncoding};
use crate::impls::{BuildStarknetAccount, HandleStarknetChainError};
use crate::types::StarknetAccount;

#[cgp_context(StarknetChainContextComponents: StarknetChainComponents)]
#[derive(Clone)]
pub struct StarknetChain {
    pub fields: Arc<StarknetChainFields>,
}

#[derive(HasField, Clone)]
pub struct StarknetChainFields {
    pub runtime: HermesRuntime,
    pub chain_id: ChainId,
    pub starknet_client: Arc<JsonRpcClient<HttpTransport>>,
    pub rpc_client: Agent,
    pub json_rpc_url: Url,
    pub feeder_gateway_url: Url,
    pub ibc_client_contract_address: OnceLock<StarknetAddress>,
    pub ibc_core_contract_address: OnceLock<StarknetAddress>,
    pub ibc_ics20_contract_address: OnceLock<StarknetAddress>,
    pub event_encoding: StarknetEventEncoding,
    pub poll_interval: Duration,
    pub block_time: Duration,
    pub nonce_mutex: Arc<Mutex<()>>,
    pub signers: Vec<StarknetWallet>,
    pub client_refresh_rate: Option<Duration>,
    pub signer_mutex: Arc<Mutex<usize>>,
    pub ed25519_attestator_addresses: Option<Vec<String>>,
}

impl Deref for StarknetChain {
    type Target = StarknetChainFields;

    fn deref(&self) -> &StarknetChainFields {
        &self.fields
    }
}

delegate_components! {
    StarknetChainContextComponents {
        [
            ErrorTypeProviderComponent,
            ErrorWrapperComponent,
        ]: UseHermesError,
        ErrorRaiserComponent:
            UseDelegate<HandleStarknetChainError>,
        RuntimeTypeProviderComponent:
            UseType<HermesRuntime>,
        [
            EncodingTypeProviderComponent<AsFelt>,
            EncodingGetterComponent<AsFelt>,
            DefaultEncodingGetterComponent<AsFelt>,
        ]:
            UseStarknetCairoEncoding,
        EncodingTypeProviderComponent<AsStarknetEvent>:
            UseType<StarknetEventEncoding>,
        EncodingTypeProviderComponent<AsBytes>:
            UseType<StarknetProtobufEncoding>,
        ChainIdGetterComponent:
            UseField<symbol!("chain_id")>,
        RuntimeGetterComponent:
            UseField<symbol!("runtime")>,
        PollIntervalGetterComponent:
            UseField<symbol!("poll_interval")>,
        ReqwestClientGetterComponent:
            UseField<symbol!("rpc_client")>,
        FeederGatewayUrlGetterComponent:
            UseField<symbol!("feeder_gateway_url")>,
        JsonRpcUrlGetterComponent:
            UseField<symbol!("json_rpc_url")>,
        LoggerComponent:
            TracingLogger,
        [
            StarknetClientTypeProviderComponent,
            StarknetClientGetterComponent,
        ]:
            WithField<symbol!("starknet_client")>,
        Ed25519AttestatorAddressesGetterComponent:
            UseField<symbol!("ed25519_attestator_addresses")>,
        DefaultSignerGetterComponent:
            GetFirstSignerAsDefault<symbol!("signers")>,
        SignerMutexGetterComponent:
            GetGlobalSignerMutex<symbol!("signer_mutex"), symbol!("signers")>,
        SignerGetterComponent:
            SignerWithIndexGetter<symbol!("signers")>,
        NonceAllocationMutexGetterComponent:
            GetGlobalNonceMutex<symbol!("nonce_mutex")>,
        ClientRefreshRateGetterComponent:
            GetStarknetClientRefreshRate,
        BlockTimeQuerierComponent:
            UseField<symbol!("block_time")>,
        StarknetAccountTypeProviderComponent:
            UseType<StarknetAccount>,
        AccountFromSignerBuilderComponent:
            BuildStarknetAccount,
        JsonRpcRequestSenderComponent:
            SendJsonRpcRequestWithReqwest,
        StorageKeyTypeProviderComponent:
            UseType<Felt>,
        StorageProofTypeProviderComponent:
            UseType<StorageProof>,
        StorageProofQuerierComponent:
            QueryStarknetStorageProof,
        MerkleProofTypeProviderComponent:
            UseType<IndexMap<Felt, MerkleNode>>,
        StarknetMerkleProofVerifierComponent:
            VerifyStarknetMerkleProof,
        StarknetStorageProofVerifierComponent:
            VerifyStarknetStorageProof,
    }
}

delegate_components! {
    DelegateCosmosChainComponents {
        StarknetChain: StarknetToCosmosComponents::Provider,
    }
}

#[cgp_provider(EncodingGetterComponent<AsStarknetEvent>)]
impl EncodingGetter<StarknetChain, AsStarknetEvent> for StarknetChainContextComponents {
    fn encoding(chain: &StarknetChain) -> &StarknetEventEncoding {
        &chain.event_encoding
    }
}

#[cgp_provider(DefaultEncodingGetterComponent<AsBytes>)]
impl DefaultEncodingGetter<StarknetChain, AsBytes> for StarknetChainContextComponents {
    fn default_encoding() -> &'static StarknetProtobufEncoding {
        &StarknetProtobufEncoding
    }
}

check_components! {
    CanUseStarknetChain for StarknetChain {
        ContractCallerComponent,
        ContractDeclarerComponent,
        ContractDeployerComponent,
        ContractInvokerComponent,
        InvokeContractMessageBuilderComponent,
        MessageSenderComponent,
        NonceQuerierComponent,
        ChainStatusQuerierComponent,
        StorageProofQuerierComponent,
        StarknetMerkleProofVerifierComponent,
        StarknetStorageProofVerifierComponent,
    }
}
