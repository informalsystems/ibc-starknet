#![recursion_limit = "256"]

use core::marker::PhantomData;
use core::time::Duration;
use std::sync::Arc;

use cgp::extra::run::CanRun;
use hermes_chain_components::traits::{
    CanQueryChainHeight, CanQueryChainStatus, CanQueryChannelEnd,
    CanQueryClientStateWithLatestHeight, CanQueryConnectionEnd, HasChainId,
};
use hermes_cli_components::traits::CanLoadBuilder;
use hermes_cosmos_chain_components::types::{
    CosmosCreateClientOptions, CosmosInitChannelOptions, CosmosInitConnectionOptions,
};
use hermes_cosmos_relayer::contexts::CosmosChain;
use hermes_error::Error;
use hermes_relayer_components::relay::impls::{CanBootstrapChannel, CanBootstrapConnection};
use hermes_relayer_components::relay::traits::{CanCreateClient, DestinationTarget, SourceTarget};
use hermes_runtime::types::runtime::HermesRuntime;
use hermes_starknet_chain_components::types::payloads::client::StarknetCreateClientPayloadOptions;
use hermes_starknet_chain_context::contexts::chain::StarknetChain;
use hermes_starknet_cli::contexts::app::StarknetApp;
use hermes_starknet_relayer::contexts::cosmos_to_starknet_relay::CosmosToStarknetRelay;
use hermes_starknet_relayer::contexts::starknet_to_cosmos_relay::StarknetToCosmosRelay;
use hex::FromHex;
use ibc::core::connection::types::version::Version as IbcConnectionVersion;
use ibc::core::host::types::identifiers::PortId;
use starknet::core::types::Felt;
use starknet::macros::short_string;
use tokio::runtime::Builder;
use tracing::{info, Level};

pub const COSMOS_HD_PATH: &str = "m/44'/118'/0'/0/0";

// https://github.com/osmosis-labs/testnets
pub const OSMOSIS_TESTNET_URL: &str = "rpc.testnet.osmosis.zone:443";
pub const OSMOSIS_TESTNET_CHAIN_ID: &str = "osmo-test-5";
pub const OSMOSIS_TOKEN: &str = "uosmo";

// https://docs.starknet.io/tools/fullnodes-rpc-providers/#open_endpoints
pub const STARKNET_TESTNET_URL: &str = "https://starknet-sepolia.public.blastapi.io/rpc/v0_8";
pub const STARKNET_TESTNET_CHAIN_ID: Felt = short_string!("SN_SEPOLIA");
// https://docs.starknet.io/chain-info
pub const STARKNET_STRK: Felt =
    Felt::from_hex_unchecked("0x04718f5a0fc34cc1af16a1cdee98ffb20c31f5cd61d6ab07201858f4287c938d");
pub const STARKNET_ETH: Felt =
    Felt::from_hex_unchecked("0x049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7");

pub const WASM_CODE_HASH_HEX: &str =
    "6be4d4cbb85ea2d7e0b17b7053e613af11e041617bdb163107dfd29f706318ef";

fn main() -> Result<(), Error> {
    let _ = stable_eyre::install();

    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let config_file = std::env::args().nth(1).expect("config file path");

    let tokio_runtime = Arc::new(Builder::new_multi_thread().enable_all().build().unwrap());

    let runtime = HermesRuntime::new(tokio_runtime);

    info!("initialized Hermes runtime");

    runtime.runtime.block_on(async {
        let (starknet_chain, cosmos_chain) = {
            let starknet_app = StarknetApp {
                config_path: config_file.into(),
                runtime: runtime.clone(),
            };

            let starknet_builder = starknet_app.load_builder().await?;

            let starknet_chain: StarknetChain = starknet_builder
                .build_chain(&STARKNET_TESTNET_CHAIN_ID.to_string().parse()?)
                .await
                .map_err(|e| eyre::eyre!("failed to build starknet chain: {:?}", e))?;

            info!(
                "starknet_chain_status: {:?}",
                starknet_chain
                    .query_chain_status()
                    .await
                    .map_err(|e| eyre::eyre!("failed to query starknet chain status: {:?}", e))?
            );

            let cosmos_chain: CosmosChain = starknet_builder
                .cosmos_builder
                .build_chain(&OSMOSIS_TESTNET_CHAIN_ID.parse()?)
                .await?;

            info!(
                "cosmos_chain_status: {:?}",
                cosmos_chain.query_chain_status().await?
            );

            (starknet_chain, cosmos_chain)
        };

        let starknet_client_id = {
            // https://lcd.testnet.osmosis.zone/cosmos/staking/v1beta1/params
            // in seconds; 5 days.
            let osmosis_unbonding_period = 432000;

            // https://docs.starknet.io/chain-info/#current_limits
            // in seconds.
            let starknet_block_time = 30;

            StarknetToCosmosRelay::create_client(
                SourceTarget,
                &starknet_chain,
                &cosmos_chain,
                &CosmosCreateClientOptions {
                    // unbonding period is 5 days on osmo-test-5
                    //
                    // using (unbonading period - 1) as maximum allowed value.
                    trusting_period: Duration::from_secs(osmosis_unbonding_period - 1),
                    // starknet has 30 seconds block time
                    // block timestamp is when the sequencer started building the block
                    // which can be in the past
                    //
                    // using 5 mins as max clock drift to be more permissive.
                    max_clock_drift: Duration::from_secs(starknet_block_time * 10),

                    ..Default::default()
                },
                &(),
            )
            .await?
        };

        let cosmos_client_id = {
            StarknetToCosmosRelay::create_client(
                DestinationTarget,
                &cosmos_chain,
                &starknet_chain,
                &StarknetCreateClientPayloadOptions {
                    wasm_code_hash: <[u8; 32]>::from_hex(WASM_CODE_HASH_HEX).expect("valid hex"),
                },
                &(),
            )
            .await?
        };

        {
            info!("created client on Starknet: {:?}", starknet_client_id);

            let client_state_on_starknet = starknet_chain
                .query_client_state_with_latest_height(
                    PhantomData::<CosmosChain>,
                    &starknet_client_id,
                )
                .await?;

            info!(
                "client state on Starknet: {} => {:?}",
                starknet_client_id, client_state_on_starknet
            );

            assert_eq!(&client_state_on_starknet.chain_id, cosmos_chain.chain_id());
        }

        {
            info!("created client on Cosmos: {:?}", cosmos_client_id);

            let client_state_on_cosmos = cosmos_chain
                .query_client_state_with_latest_height(
                    PhantomData::<StarknetChain>,
                    &cosmos_client_id,
                )
                .await?;

            info!(
                "client state on Cosmos: {} => {:?}",
                cosmos_client_id, client_state_on_cosmos
            );

            assert_eq!(
                &client_state_on_cosmos.client_state.chain_id,
                starknet_chain.chain_id()
            );
        }

        let starknet_to_cosmos_relay = StarknetToCosmosRelay::new(
            runtime.clone(),
            starknet_chain.clone(),
            cosmos_chain.clone(),
            starknet_client_id.clone(),
            cosmos_client_id.clone(),
        );

        let cosmos_to_starknet_relay = CosmosToStarknetRelay::new(
            runtime.clone(),
            cosmos_chain.clone(),
            starknet_chain.clone(),
            cosmos_client_id.clone(),
            starknet_client_id.clone(),
        );

        {
            let starknet_to_cosmos_relay = starknet_to_cosmos_relay.clone();

            let cosmos_to_starknet_relay = cosmos_to_starknet_relay.clone();

            runtime.runtime.spawn(async move {
                let _ = starknet_to_cosmos_relay.run().await;
            });

            runtime.runtime.spawn(async move {
                let _ = cosmos_to_starknet_relay.run().await;
            });
        }

        let (starknet_connection_id, cosmos_connection_id) = {
            let conn_init_option = CosmosInitConnectionOptions {
                delay_period: Duration::from_secs(0),
                connection_version: IbcConnectionVersion::compatibles().first().unwrap().clone(),
            };

            starknet_to_cosmos_relay
                .bootstrap_connection(&conn_init_option)
                .await?
        };

        {
            info!(
                "created connection: {:?}(Starknet) <> {:?}(Osmosis)",
                starknet_connection_id, cosmos_connection_id
            );

            let starknet_connection_end =
                CanQueryConnectionEnd::<CosmosChain>::query_connection_end(
                    &starknet_chain,
                    &starknet_connection_id,
                    &starknet_chain.query_chain_height().await?,
                )
                .await?;

            info!(
                "starknet_connection: {:?} => {:?}",
                starknet_connection_id, starknet_connection_end
            );

            let cosmos_connection_end =
                CanQueryConnectionEnd::<StarknetChain>::query_connection_end(
                    &cosmos_chain,
                    &cosmos_connection_id,
                    &cosmos_chain.query_chain_height().await?,
                )
                .await?;

            info!(
                "cosmos_connection: {:?} => {:?}",
                cosmos_connection_id, cosmos_connection_end
            );
        }

        let (starknet_channel_id, cosmos_channel_id) = {
            let init_channel_options =
                CosmosInitChannelOptions::new(starknet_connection_id.clone());

            let ics20_port = PortId::transfer();

            starknet_to_cosmos_relay
                .bootstrap_channel(&ics20_port, &ics20_port, &init_channel_options)
                .await?
        };

        {
            info!(
                "created channel: {:?}(Starknet) <> {:?}(Osmosis)",
                starknet_channel_id, cosmos_channel_id
            );

            let starknet_channel_end = CanQueryChannelEnd::<CosmosChain>::query_channel_end(
                &starknet_chain,
                &starknet_channel_id,
                &PortId::transfer(),
                &starknet_chain.query_chain_height().await?,
            )
            .await?;

            info!(
                "starknet_channel: {:?} => {:?}",
                starknet_channel_id, starknet_channel_end
            );

            let cosmos_channel_end = CanQueryChannelEnd::<StarknetChain>::query_channel_end(
                &cosmos_chain,
                &cosmos_channel_id,
                &PortId::transfer(),
                &cosmos_chain.query_chain_height().await?,
            )
            .await?;

            info!(
                "cosmos_channel: {:?} => {:?}",
                cosmos_channel_id, cosmos_channel_end
            );
        }

        info!(
            "Osmosis: {}, {}, {}",
            cosmos_client_id, cosmos_connection_id, cosmos_channel_id
        );

        info!(
            "Starknet: {}, {}, {}",
            starknet_client_id, starknet_connection_id, starknet_channel_id
        );

        Ok(())
    })
}
