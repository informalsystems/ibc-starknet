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
use hermes_chain_components::traits::types::poll_interval::PollIntervalGetterComponent;
use hermes_chain_type_components::traits::fields::chain_id::ChainIdGetterComponent;
use hermes_cosmos_chain_components::types::key_types::secp256k1::Secp256k1KeyPair;
use hermes_cosmos_chain_preset::delegate::DelegateCosmosChainComponents;
use hermes_encoding_components::traits::has_encoding::{
    DefaultEncodingGetter, DefaultEncodingGetterComponent, EncodingGetter, EncodingGetterComponent,
    EncodingTypeProviderComponent,
};
use hermes_encoding_components::types::AsBytes;
use hermes_error::impls::UseHermesError;
use hermes_logger::UseHermesLogger;
use hermes_logging_components::traits::has_logger::{
    GlobalLoggerGetterComponent, LoggerGetterComponent, LoggerTypeProviderComponent,
};
use hermes_relayer_components::transaction::impls::global_nonce_mutex::GetGlobalNonceMutex;
use hermes_relayer_components::transaction::traits::default_signer::DefaultSignerGetterComponent;
use hermes_relayer_components::transaction::traits::nonce::nonce_mutex::NonceAllocationMutexGetterComponent;
use hermes_runtime::types::runtime::HermesRuntime;
use hermes_runtime_components::traits::runtime::{
    RuntimeGetterComponent, RuntimeTypeProviderComponent,
};
use hermes_starknet_chain_components::components::starknet_to_cosmos::StarknetToCosmosComponents;
use hermes_starknet_chain_components::impls::types::address::StarknetAddress;
use hermes_starknet_chain_components::traits::account::{
    AccountFromSignerBuilderComponent, StarknetAccountTypeProviderComponent,
};
use hermes_starknet_chain_components::traits::client::{
    StarknetClientGetterComponent, StarknetClientTypeProviderComponent,
};
use hermes_starknet_chain_components::traits::proof_signer::{
    StarknetProofSignerGetterComponent, StarknetProofSignerTypeProviderComponent,
};
use hermes_starknet_chain_components::types::wallet::StarknetWallet;
use hermes_starknet_chain_context::contexts::encoding::cairo::UseStarknetCairoEncoding;
use hermes_starknet_chain_context::contexts::encoding::event::StarknetEventEncoding;
use hermes_starknet_chain_context::contexts::encoding::protobuf::StarknetProtobufEncoding;
use hermes_starknet_chain_context::impls::error::HandleStarknetChainError;
use ibc::core::host::types::identifiers::ChainId;
use starknet_v13::providers::jsonrpc::HttpTransport;
use starknet_v13::providers::JsonRpcClient;

use crate::impls::BuildStarknetAccount;
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
        [
            LoggerTypeProviderComponent,
            LoggerGetterComponent,
            GlobalLoggerGetterComponent,
        ]:
            UseHermesLogger,
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
    }
}

delegate_components! {
    DelegateCosmosChainComponents {
        MadaraChain: StarknetToCosmosComponents::Provider,
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
