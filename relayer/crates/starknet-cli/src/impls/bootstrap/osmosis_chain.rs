use cgp::extra::runtime::HasRuntime;
use hermes_cli_components::traits::{BootstrapLoader, BootstrapLoaderComponent, HasBootstrapType};
use hermes_cosmos::chain_components::types::{DynamicGasConfig, EipQueryType};
use hermes_cosmos::error::HermesError;
use hermes_cosmos::relayer::contexts::CosmosBuilder;
use hermes_cosmos::runtime::types::runtime::HermesRuntime;
use hermes_prelude::*;
use hermes_starknet_integration_tests::contexts::OsmosisBootstrap;
use hermes_starknet_integration_tests::utils::load_wasm_client;
use tracing::info;

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

    #[clap(long = "wasm-additional-byte-code")]
    pub wasm_additional_byte_code: Option<String>,

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

        let (wasm_code_hash, wasm_client_byte_code) = load_wasm_client(&args.wasm_client_code_path)
            .await
            .map_err(App::raise_error)?;

        let wasm_additional_byte_code = args
            .wasm_additional_byte_code
            .as_ref()
            .map_or_else(
                || Ok(Vec::new()),
                |paths_str| paths_str.split(',').map(std::fs::read).collect(),
            )
            .map_err(App::raise_error)?;

        info!(
            target: "hermes::cli",
            wasm_code_hash = %hex::encode(wasm_code_hash),
            "bootstrapping Osmosis chain with Starknet Wasm client",
        );

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
            wasm_additional_byte_code,
            governance_proposal_authority: args.governance_proposal_authority.clone(),
        };

        Ok(bootstrap)
    }
}
