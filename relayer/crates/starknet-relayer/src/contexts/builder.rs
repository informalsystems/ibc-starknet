use alloc::sync::Arc;
use core::marker::PhantomData;
use core::ops::Deref;

use cgp::core::component::UseDelegate;
use cgp::core::error::{ErrorRaiserComponent, ErrorTypeProviderComponent};
use cgp::core::field::{Index, WithField};
use cgp::core::types::WithType;
use cgp::prelude::*;
use eyre::eyre;
use futures::lock::Mutex;
use hermes_cosmos_chain_components::types::key_types::secp256k1::Secp256k1KeyPair;
use hermes_cosmos_relayer::contexts::build::CosmosBuilder;
use hermes_cosmos_relayer::contexts::chain::CosmosChain;
use hermes_error::impls::ProvideHermesError;
use hermes_error::types::Error;
use hermes_error::HermesError;
use hermes_relayer_components::build::traits::builders::birelay_builder::{
    BiRelayBuilder, BiRelayBuilderComponent,
};
use hermes_relayer_components::build::traits::builders::chain_builder::{
    CanBuildChain, ChainBuilder, ChainBuilderComponent,
};
use hermes_relayer_components::multi::traits::birelay_at::BiRelayTypeAtComponent;
use hermes_relayer_components::multi::traits::chain_at::ChainTypeAtComponent;
use hermes_relayer_components::multi::traits::relay_at::RelayTypeAtComponent;
use hermes_runtime::types::runtime::HermesRuntime;
use hermes_runtime_components::traits::runtime::{
    RuntimeGetterComponent, RuntimeTypeProviderComponent,
};
use hermes_starknet_chain_components::impls::types::config::StarknetChainConfig;
use hermes_starknet_chain_components::types::client_id::ClientId as StarknetClientId;
use hermes_starknet_chain_context::contexts::chain::{StarknetChain, StarknetChainFields};
use hermes_starknet_chain_context::impls::error::HandleStarknetChainError;
use ibc::core::host::types::identifiers::{ChainId, ClientId, ClientId as CosmosClientId};
use starknet::accounts::{ExecutionEncoding, SingleOwnerAccount};
use starknet::providers::jsonrpc::HttpTransport;
use starknet::providers::{JsonRpcClient, Provider};
use starknet::signers::{LocalWallet, SigningKey};
use url::Url;

use super::cosmos_to_starknet_relay::CosmosToStarknetRelay;
use crate::contexts::cosmos_starknet_birelay::CosmosStarknetBiRelay;
use crate::contexts::starknet_cosmos_birelay::StarknetCosmosBiRelay;
use crate::contexts::starknet_to_cosmos_relay::StarknetToCosmosRelay;

#[cgp_context(StarknetBuildComponents)]
#[derive(Clone)]
pub struct StarknetBuilder {
    pub fields: Arc<dyn HasStarknetBuilderFields>,
}

#[derive(HasField)]
pub struct StarknetBuilderFields {
    // Used to build CosmosChain
    pub cosmos_builder: CosmosBuilder,
    pub runtime: HermesRuntime,
    // Fields for StarknetChain
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

delegate_components! {
    StarknetBuildComponents {
        ErrorTypeProviderComponent: ProvideHermesError,
        ErrorRaiserComponent: UseDelegate<HandleStarknetChainError>,
        ChainTypeAtComponent<Index<0>>: WithType<StarknetChain>,
        ChainTypeAtComponent<Index<1>>: WithType<CosmosChain>,
        RuntimeTypeProviderComponent: WithType<HermesRuntime>,
        RuntimeGetterComponent: WithField<symbol!("runtime")>,
        RelayTypeAtComponent<Index<0>, Index<1>>: WithType<StarknetToCosmosRelay>,
        RelayTypeAtComponent<Index<1>, Index<0>>: WithType<CosmosToStarknetRelay>,
        BiRelayTypeAtComponent<Index<0>, Index<1>>: WithType<StarknetCosmosBiRelay>,
        BiRelayTypeAtComponent<Index<1>, Index<0>>: WithType<CosmosStarknetBiRelay>,
    }
}

#[cgp_provider(ChainBuilderComponent)]
impl ChainBuilder<StarknetBuilder, Index<0>> for StarknetBuildComponents {
    async fn build_chain(
        build: &StarknetBuilder,
        _index: PhantomData<Index<0>>,
        chain_id: &ChainId,
    ) -> Result<StarknetChain, HermesError> {
        build.build_chain(chain_id).await
    }
}

#[cgp_provider(ChainBuilderComponent)]
impl ChainBuilder<StarknetBuilder, Index<1>> for StarknetBuildComponents {
    async fn build_chain(
        build: &StarknetBuilder,
        _index: PhantomData<Index<1>>,
        chain_id: &ChainId,
    ) -> Result<CosmosChain, Error> {
        build.cosmos_builder.build_chain(chain_id).await
    }
}

#[cgp_provider(BiRelayBuilderComponent)]
impl BiRelayBuilder<StarknetBuilder, Index<0>, Index<1>> for StarknetBuildComponents {
    async fn build_birelay(
        build: &StarknetBuilder,
        chain_id_a: &ChainId,
        chain_id_b: &ChainId,
        client_id_a: &ClientId,
        client_id_b: &ClientId,
    ) -> Result<StarknetCosmosBiRelay, HermesError> {
        let starknet_chain = build.build_chain(chain_id_a).await?;
        let cosmos_chain = build.cosmos_builder.build_chain(chain_id_b).await?;

        let relay_a_to_b = StarknetToCosmosRelay::new(
            build.runtime.clone(),
            starknet_chain.clone(),
            cosmos_chain.clone(),
            client_id_a.clone(),
            client_id_b.clone(),
        );

        let relay_b_to_a = CosmosToStarknetRelay::new(
            build.runtime.clone(),
            cosmos_chain,
            starknet_chain,
            client_id_b.clone(),
            client_id_a.clone(),
        );

        let birelay = StarknetCosmosBiRelay {
            runtime: build.runtime.clone(),
            relay_a_to_b,
            relay_b_to_a,
        };

        Ok(birelay)
    }
}

#[cgp_provider(BiRelayBuilderComponent)]
impl BiRelayBuilder<StarknetBuilder, Index<1>, Index<0>> for StarknetBuildComponents {
    async fn build_birelay(
        build: &StarknetBuilder,
        chain_id_a: &ChainId,
        chain_id_b: &ChainId,
        client_id_a: &ClientId,
        client_id_b: &ClientId,
    ) -> Result<CosmosStarknetBiRelay, HermesError> {
        let starknet_chain = build.build_chain(chain_id_a).await?;
        let cosmos_chain = build.cosmos_builder.build_chain(chain_id_b).await?;

        let relay_a_to_b = CosmosToStarknetRelay::new(
            build.runtime.clone(),
            cosmos_chain.clone(),
            starknet_chain.clone(),
            client_id_b.clone(),
            client_id_a.clone(),
        );

        let relay_b_to_a = StarknetToCosmosRelay::new(
            build.runtime.clone(),
            starknet_chain,
            cosmos_chain,
            client_id_a.clone(),
            client_id_b.clone(),
        );

        let birelay = CosmosStarknetBiRelay {
            runtime: build.runtime.clone(),
            relay_a_to_b,
            relay_b_to_a,
        };

        Ok(birelay)
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

    pub async fn build_chain(&self, chain_id: &ChainId) -> Result<StarknetChain, HermesError> {
        self.build_chain_with_config(chain_id).await
    }

    pub async fn build_chain_with_config(
        &self,
        expected_chain_id: &ChainId,
    ) -> Result<StarknetChain, HermesError> {
        let json_rpc_url = Url::parse(&self.starknet_chain_config.json_rpc_url)?;

        let rpc_client = Arc::new(JsonRpcClient::new(HttpTransport::new(json_rpc_url)));

        let chain_id_felt = rpc_client.chain_id().await?;

        let chain_id = chain_id_felt.to_string().parse()?;

        if &chain_id != expected_chain_id {
            return Err(eyre!("Starknet chain has a different ID as configured. Expected: {expected_chain_id}, got: {chain_id}").into());
        }

        let account = SingleOwnerAccount::new(
            rpc_client.clone(),
            LocalWallet::from_signing_key(SigningKey::from_secret_scalar(
                self.starknet_chain_config.relayer_wallet.signing_key,
            )),
            *self.starknet_chain_config.relayer_wallet.account_address,
            chain_id_felt,
            ExecutionEncoding::New,
        );

        let proof_signer = Secp256k1KeyPair::from_mnemonic(
            bip39::Mnemonic::from_entropy(
                &self
                    .starknet_chain_config
                    .relayer_wallet
                    .signing_key
                    .to_bytes_be(),
                bip39::Language::English,
            )
            .expect("valid mnemonic")
            .phrase(),
            &"m/84'/0'/0'/0/0".parse().expect("valid hdpath"),
            "strk",
        )
        .expect("valid key pair");

        let context = StarknetChain {
            fields: Arc::new(StarknetChainFields {
                runtime: self.runtime.clone(),
                chain_id,
                rpc_client,
                account: Arc::new(account),
                ibc_client_contract_address: None,
                ibc_core_contract_address: None,
                event_encoding: Default::default(),
                proof_signer,
                poll_interval: self.starknet_chain_config.poll_interval,
                nonce_mutex: Arc::new(Mutex::new(())),
            }),
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
