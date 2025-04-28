use core::time::Duration;

use cgp::core::field::Index;
use cgp::prelude::*;
use hermes_cli::commands::CreateCosmosClientArgs;
use hermes_cli_components::impls::{CreateClientOptionsParser, CreateClientOptionsParserComponent};
use hermes_cosmos_chain_components::types::CosmosCreateClientOptions;
use hermes_cosmos_relayer::contexts::CosmosChain;
use hermes_error::HermesError;
use hermes_starknet_chain_components::types::payloads::client::StarknetCreateClientPayloadOptions;
use hermes_starknet_chain_context::contexts::chain::StarknetChain;
use hex::FromHex;
use ibc::clients::tendermint::types::TrustThreshold;

use crate::contexts::app::{StarknetApp, StarknetAppComponents};

#[derive(Debug, clap::Parser, HasField)]
pub struct CreateStarknetClientArgs {
    /// Identifier of the chain that hosts the client
    #[clap(
        long = "target-chain-id",
        required = true,
        value_name = "TARGET_CHAIN_ID",
        help_heading = "REQUIRED"
    )]
    pub target_chain_id: String,

    /// Identifier of the chain targeted by the client
    #[clap(
        long = "counterparty-chain-id",
        required = true,
        value_name = "COUNTERPARTY_CHAIN_ID",
        help_heading = "REQUIRED"
    )]
    pub counterparty_chain_id: String,

    #[clap(long = "wasm-code-hash")]
    pub wasm_code_hash: String,
}

#[cgp_provider(CreateClientOptionsParserComponent)]
impl CreateClientOptionsParser<StarknetApp, CreateCosmosClientArgs, Index<0>, Index<1>>
    for StarknetAppComponents
{
    async fn parse_create_client_options(
        _app: &StarknetApp,
        args: &CreateCosmosClientArgs,
        _target_chain: &StarknetChain,
        counterparty_chain: &CosmosChain,
    ) -> Result<((), CosmosCreateClientOptions), HermesError> {
        let max_clock_drift = match args.clock_drift.map(|d| d.into()) {
            Some(input) => input,
            None => {
                counterparty_chain.chain_config.clock_drift
                    + counterparty_chain.chain_config.max_block_time
            }
        };

        let settings = CosmosCreateClientOptions {
            max_clock_drift,
            trusting_period: args
                .trusting_period
                .map(|d| d.into())
                .unwrap_or_else(|| Duration::from_secs(14 * 24 * 3600)),
            trust_threshold: args
                .trust_threshold
                .unwrap_or(TrustThreshold::TWO_THIRDS)
                .into(),
        };

        Ok(((), settings))
    }
}

#[cgp_provider(CreateClientOptionsParserComponent)]
impl CreateClientOptionsParser<StarknetApp, CreateStarknetClientArgs, Index<1>, Index<0>>
    for StarknetAppComponents
{
    async fn parse_create_client_options(
        _app: &StarknetApp,
        args: &CreateStarknetClientArgs,
        _target_chain: &CosmosChain,
        _counterparty_chain: &StarknetChain,
    ) -> Result<((), StarknetCreateClientPayloadOptions), HermesError> {
        let wasm_code_hash = <[u8; 32]>::from_hex(&args.wasm_code_hash)?;

        Ok(((), StarknetCreateClientPayloadOptions { wasm_code_hash }))
    }
}
