use alloc::sync::Arc;
use core::ops::Deref;
use std::marker::PhantomData;

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
use hermes_relayer_components::multi::types::index::Index;
use hermes_runtime::types::runtime::HermesRuntime;
use hermes_runtime_components::traits::runtime::{RuntimeGetterComponent, RuntimeTypeComponent};
use hermes_starknet_chain_components::impls::types::config::StarknetChainConfig;
use hermes_starknet_chain_context::contexts::chain::StarknetChain;
use hermes_starknet_chain_context::impls::error::HandleStarknetChainError;
use ibc::core::host::types::identifiers::ChainId;
use starknet::accounts::{ExecutionEncoding, SingleOwnerAccount};
use starknet::core::types::Felt;
use starknet::providers::jsonrpc::HttpTransport;
use starknet::providers::{JsonRpcClient, Provider};
use starknet::signers::{LocalWallet, SigningKey};
use url::Url;

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
    }
}

impl ChainBuilder<StarknetBuilder, Index<0>> for StarknetBuildComponents {
    async fn build_chain(
        build: &StarknetBuilder,
        _index: PhantomData<Index<0>>,
        _chain_id: &Felt,
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
            chain_id,
            rpc_client,
            account,
            ibc_client_contract_address: None,
            ibc_core_contract_address: None,
        };

        Ok(context)
    }
}

pub trait CanUseStarknetBuilder: CanBuildChain<Index<0>> + CanBuildChain<Index<1>> {}

impl CanUseStarknetBuilder for StarknetBuilder {}
