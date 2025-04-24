use core::ops::Deref;
use core::time::Duration;
use std::sync::{Arc, OnceLock};

use cgp::core::component::UseDelegate;
use cgp::core::error::{ErrorRaiserComponent, ErrorTypeProviderComponent, ErrorWrapperComponent};
use cgp::core::field::WithField;
use cgp::prelude::*;
use futures::lock::Mutex;
use hermes_cairo_encoding_components::types::as_felt::AsFelt;
use hermes_cairo_encoding_components::types::as_starknet_event::AsStarknetEvent;
use hermes_chain_components::traits::queries::block_time::BlockTimeQuerierComponent;
use hermes_chain_components::traits::queries::chain_status::ChainStatusQuerierComponent;
use hermes_chain_components::traits::send_message::MessageSenderComponent;
use hermes_chain_components::traits::types::poll_interval::PollIntervalGetterComponent;
use hermes_chain_type_components::traits::fields::chain_id::ChainIdGetterComponent;
use hermes_cosmos_chain_components::types::key_types::secp256k1::Secp256k1KeyPair;
use hermes_encoding_components::traits::has_encoding::{
    DefaultEncodingGetter, DefaultEncodingGetterComponent, EncodingGetter, EncodingGetterComponent,
    EncodingTypeProviderComponent,
};
use hermes_encoding_components::types::AsBytes;
use hermes_error::impls::UseHermesError;
use hermes_logging_components::traits::logger::LoggerComponent;
use hermes_relayer_components::transaction::impls::global_nonce_mutex::GetGlobalNonceMutex;
use hermes_relayer_components::transaction::traits::default_signer::DefaultSignerGetterComponent;
use hermes_relayer_components::transaction::traits::nonce::nonce_mutex::NonceAllocationMutexGetterComponent;
use hermes_relayer_components::transaction::traits::nonce::query_nonce::NonceQuerierComponent;
use hermes_runtime::types::runtime::HermesRuntime;
use hermes_runtime_components::traits::runtime::{
    RuntimeGetterComponent, RuntimeTypeProviderComponent,
};
use hermes_starknet_chain_components::impls::commitment_proof::{
    VerifyStarknetMerkleProof, VerifyStarknetStorageProof,
};
use hermes_starknet_chain_components::impls::json_rpc::SendJsonRpcRequestWithReqwest;
use hermes_starknet_chain_components::impls::queries::storage_proof::QueryStarknetStorageProof;
use hermes_starknet_chain_components::impls::types::address::StarknetAddress;
use hermes_starknet_chain_components::traits::account::{
    AccountFromSignerBuilderComponent, StarknetAccountTypeProviderComponent,
};
use hermes_starknet_chain_components::traits::client::{
    StarknetClientGetterComponent, StarknetClientTypeProviderComponent,
};
use hermes_starknet_chain_components::traits::commitment_proof::{
    StarknetMerkleProofVerifierComponent, StarknetStorageProofVerifierComponent,
};
use hermes_starknet_chain_components::traits::contract::call::ContractCallerComponent;
use hermes_starknet_chain_components::traits::contract::declare::ContractDeclarerComponent;
use hermes_starknet_chain_components::traits::contract::deploy::ContractDeployerComponent;
use hermes_starknet_chain_components::traits::contract::invoke::ContractInvokerComponent;
use hermes_starknet_chain_components::traits::contract::message::InvokeContractMessageBuilderComponent;
use hermes_starknet_chain_components::traits::json_rpc::JsonRpcRequestSenderComponent;
use hermes_starknet_chain_components::traits::proof_signer::{
    StarknetProofSignerGetterComponent, StarknetProofSignerTypeProviderComponent,
};
use hermes_starknet_chain_components::traits::queries::storage_proof::StorageProofQuerierComponent;
use hermes_starknet_chain_components::traits::rpc_client::{
    JsonRpcUrlGetterComponent, ReqwestClientGetterComponent,
};
use hermes_starknet_chain_components::traits::types::commitment::MerkleProofTypeProviderComponent;
use hermes_starknet_chain_components::traits::types::storage_proof::{
    StorageKeyTypeProviderComponent, StorageProofTypeProviderComponent,
};
use hermes_starknet_chain_components::types::wallet::StarknetWallet;
use hermes_starknet_chain_context::contexts::encoding::cairo::UseStarknetCairoEncoding;
use hermes_starknet_chain_context::contexts::encoding::event::StarknetEventEncoding;
use hermes_starknet_chain_context::contexts::encoding::protobuf::StarknetProtobufEncoding;
use hermes_tracing_logging_components::contexts::logger::TracingLogger;
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
