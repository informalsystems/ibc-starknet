use std::time::{SystemTime, UNIX_EPOCH};

use hermes_chain_components::traits::queries::client_state::CanQueryClientStateWithLatestHeight;
use hermes_chain_components::traits::queries::consensus_state::CanQueryConsensusStateWithLatestHeight;
use hermes_chain_components::traits::send_message::CanSendSingleMessage;
use hermes_cosmos_integration_tests::init::init_test_runtime;
use hermes_cosmos_relayer::contexts::chain::CosmosChain;
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
use ibc_relayer_types::Height as RelayerHeight;
use starknet::accounts::Call;
use starknet::macros::{selector, short_string};

#[test]
fn test_starknet_comet_client_contract() -> Result<(), Error> {
    let runtime = init_test_runtime();

    runtime.runtime.clone().block_on(async move {
        let chain_command_path = std::env::var("STARKNET_BIN")
            .unwrap_or("starknet-devnet".into())
            .into();

        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)?
            .as_secs();

        let bootstrap = StarknetBootstrap {
            runtime: runtime.clone(),
            chain_command_path,
            chain_store_dir: format!("./test-data/{timestamp}").into(),
        };

        let mut chain_driver = bootstrap.bootstrap_chain("starknet").await?;

        let chain = &mut chain_driver.chain;

        let comet_client_class_hash = {
            let contract_path = std::env::var("COMET_CLIENT_CONTRACT")?;

            let contract_str = runtime.read_file_as_string(&contract_path.into()).await?;

            let contract = serde_json::from_str(&contract_str)?;

            let class_hash = chain.declare_contract(&contract).await?;

            println!("declared class: {:?}", class_hash);

            class_hash
        };

        let comet_client_address = chain
            .deploy_contract(&comet_client_class_hash, false, &Vec::new())
            .await?;

        println!(
            "deployed Comet client contract to address: {:?}",
            comet_client_address
        );

        chain.ibc_client_contract_address = Some(comet_client_address);

        let event_encoding = StarknetEventEncoding {
            erc20_hashes: Default::default(),
            ics20_hashes: Default::default(),
            ibc_client_hashes: [comet_client_class_hash].into(),
        };

        let client_id = {
            let message = {
                let client_type = short_string!("07-cometbft");

                let height = Height {
                    revision_number: 0,
                    revision_height: 1,
                };

                let client_state = CometClientState {
                    latest_height: height,
                    trusting_period: 3600,
                    status: ClientStatus::Active,
                };

                let consensus_state = CometConsensusState {
                    timestamp: SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() - 10,
                    root: vec![1, 2, 3],
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

            let events = chain.send_message(message).await?;

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
            let message = {
                let update_header = CometUpdateHeader {
                    trusted_height: Height {
                        revision_number: 0,
                        revision_height: 1,
                    },
                    target_height: Height {
                        revision_number: 0,
                        revision_height: 2,
                    },
                    time: SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() - 10,
                    root: vec![4, 5, 6],
                };

                let raw_header = StarknetCairoEncoding.encode(&update_header)?;

                let calldata = StarknetCairoEncoding.encode(&(&client_id, raw_header))?;

                Call {
                    to: comet_client_address,
                    selector: selector!("update_client"),
                    calldata,
                }
            };

            let events = chain.send_message(message).await?;

            println!("update client events: {:?}", events);
        }

        {
            let client_state = <StarknetChain as CanQueryClientStateWithLatestHeight<
                CosmosChain,
            >>::query_client_state_with_latest_height(
                chain, &client_id
            )
            .await?;

            println!("queried client state: {client_state:?}");
        }

        {
            let consensus_state = <StarknetChain as CanQueryConsensusStateWithLatestHeight<
                CosmosChain,
            >>::query_consensus_state_with_latest_height(
                chain,
                &client_id,
                &RelayerHeight::new(0, 2)?,
            )
            .await?;

            println!("queried consensus state: {consensus_state:?}");

            assert_eq!(consensus_state.root, vec![4, 5, 6]);
        }

        Ok(())
    })
}
