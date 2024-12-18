use cgp::prelude::CanRaiseError;
use hermes_cli_components::traits::build::{BuilderLoader, HasBuilderType};
use hermes_cli_components::traits::config::load_config::CanLoadConfig;
use hermes_cli_components::traits::types::config::HasConfigType;
use hermes_cosmos_relayer::contexts::build::CosmosBuilder;
use hermes_runtime::types::runtime::HermesRuntime;
use hermes_runtime_components::traits::runtime::HasRuntime;
use hermes_starknet_chain_components::impls::types::config::{
    StarknetChainConfig, StarknetRelayerConfig,
};
use hermes_starknet_chain_components::types::wallet::StarknetWallet;
use hermes_starknet_relayer::contexts::builder::StarknetBuilder;
use starknet::macros::felt;

pub struct LoadStarknetBuilder;

impl<App> BuilderLoader<App> for LoadStarknetBuilder
where
    App: HasBuilderType<Builder = StarknetBuilder>
        + HasConfigType<Config = StarknetRelayerConfig>
        + HasRuntime<Runtime = HermesRuntime>
        + CanLoadConfig
        + CanRaiseError<&'static str>,
{
    async fn load_builder(app: &App) -> Result<App::Builder, App::Error> {
        let runtime = app.runtime().clone();
        let config = app.load_config().await?;
        let cosmos_chain_config = config.cosmos_chain_config.ok_or_else(|| {
            App::raise_error("missing Cosmos chain config in Starknet relayer config")
        })?;
        let rpc_addr = cosmos_chain_config.rpc_addr.to_string();

        let cosmos_builder = CosmosBuilder::new(
            vec![cosmos_chain_config],
            runtime.clone(),
            Default::default(),
            Default::default(),
            Default::default(),
            Default::default(),
        );

        // Extracted from relayer/crates/starknet-test-components/src/impls/bootstrap/bootstrap_chain.rs
        let relayer_wallet = StarknetWallet {
            account_address: felt!(
                "0x64b48806902a367c8598f4f95c305e8c1a1acba5f082d294a43793113115691"
            ),
            signing_key: felt!("0x71d7bb07b9a64f6f78ac4c816aff4da9"),
            public_key: felt!("0x39d9e6ce352ad4530a0ef5d5a18fd3303c3606a7fa6ac5b620020ad681cc33b"),
        };

        let starknet_chain_config = StarknetChainConfig {
            json_rpc_url: rpc_addr,
            relayer_wallet,
        };

        let builder = StarknetBuilder::new(cosmos_builder, runtime, starknet_chain_config);

        Ok(builder)
    }
}
