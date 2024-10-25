#![recursion_limit = "256"]

use core::time::Duration;
use std::env::var;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::SystemTime;

use hermes_chain_components::traits::payload_builders::create_client::CanBuildCreateClientPayload;
use hermes_chain_components::traits::queries::client_state::CanQueryClientStateWithLatestHeight;
use hermes_chain_components::traits::queries::consensus_state::CanQueryConsensusStateWithLatestHeight;
use hermes_chain_components::traits::send_message::CanSendSingleMessage;
use hermes_cosmos_integration_tests::init::init_test_runtime;
use hermes_cosmos_relayer::contexts::build::CosmosBuilder;
use hermes_cosmos_relayer::contexts::chain::CosmosChain;
use hermes_cosmos_wasm_relayer::context::cosmos_bootstrap::CosmosWithWasmClientBootstrap;
use hermes_encoding_components::traits::encode::CanEncode;
use hermes_encoding_components::HList;
use hermes_error::types::Error;
use hermes_runtime_components::traits::fs::read_file::CanReadFileAsString;
use hermes_starknet_chain_components::impls::encoding::events::CanFilterDecodeEvents;
use hermes_starknet_chain_components::traits::contract::declare::CanDeclareContract;
use hermes_starknet_chain_components::traits::contract::deploy::CanDeployContract;
use hermes_starknet_chain_components::types::cosmos::client_state::{
    ClientStatus, CometClientState,
};
use hermes_starknet_chain_components::types::cosmos::consensus_state::CometConsensusState;
use hermes_starknet_chain_components::types::cosmos::height::Height;
use hermes_starknet_chain_components::types::cosmos::update::CometUpdateHeader;
use hermes_starknet_chain_components::types::events::create_client::CreateClientEvent;
use hermes_starknet_chain_context::contexts::chain::StarknetChain;
use hermes_starknet_chain_context::contexts::encoding::cairo::StarknetCairoEncoding;
use hermes_starknet_chain_context::contexts::encoding::event::StarknetEventEncoding;
use hermes_starknet_integration_tests::contexts::bootstrap::StarknetBootstrap;
use hermes_test_components::bootstrap::traits::chain::CanBootstrapChain;
use ibc_relayer::chain::cosmos::client::Settings;
use ibc_relayer::config::types::TrustThreshold;
use starknet::accounts::Call;
use starknet::macros::{selector, short_string};

#[test]
fn test_starknet_comet_client_contract() -> Result<(), Error> {
    let runtime = init_test_runtime();

    runtime.runtime.clone().block_on(async move {
        let starknet_chain_command_path = std::env::var("STARKNET_BIN")
            .unwrap_or("starknet-devnet".into())
            .into();

        let wasm_client_code_path = PathBuf::from(
            var("STARKNET_WASM_CLIENT_PATH").expect("Wasm blob for Starknet light client is required"),
        );

        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)?
            .as_secs();

        let starknet_bootstrap = StarknetBootstrap {
            runtime: runtime.clone(),
            chain_command_path: starknet_chain_command_path,
            chain_store_dir: format!("./test-data/{timestamp}/starknet").into(),
        };

        let wasm_client_byte_code = tokio::fs::read(&wasm_client_code_path).await?;

        let cosmos_builder = Arc::new(CosmosBuilder::new_with_default(runtime.clone()));

        let cosmos_bootstrap = Arc::new(CosmosWithWasmClientBootstrap {
            runtime: runtime.clone(),
            builder: cosmos_builder,
            should_randomize_identifiers: true,
            chain_store_dir: format!("./test-data/{timestamp}/cosmos").into(),
            chain_command_path: "simd".into(),
            account_prefix: "cosmos".into(),
            staking_denom: "stake".into(),
            transfer_denom: "coin".into(),
            wasm_client_byte_code,
            governance_proposal_authority: "cosmos10d07y265gmmuvt4z0w9aw880jnsr700j6zn9kn".into(), // TODO: don't hard code this
        });

        let mut starknet_chain_driver = starknet_bootstrap.bootstrap_chain("starknet").await?;

        let starknet_chain = &mut starknet_chain_driver.chain;

        let cosmos_chain_driver = cosmos_bootstrap.bootstrap_chain("cosmos").await?;

        let cosmos_chain = &cosmos_chain_driver.chain;

        let comet_client_class_hash = {
            let contract_path = std::env::var("COMET_CLIENT_CONTRACT")?;

            let contract_str = runtime.read_file_as_string(&contract_path.into()).await?;

            let contract = serde_json::from_str(&contract_str)?;

            let class_hash = starknet_chain.declare_contract(&contract).await?;

            println!("declared class: {:?}", class_hash);

            class_hash
        };

        let comet_client_address = starknet_chain
            .deploy_contract(&comet_client_class_hash, false, &Vec::new())
            .await?;

        println!(
            "deployed Comet client contract to address: {:?}",
            comet_client_address
        );

        starknet_chain.ibc_client_contract_address = Some(comet_client_address);

        let event_encoding = StarknetEventEncoding {
            erc20_hashes: Default::default(),
            ics20_hashes: Default::default(),
            ibc_client_hashes: [comet_client_class_hash].into(),
        };

        let create_client_settings = Settings {
            max_clock_drift: Duration::from_secs(40),
            trusting_period: Some(Duration::from_secs(60 * 60)),
            trust_threshold: TrustThreshold::ONE_THIRD,
        };

        let create_client_payload_1 = <CosmosChain as CanBuildCreateClientPayload<StarknetChain>>::build_create_client_payload(cosmos_chain, &create_client_settings).await?;

        let height_1 = Height {
            revision_number: create_client_payload_1.client_state.latest_height().revision_number(),
            revision_height: create_client_payload_1.client_state.latest_height().revision_height(),
        };

        let root_1 = create_client_payload_1.consensus_state.root.into_vec();

        let client_id = {
            let message = {
                let client_type = short_string!("07-cometbft");

                let client_state = CometClientState {
                    latest_height: height_1.clone(),
                    trusting_period: create_client_payload_1.client_state.trusting_period.as_secs(),
                    status: ClientStatus::Active,
                };

                let consensus_state = CometConsensusState {
                    timestamp: create_client_payload_1.consensus_state.timestamp.unix_timestamp() as u64,
                    root: root_1.clone(),
                };

                let raw_client_state = StarknetCairoEncoding.encode(&client_state)?;
                let raw_consensus_state = StarknetCairoEncoding.encode(&consensus_state)?;

                let calldata = StarknetCairoEncoding.encode(&HList![
                    client_type,
                    raw_client_state,
                    raw_consensus_state
                ])?;

                Call {
                    to: comet_client_address,
                    selector: selector!("create_client"),
                    calldata,
                }
            };

            let events = starknet_chain.send_message(message).await?;

            let create_client_event: CreateClientEvent = event_encoding
                .filter_decode_events(&events)?
                .into_iter()
                .next()
                .unwrap();

            let client_id = create_client_event.client_id;

            println!("created client on Starknet: {:?}", client_id);

            client_id
        };

        {
            let consensus_state = <StarknetChain as CanQueryConsensusStateWithLatestHeight<
                CosmosChain,
            >>::query_consensus_state_with_latest_height(
                starknet_chain,
                &client_id,
                &create_client_payload_1.client_state.latest_height(),
            )
            .await?;

            println!("queried consensus state: {consensus_state:?}");

            assert_eq!(consensus_state.root, root_1);
        }

        let create_client_payload_2 = <CosmosChain as CanBuildCreateClientPayload<StarknetChain>>::build_create_client_payload(cosmos_chain, &create_client_settings).await?;
        let root_2 = create_client_payload_2.consensus_state.root.into_vec();

        {
            let message = {
                let height_2 = Height {
                    revision_number: create_client_payload_2.client_state.latest_height().revision_number(),
                    revision_height: create_client_payload_2.client_state.latest_height().revision_height(),
                };

                let update_header = CometUpdateHeader {
                    trusted_height: height_1,
                    target_height: height_2,
                    time: create_client_payload_2.consensus_state.timestamp.unix_timestamp() as u64,
                    root: root_2.clone(),
                };

                let raw_header = StarknetCairoEncoding.encode(&update_header)?;

                let calldata = StarknetCairoEncoding.encode(&(&client_id, raw_header))?;

                Call {
                    to: comet_client_address,
                    selector: selector!("update_client"),
                    calldata,
                }
            };

            let events = starknet_chain.send_message(message).await?;

            println!("update client events: {:?}", events);
        }

        {
            let client_state = <StarknetChain as CanQueryClientStateWithLatestHeight<
                CosmosChain,
            >>::query_client_state_with_latest_height(
                starknet_chain, &client_id
            )
            .await?;

            println!("queried client state: {client_state:?}");
        }

        {
            let consensus_state = <StarknetChain as CanQueryConsensusStateWithLatestHeight<
                CosmosChain,
            >>::query_consensus_state_with_latest_height(
                starknet_chain,
                &client_id,
                &create_client_payload_2.client_state.latest_height(),
            )
            .await?;

            println!("queried consensus state: {consensus_state:?}");

            assert_eq!(consensus_state.root, root_2);
        }

        Ok(())
    })
}
