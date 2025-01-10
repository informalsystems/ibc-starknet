use alloc::sync::Arc;
use core::marker::PhantomData;
use core::ops::Deref;

use cgp::core::component::UseDelegate;
use cgp::core::error::{ErrorRaiserComponent, ErrorTypeComponent};
use cgp::core::field::impls::use_field::WithField;
use cgp::core::types::impls::WithType;
use cgp::prelude::*;
use hermes_cosmos_relayer::contexts::build::CosmosBuilder;
use hermes_cosmos_relayer::contexts::chain::CosmosChain;
use hermes_error::impls::ProvideHermesError;
use hermes_error::types::Error;
use hermes_error::HermesError;
use hermes_relayer_components::build::traits::builders::chain_builder::{
    CanBuildChain, ChainBuilder,
};
use hermes_relayer_components::multi::traits::chain_at::ChainTypeAtComponent;
use hermes_relayer_components::multi::traits::relay_at::RelayTypeAtComponent;
use hermes_relayer_components::multi::types::index::Index;
use hermes_runtime::types::runtime::HermesRuntime;
use hermes_runtime_components::traits::runtime::{RuntimeGetterComponent, RuntimeTypeComponent};
use hermes_starknet_chain_components::impls::types::config::StarknetChainConfig;
use hermes_starknet_chain_components::types::client_id::ClientId as StarknetClientId;
use hermes_starknet_chain_context::contexts::chain::StarknetChain;
use hermes_starknet_chain_context::impls::error::HandleStarknetChainError;
use ibc::core::host::types::identifiers::{ChainId, ClientId as CosmosClientId};
use starknet::accounts::{ExecutionEncoding, SingleOwnerAccount};
use starknet::providers::jsonrpc::HttpTransport;
use starknet::providers::{JsonRpcClient, Provider};
use starknet::signers::{LocalWallet, SigningKey};
use url::Url;

use super::cosmos_to_starknet_relay::CosmosToStarknetRelay;
use crate::contexts::starknet_to_cosmos_relay::StarknetToCosmosRelay;

#[derive(Clone)]
pub struct StarknetBuilder {
    pub fields: Arc<dyn HasStarknetBuilderFields>,
}

#[derive(HasField)]
pub struct StarknetBuilderFields {
    // Used to build CosmosChain
    pub cosmos_builder: CosmosBuilder,
    // Fields for StarknetChain
    pub runtime: HermesRuntime,
    pub starknet_chain_config: StarknetChainConfig,
}

impl Deref for StarknetBuilder {
    type Target = StarknetBuilderFields;

    fn deref(&self) -> &Self::Target {
        self.fields.fields()
    }
}

pub trait HasStarknetBuilderFields: Send + Sync + 'static {
    fn fields(&self) -> &StarknetBuilderFields;
}

impl HasStarknetBuilderFields for StarknetBuilderFields {
    fn fields(&self) -> &StarknetBuilderFields {
        self
    }
}

pub struct StarknetBuildComponents;

impl HasComponents for StarknetBuilder {
    type Components = StarknetBuildComponents;
}

delegate_components! {
    StarknetBuildComponents {
        ErrorTypeComponent: ProvideHermesError,
        ErrorRaiserComponent: UseDelegate<HandleStarknetChainError>,
        ChainTypeAtComponent<Index<0>>: WithType<StarknetChain>,
        ChainTypeAtComponent<Index<1>>: WithType<CosmosChain>,
        RuntimeTypeComponent: WithType<HermesRuntime>,
        RuntimeGetterComponent: WithField<symbol!("runtime")>,
        RelayTypeAtComponent<Index<0>, Index<1>>: WithType<StarknetToCosmosRelay>,
    }
}

impl ChainBuilder<StarknetBuilder, Index<0>> for StarknetBuildComponents {
    async fn build_chain(
        build: &StarknetBuilder,
        _index: PhantomData<Index<0>>,
        _chain_id: &String,
    ) -> Result<StarknetChain, HermesError> {
        build.build_chain().await
    }
}

impl ChainBuilder<StarknetBuilder, Index<1>> for StarknetBuildComponents {
    async fn build_chain(
        build: &StarknetBuilder,
        _index: PhantomData<Index<1>>,
        chain_id: &ChainId,
    ) -> Result<CosmosChain, Error> {
        build.cosmos_builder.build_chain(chain_id).await
    }
}

impl StarknetBuilder {
    pub fn new(
        cosmos_builder: CosmosBuilder,
        runtime: HermesRuntime,
        starknet_chain_config: StarknetChainConfig,
    ) -> Self {
        Self {
            fields: Arc::new(StarknetBuilderFields {
                cosmos_builder,
                runtime,
                starknet_chain_config,
            }),
        }
    }

    pub async fn build_chain(&self) -> Result<StarknetChain, HermesError> {
        self.build_chain_with_config().await
    }

    pub async fn build_chain_with_config(&self) -> Result<StarknetChain, HermesError> {
        let json_rpc_url = Url::parse(&self.starknet_chain_config.json_rpc_url)?;

        let rpc_client = Arc::new(JsonRpcClient::new(HttpTransport::new(json_rpc_url)));

        let chain_id = rpc_client.chain_id().await?;

        let account = SingleOwnerAccount::new(
            rpc_client.clone(),
            LocalWallet::from_signing_key(SigningKey::from_secret_scalar(
                self.starknet_chain_config.relayer_wallet.signing_key,
            )),
            self.starknet_chain_config.relayer_wallet.account_address,
            chain_id,
            ExecutionEncoding::New,
        );

        let context = StarknetChain {
            runtime: self.runtime.clone(),
            chain_id: chain_id.to_string(),
            rpc_client,
            account,
            ibc_client_contract_address: None,
            ibc_core_contract_address: None,
        };

        Ok(context)
    }

    pub fn build_starknet_to_cosmos_relay(
        &self,
        src_chain: StarknetChain,
        dst_chain: CosmosChain,
        src_client_id: &StarknetClientId,
        dst_client_id: &CosmosClientId,
    ) -> StarknetToCosmosRelay {
        StarknetToCosmosRelay::new(
            self.runtime.clone(),
            src_chain,
            dst_chain,
            src_client_id.clone(),
            dst_client_id.clone(),
        )
    }

    pub fn build_cosmos_to_starknet_relay(
        &self,
        src_chain: CosmosChain,
        dst_chain: StarknetChain,
        src_client_id: &CosmosClientId,
        dst_client_id: &StarknetClientId,
    ) -> CosmosToStarknetRelay {
        CosmosToStarknetRelay::new(
            self.runtime.clone(),
            src_chain,
            dst_chain,
            src_client_id.clone(),
            dst_client_id.clone(),
        )
    }
}

pub trait CanUseStarknetBuilder: CanBuildChain<Index<0>> + CanBuildChain<Index<1>> {}

impl CanUseStarknetBuilder for StarknetBuilder {}
