use cgp::extra::runtime::HasRuntime;
use cgp::prelude::*;
use hermes_cli_components::traits::bootstrap::{
    BootstrapLoader, BootstrapLoaderComponent, HasBootstrapType,
};
use hermes_cosmos_chain_components::types::config::gas::dynamic_gas_config::DynamicGasConfig;
use hermes_cosmos_chain_components::types::config::gas::eip_type::EipQueryType;
use hermes_cosmos_relayer::contexts::build::CosmosBuilder;
use hermes_error::HermesError;
use hermes_runtime::types::runtime::HermesRuntime;
use hermes_starknet_integration_tests::contexts::osmosis_bootstrap::OsmosisBootstrap;

#[derive(Debug, clap::Parser, HasField)]
pub struct BootstrapOsmosisChainArgs {
    #[clap(long = "chain-id", required = true)]
    pub chain_id: String,

    #[clap(long = "chain-store-dir", required = true)]
    pub chain_store_dir: String,

    #[clap(long = "chain-command-path", default_value = "osmosisd")]
    pub chain_command_path: String,

    #[clap(long = "account-prefix", default_value = "cosmos")]
    pub account_prefix: String,

    #[clap(long = "staking-denom", default_value = "stake")]
    pub staking_denom: String,

    #[clap(long = "transfer-denom", default_value = "samoleon")]
    pub transfer_denom: String,

    #[clap(long = "wasm-client-code-path")]
    pub wasm_client_code_path: String,

    #[clap(long = "governance-proposal-authority")]
    pub governance_proposal_authority: String,
}

#[cgp_new_provider(BootstrapLoaderComponent)]
impl<App, Tag> BootstrapLoader<App, Tag, BootstrapOsmosisChainArgs> for LoadOsmosisBootstrap
where
    App: HasBootstrapType<Tag, Bootstrap = OsmosisBootstrap>
        + HasRuntime<Runtime = HermesRuntime>
        + CanRaiseAsyncError<HermesError>
        + CanRaiseAsyncError<std::io::Error>,
{
    async fn load_bootstrap(
        app: &App,
        args: &BootstrapOsmosisChainArgs,
    ) -> Result<App::Bootstrap, App::Error> {
        let runtime = app.runtime();

        let builder = CosmosBuilder::new_with_default(runtime.clone());

        let wasm_client_byte_code = tokio::fs::read(&args.wasm_client_code_path)
            .await
            .map_err(App::raise_error)?;

        let bootstrap = OsmosisBootstrap {
            runtime: runtime.clone(),
            cosmos_builder: builder,
            should_randomize_identifiers: false,
            chain_store_dir: args.chain_store_dir.clone().into(),
            chain_command_path: args.chain_command_path.clone().into(),
            account_prefix: args.account_prefix.clone(),
            staking_denom_prefix: args.staking_denom.clone(),
            transfer_denom_prefix: args.transfer_denom.clone(),
            dynamic_gas: Some(DynamicGasConfig {
                multiplier: 1.1,
                max: 1.6,
                eip_query_type: EipQueryType::Osmosis,
                denom: args.staking_denom.clone(),
            }),
            wasm_client_byte_code,
            governance_proposal_authority: args.governance_proposal_authority.clone(),
        };

        Ok(bootstrap)
    }
}
