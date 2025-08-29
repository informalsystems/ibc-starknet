use hermes_cli_components::traits::{
    BuilderLoader, BuilderLoaderComponent, CanLoadConfig, HasBuilderType, HasConfigType,
};
use hermes_core::runtime_components::traits::HasRuntime;
use hermes_cosmos::relayer::contexts::CosmosBuilder;
use hermes_cosmos::runtime::types::runtime::HermesRuntime;
use hermes_prelude::*;
use hermes_starknet_chain_components::impls::StarknetRelayerConfig;
use hermes_starknet_relayer::contexts::StarknetBuilder;

pub struct LoadStarknetBuilder;

#[cgp_provider(BuilderLoaderComponent)]
impl<App> BuilderLoader<App> for LoadStarknetBuilder
where
    App: HasBuilderType<Builder = StarknetBuilder>
        + HasConfigType<Config = StarknetRelayerConfig>
        + HasRuntime<Runtime = HermesRuntime>
        + CanLoadConfig
        + CanRaiseAsyncError<&'static str>,
{
    async fn load_builder(app: &App) -> Result<App::Builder, App::Error> {
        let runtime = app.runtime().clone();
        let config = app.load_config().await?;
        let cosmos_chain_config = config.cosmos_chain_config.ok_or_else(|| {
            App::raise_error("missing Cosmos chain config in Starknet relayer config")
        })?;

        let cosmos_builder = CosmosBuilder::new(
            vec![cosmos_chain_config],
            runtime.clone(),
            Default::default(),
            Default::default(),
            Default::default(),
        );

        let starknet_chain_config = config.starknet_chain_config.ok_or_else(|| {
            App::raise_error("missing Starknet chain config in Starknet relayer config")
        })?;

        let builder = StarknetBuilder::new(runtime, cosmos_builder, Some(starknet_chain_config));

        Ok(builder)
    }
}
