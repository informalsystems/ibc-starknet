use hermes_core::test_components::setup::traits::{FullNodeForker, FullNodeForkerComponent};
use hermes_cosmos::error::HermesError;
use hermes_cosmos::integration_tests::contexts::CosmosChainDriver;
use hermes_prelude::*;

use crate::contexts::{StarknetChainDriver, StarknetTestDriver};

#[cgp_new_provider(FullNodeForkerComponent)]
impl FullNodeForker<StarknetTestDriver> for ForkSecondFullNode {
    async fn fork_full_node(
        driver: &StarknetTestDriver,
    ) -> Result<StarknetTestDriver, HermesError> {
        // TODO

        let forked_starknet_chain_driver = StarknetChainDriver {
            runtime: driver.starknet_chain_driver.runtime.clone(),
            chain: driver.starknet_chain_driver.chain.clone(),
            chain_store_dir: driver.starknet_chain_driver.chain_store_dir.clone(),
            genesis_config: driver.starknet_chain_driver.genesis_config.clone(),
            node_config: driver.starknet_chain_driver.node_config.clone(),
            wallets: driver.starknet_chain_driver.wallets.clone(),
            chain_processes: vec![],
            relayer_wallet_1: driver.starknet_chain_driver.relayer_wallet_1.clone(),
            relayer_wallet_2: driver.starknet_chain_driver.relayer_wallet_2.clone(),
            user_wallet_a: driver.starknet_chain_driver.user_wallet_a.clone(),
            user_wallet_b: driver.starknet_chain_driver.user_wallet_b.clone(),
        };

        let forked_cosmos_chain_driver = CosmosChainDriver {
            chain: driver.cosmos_chain_driver.chain.clone(),
            chain_command_path: driver.cosmos_chain_driver.chain_command_path.clone(),
            chain_processes: vec![],
            chain_node_config: driver.cosmos_chain_driver.chain_node_config.clone(),
            genesis_config: driver.cosmos_chain_driver.genesis_config.clone(),
            validator_wallet: driver.cosmos_chain_driver.validator_wallet.clone(),
            relayer_wallet: driver.cosmos_chain_driver.relayer_wallet.clone(),
            user_wallet_a: driver.cosmos_chain_driver.user_wallet_a.clone(),
            user_wallet_b: driver.cosmos_chain_driver.user_wallet_b.clone(),
            wallets: driver.cosmos_chain_driver.wallets.clone(),
        };

        Ok(StarknetTestDriver {
            relay_driver_a_b: driver.relay_driver_a_b.clone(),
            relay_driver_b_a: driver.relay_driver_b_a.clone(),
            starknet_chain_driver: forked_starknet_chain_driver,
            cosmos_chain_driver: forked_cosmos_chain_driver,
            client_id_a: driver.client_id_a.clone(),
            client_id_b: driver.client_id_b.clone(),
            connection_id_a: driver.connection_id_a.clone(),
            connection_id_b: driver.connection_id_b.clone(),
            channel_id_a: driver.channel_id_a.clone(),
            channel_id_b: driver.channel_id_b.clone(),
            port_id_a: driver.port_id_a.clone(),
            port_id_b: driver.port_id_b.clone(),
            create_client_payload_options_a: driver.create_client_payload_options_a.clone(),
            create_client_payload_options_b: driver.create_client_payload_options_b.clone(),
            create_client_message_options_a: (),
            create_client_message_options_b: (),
            recover_client_payload_options_a:
                hermes_starknet_chain_components::impls::StarknetRecoverClientPayload,
            recover_client_payload_options_b: driver.recover_client_payload_options_b.clone(),
        })
    }
}
