use core::marker::PhantomData;
use core::time::Duration;
use std::env::var;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::SystemTime;

use hermes_chain_components::traits::message_builders::update_client::CanBuildUpdateClientMessage;
use hermes_chain_components::traits::payload_builders::update_client::CanBuildUpdateClientPayload;
use hermes_chain_components::traits::queries::chain_status::CanQueryChainHeight;
use hermes_chain_components::traits::queries::client_state::CanQueryClientStateWithLatestHeight;
use hermes_chain_components::traits::queries::consensus_state::CanQueryConsensusStateWithLatestHeight;
use hermes_chain_components::traits::send_message::CanSendMessages;
use hermes_cosmos_integration_tests::init::init_test_runtime;
use hermes_cosmos_relayer::contexts::build::CosmosBuilder;
use hermes_cosmos_relayer::contexts::chain::CosmosChain;
use hermes_cosmos_wasm_relayer::context::cosmos_bootstrap::CosmosWithWasmClientBootstrap;
use hermes_error::types::Error;
use hermes_relayer_components::relay::traits::client_creator::CanCreateClient;
use hermes_relayer_components::relay::traits::target::SourceTarget;
use hermes_runtime_components::traits::fs::read_file::CanReadFileAsString;
use hermes_runtime_components::traits::sleep::CanSleep;
use hermes_starknet_chain_components::traits::contract::declare::CanDeclareContract;
use hermes_starknet_chain_components::traits::contract::deploy::CanDeployContract;
use hermes_starknet_chain_context::contexts::chain::StarknetChain;
use hermes_starknet_relayer::contexts::starknet_to_cosmos_relay::StarknetToCosmosRelay;
use hermes_test_components::bootstrap::traits::chain::CanBootstrapChain;
use ibc_relayer::chain::cosmos::client::Settings;
use ibc_relayer::config::types::TrustThreshold;
use ibc_relayer_types::Height as CosmosHeight;

use crate::contexts::bootstrap::StarknetBootstrap;

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

        let create_client_settings = Settings {
            max_clock_drift: Duration::from_secs(40),
            trusting_period: Some(Duration::from_secs(60 * 60)),
            trust_threshold: TrustThreshold::ONE_THIRD,
        };

        let client_id = StarknetToCosmosRelay::create_client(
            SourceTarget,
            starknet_chain,
            cosmos_chain,
            &create_client_settings,
            &(),
        ).await?;

        println!("created client on Starknet: {:?}", client_id);

        runtime.sleep(Duration::from_secs(1)).await;

        let update_header = {
            let target_height = cosmos_chain.query_chain_height().await?;

            let client_state = starknet_chain.query_client_state_with_latest_height(
                PhantomData::<CosmosChain>, &client_id
            )
            .await?;

            let trusted_height = CosmosHeight::new(
                client_state.latest_height.revision_number,
                client_state.latest_height.revision_height,
            )?;

            <CosmosChain as CanBuildUpdateClientPayload<StarknetChain>>::build_update_client_payload(cosmos_chain, &trusted_height, &target_height, client_state).await?
        };

        {
            let message = <StarknetChain as CanBuildUpdateClientMessage<CosmosChain>>::build_update_client_message(
                starknet_chain, &client_id, update_header.clone()).await?;

            let events = starknet_chain.send_messages(message).await?;

            println!("update client events: {:?}", events);
        }

        {
            let client_state = starknet_chain.query_client_state_with_latest_height(
                PhantomData::<CosmosChain>, &client_id
            )
            .await?;

            println!("queried client state: {client_state:?}");
        }

        {
            let consensus_state = starknet_chain.query_consensus_state_with_latest_height(
                PhantomData::<CosmosChain>,
                &client_id,
                &CosmosHeight::new(update_header.target_height.revision_number, update_header.target_height.revision_height)?,
            )
            .await?;

            println!("queried consensus state: {consensus_state:?}");
        }

        Ok(())
    })
}
