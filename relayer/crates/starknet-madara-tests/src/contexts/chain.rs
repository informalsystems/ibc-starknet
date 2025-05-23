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
use hermes_core::relayer_components::transaction::impls::GetGlobalNonceMutex;
use hermes_core::relayer_components::transaction::traits::{
    DefaultSignerGetterComponent, NonceAllocationMutexGetterComponent, NonceQuerierComponent,
};
use hermes_core::runtime_components::traits::{
    RuntimeGetterComponent, RuntimeTypeProviderComponent,
};
use hermes_cosmos_core::chain_components::types::Secp256k1KeyPair;
use hermes_cosmos_core::tracing_logging_components::contexts::TracingLogger;
use hermes_error::impls::UseHermesError;
use hermes_prelude::*;
use hermes_runtime::types::runtime::HermesRuntime;
use hermes_starknet_chain_components::impls::{
    QueryStarknetStorageProof, SendJsonRpcRequestWithReqwest, StarknetAddress,
    VerifyStarknetMerkleProof, VerifyStarknetStorageProof,
};
use hermes_starknet_chain_components::traits::{
    AccountFromSignerBuilderComponent, ContractCallerComponent, ContractDeclarerComponent,
    ContractDeployerComponent, ContractInvokerComponent, InvokeContractMessageBuilderComponent,
    JsonRpcRequestSenderComponent, JsonRpcUrlGetterComponent, MerkleProofTypeProviderComponent,
    ReqwestClientGetterComponent, StarknetAccountTypeProviderComponent,
    StarknetClientGetterComponent, StarknetClientTypeProviderComponent,
    StarknetMerkleProofVerifierComponent, StarknetProofSignerGetterComponent,
    StarknetProofSignerTypeProviderComponent, StarknetStorageProofVerifierComponent,
    StorageKeyTypeProviderComponent, StorageProofQuerierComponent,
    StorageProofTypeProviderComponent,
};
use hermes_starknet_chain_components::types::StarknetWallet;
use hermes_starknet_chain_context::contexts::{
    StarknetEventEncoding, StarknetProtobufEncoding, UseStarknetCairoEncoding,
};
use ibc::core::host::types::identifiers::ChainId;
use indexmap::IndexMap;
use reqwest::Client;
use starknet::core::types::{Felt, MerkleNode, StorageProof};
use starknet_v13::providers::jsonrpc::HttpTransport;
use starknet_v13::providers::JsonRpcClient;
use url::Url;

use crate::impls::{BuildStarknetAccount, HandleMadaraChainError};
use crate::presets::MadaraChainPreset;
use crate::types::StarknetAccount;

#[cgp_context(MadaraChainComponents: MadaraChainPreset)]
#[derive(Clone)]
pub struct MadaraChain {
    pub fields: Arc<MadaraChainFields>,
}

#[derive(HasField, Clone)]
pub struct MadaraChainFields {
    pub runtime: HermesRuntime,
    pub chain_id: ChainId,
    pub starknet_client: Arc<JsonRpcClient<HttpTransport>>,
    pub rpc_client: Client,
    pub json_rpc_url: Url,
    pub ibc_client_contract_address: OnceLock<StarknetAddress>,
    pub ibc_core_contract_address: OnceLock<StarknetAddress>,
    pub ibc_ics20_contract_address: OnceLock<StarknetAddress>,
    pub event_encoding: StarknetEventEncoding,
    pub poll_interval: Duration,
    pub block_time: Duration,
    pub proof_signer: Secp256k1KeyPair,
    pub nonce_mutex: Arc<Mutex<()>>,
    pub signer: StarknetWallet,
}

impl Deref for MadaraChain {
    type Target = MadaraChainFields;

    fn deref(&self) -> &MadaraChainFields {
        &self.fields
    }
}

delegate_components! {
    MadaraChainComponents {
        [
            ErrorTypeProviderComponent,
            ErrorWrapperComponent,
        ]: UseHermesError,
        ErrorRaiserComponent:
            UseDelegate<HandleMadaraChainError>,
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
        JsonRpcUrlGetterComponent:
            UseField<symbol!("json_rpc_url")>,
        LoggerComponent:
            TracingLogger,
        [
            StarknetClientTypeProviderComponent,
            StarknetClientGetterComponent,
        ]:
            WithField<symbol!("starknet_client")>,
        StarknetProofSignerTypeProviderComponent:
            UseType<Secp256k1KeyPair>,
        StarknetProofSignerGetterComponent:
            UseField<symbol!("proof_signer")>,
        DefaultSignerGetterComponent:
            UseField<symbol!("signer")>,
        NonceAllocationMutexGetterComponent:
            GetGlobalNonceMutex<symbol!("nonce_mutex")>,
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

#[cgp_provider(EncodingGetterComponent<AsStarknetEvent>)]
impl EncodingGetter<MadaraChain, AsStarknetEvent> for MadaraChainComponents {
    fn encoding(chain: &MadaraChain) -> &StarknetEventEncoding {
        &chain.event_encoding
    }
}

#[cgp_provider(DefaultEncodingGetterComponent<AsBytes>)]
impl DefaultEncodingGetter<MadaraChain, AsBytes> for MadaraChainComponents {
    fn default_encoding() -> &'static StarknetProtobufEncoding {
        &StarknetProtobufEncoding
    }
}

check_components! {
    CanUseMadaraChain for MadaraChain {
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
