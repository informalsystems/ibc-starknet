use alloc::sync::Arc;
use core::marker::PhantomData;
use core::ops::Deref;
use std::collections::HashSet;
use std::path::PathBuf;
use std::sync::OnceLock;

use cgp::core::component::UseDelegate;
use cgp::core::error::{ErrorRaiserComponent, ErrorTypeProviderComponent};
use cgp::core::field::{Index, WithField};
use cgp::core::types::WithType;
use eyre::eyre;
use futures::lock::Mutex;
use hermes_core::relayer_components::build::traits::builders::birelay_builder::{
    BiRelayBuilder, BiRelayBuilderComponent,
};
use hermes_core::relayer_components::build::traits::builders::birelay_from_relay_builder::{
    BiRelayFromRelayBuilder, BiRelayFromRelayBuilderComponent,
};
use hermes_core::relayer_components::build::traits::builders::chain_builder::{
    CanBuildChain, ChainBuilder, ChainBuilderComponent,
};
use hermes_core::relayer_components::build::traits::builders::relay_builder::{
    RelayBuilder, RelayBuilderComponent,
};
use hermes_core::relayer_components::build::traits::builders::relay_from_chains_builder::{
    RelayFromChainsBuilder, RelayFromChainsBuilderComponent,
};
use hermes_core::relayer_components::multi::traits::birelay_at::BiRelayTypeProviderAtComponent;
use hermes_core::relayer_components::multi::traits::chain_at::ChainTypeProviderAtComponent;
use hermes_core::relayer_components::multi::traits::relay_at::RelayTypeProviderAtComponent;
use hermes_core::runtime_components::traits::{
    CanReadFileAsString, RuntimeGetterComponent, RuntimeTypeProviderComponent,
};
use hermes_cosmos::chain_components::types::Secp256k1KeyPair;
use hermes_cosmos::error::impls::UseHermesError;
use hermes_cosmos::error::types::Error;
use hermes_cosmos::error::HermesError;
use hermes_cosmos::relayer::contexts::{CosmosBuilder, CosmosChain};
use hermes_cosmos::runtime::types::runtime::HermesRuntime;
use hermes_prelude::*;
use hermes_starknet_chain_components::impls::StarknetChainConfig;
use hermes_starknet_chain_components::types::StarknetWallet;
use hermes_starknet_chain_context::contexts::{
    StarknetChain, StarknetChainFields, StarknetEventEncoding,
};
use hermes_starknet_chain_context::impls::HandleStarknetChainError;
use ibc::core::host::types::identifiers::{ChainId, ClientId};
use reqwest::Client;
use starknet::providers::jsonrpc::HttpTransport;
use starknet::providers::{JsonRpcClient, Provider};
use url::Url;

use crate::contexts::cosmos_starknet_birelay::CosmosStarknetBiRelay;
use crate::contexts::cosmos_to_starknet_relay::CosmosToStarknetRelay;
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
    pub starknet_chain_config: Option<StarknetChainConfig>,
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
        ErrorTypeProviderComponent: UseHermesError,
        ErrorRaiserComponent: UseDelegate<HandleStarknetChainError>,
        ChainTypeProviderAtComponent<Index<0>>: WithType<StarknetChain>,
        ChainTypeProviderAtComponent<Index<1>>: WithType<CosmosChain>,
        RuntimeTypeProviderComponent: WithType<HermesRuntime>,
        RuntimeGetterComponent: WithField<symbol!("runtime")>,
        RelayTypeProviderAtComponent<Index<0>, Index<1>>: WithType<StarknetToCosmosRelay>,
        RelayTypeProviderAtComponent<Index<1>, Index<0>>: WithType<CosmosToStarknetRelay>,
        BiRelayTypeProviderAtComponent<Index<0>, Index<1>>: WithType<StarknetCosmosBiRelay>,
        BiRelayTypeProviderAtComponent<Index<1>, Index<0>>: WithType<CosmosStarknetBiRelay>,
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

#[cgp_provider(RelayBuilderComponent)]
impl RelayBuilder<StarknetBuilder, Index<0>, Index<1>> for StarknetBuildComponents {
    async fn build_relay(
        build: &StarknetBuilder,
        _index: PhantomData<(Index<0>, Index<1>)>,
        src_chain_id: &ChainId,
        dst_chain_id: &ChainId,
        src_client_id: &ClientId,
        dst_client_id: &ClientId,
    ) -> Result<StarknetToCosmosRelay, HermesError> {
        let src_chain = build.build_chain(src_chain_id).await?;

        let dst_chain = build.cosmos_builder.build_chain(dst_chain_id).await?;

        Ok(
            build.build_starknet_to_cosmos_relay(
                src_chain,
                dst_chain,
                src_client_id,
                dst_client_id,
            ),
        )
    }
}

#[cgp_provider(RelayBuilderComponent)]
impl RelayBuilder<StarknetBuilder, Index<1>, Index<0>> for StarknetBuildComponents {
    async fn build_relay(
        build: &StarknetBuilder,
        _index: PhantomData<(Index<1>, Index<0>)>,
        src_chain_id: &ChainId,
        dst_chain_id: &ChainId,
        src_client_id: &ClientId,
        dst_client_id: &ClientId,
    ) -> Result<CosmosToStarknetRelay, HermesError> {
        let src_chain = build.cosmos_builder.build_chain(dst_chain_id).await?;

        let dst_chain = build.build_chain(src_chain_id).await?;

        Ok(
            build.build_cosmos_to_starknet_relay(
                src_chain,
                dst_chain,
                src_client_id,
                dst_client_id,
            ),
        )
    }
}

#[cgp_provider(RelayFromChainsBuilderComponent)]
impl RelayFromChainsBuilder<StarknetBuilder, Index<0>, Index<1>> for StarknetBuildComponents {
    async fn build_relay_from_chains(
        build: &StarknetBuilder,
        _index: PhantomData<(Index<0>, Index<1>)>,
        src_client_id: &ClientId,
        dst_client_id: &ClientId,
        src_chain: StarknetChain,
        dst_chain: CosmosChain,
    ) -> Result<StarknetToCosmosRelay, HermesError> {
        Ok(
            build.build_starknet_to_cosmos_relay(
                src_chain,
                dst_chain,
                src_client_id,
                dst_client_id,
            ),
        )
    }
}

#[cgp_provider(RelayFromChainsBuilderComponent)]
impl RelayFromChainsBuilder<StarknetBuilder, Index<1>, Index<0>> for StarknetBuildComponents {
    async fn build_relay_from_chains(
        build: &StarknetBuilder,
        _index: PhantomData<(Index<1>, Index<0>)>,
        src_client_id: &ClientId,
        dst_client_id: &ClientId,
        src_chain: CosmosChain,
        dst_chain: StarknetChain,
    ) -> Result<CosmosToStarknetRelay, HermesError> {
        Ok(
            build.build_cosmos_to_starknet_relay(
                src_chain,
                dst_chain,
                src_client_id,
                dst_client_id,
            ),
        )
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

#[cgp_provider(BiRelayFromRelayBuilderComponent)]
impl BiRelayFromRelayBuilder<StarknetBuilder, Index<0>, Index<1>> for StarknetBuildComponents {
    async fn build_birelay_from_relays(
        build: &StarknetBuilder,
        relay_a_to_b: StarknetToCosmosRelay,
        relay_b_to_a: CosmosToStarknetRelay,
    ) -> Result<StarknetCosmosBiRelay, HermesError> {
        let birelay = StarknetCosmosBiRelay {
            runtime: build.runtime.clone(),
            relay_a_to_b,
            relay_b_to_a,
        };

        Ok(birelay)
    }
}

impl StarknetBuilder {
    pub fn new(
        runtime: HermesRuntime,
        cosmos_builder: CosmosBuilder,
        starknet_chain_config: Option<StarknetChainConfig>,
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
        let chain_config = self
            .starknet_chain_config
            .as_ref()
            .ok_or_else(|| Self::raise_error("starknet chain config not found"))?;

        let json_rpc_url = Url::parse(&chain_config.json_rpc_url)?;

        let starknet_rpc_client =
            Arc::new(JsonRpcClient::new(HttpTransport::new(json_rpc_url.clone())));

        let chain_id_felt = starknet_rpc_client.chain_id().await?;

        let chain_id = chain_id_felt.to_string().parse()?;

        if &chain_id != expected_chain_id {
            return Err(eyre!("Starknet Starknet chain has a different ID as configured. Expected: {expected_chain_id}, got: {chain_id}").into());
        }

        let wallet_path = PathBuf::from(chain_config.relayer_wallet.clone());

        let wallet_str = self.runtime.read_file_as_string(&wallet_path).await?;

        let relayer_wallet: StarknetWallet = toml::from_str(&wallet_str)
            .map_err(|e| eyre!("Failed to parse relayer wallet: {e}"))?;

        let proof_signer = Secp256k1KeyPair::from_mnemonic(
            bip39::Mnemonic::from_entropy(
                &relayer_wallet.signing_key.to_bytes_be(),
                bip39::Language::English,
            )
            .expect("valid mnemonic")
            .phrase(),
            &"m/84'/0'/0'/0/0".parse().expect("valid hdpath"),
            "strk",
        )
        .expect("valid key pair");

        let contract_classes = &chain_config.contract_classes;

        let contract_addresses = &chain_config.contract_addresses;

        let event_encoding = StarknetEventEncoding::default();

        event_encoding
            .erc20_hashes
            .set(HashSet::from_iter(contract_classes.erc20))
            .unwrap();
        event_encoding
            .ics20_hashes
            .set(HashSet::from_iter(contract_classes.ics20))
            .unwrap();
        event_encoding
            .ibc_client_hashes
            .set(HashSet::from_iter(contract_classes.ibc_client))
            .unwrap();
        event_encoding
            .ibc_core_contract_addresses
            .set(HashSet::from_iter(contract_addresses.ibc_core))
            .unwrap();

        let ibc_client_contract_address = OnceLock::new();

        if let Some(address) = contract_addresses.ibc_client {
            ibc_client_contract_address.set(address).unwrap();
        }

        let ibc_core_contract_address = OnceLock::new();

        if let Some(address) = contract_addresses.ibc_core {
            ibc_core_contract_address.set(address).unwrap();
        }

        let ibc_ics20_contract_address = OnceLock::new();

        if let Some(address) = contract_addresses.ibc_ics20 {
            ibc_ics20_contract_address.set(address).unwrap();
        }

        let rpc_client = Client::new();

        let context = StarknetChain {
            fields: Arc::new(StarknetChainFields {
                runtime: self.runtime.clone(),
                chain_id,
                starknet_client: starknet_rpc_client,
                ibc_client_contract_address,
                ibc_core_contract_address,
                ibc_ics20_contract_address,
                event_encoding,
                proof_signer,
                poll_interval: chain_config.poll_interval,
                block_time: chain_config.block_time,
                nonce_mutex: Arc::new(Mutex::new(())),
                signer: relayer_wallet,
                rpc_client,
                json_rpc_url,
            }),
        };

        Ok(context)
    }

    pub fn build_starknet_to_cosmos_relay(
        &self,
        src_chain: StarknetChain,
        dst_chain: CosmosChain,
        src_client_id: &ClientId,
        dst_client_id: &ClientId,
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
        src_client_id: &ClientId,
        dst_client_id: &ClientId,
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
