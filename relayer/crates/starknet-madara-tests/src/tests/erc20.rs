use hermes_core::chain_components::traits::{
    CanQueryBlock, CanQueryChainHeight, CanQueryChainStatus, CanSendSingleMessage,
};
use hermes_core::encoding_components::traits::CanEncode;
use hermes_core::runtime_components::traits::CanReadFileAsString;
use hermes_core::test_components::bootstrap::traits::CanBootstrapChain;
use hermes_error::Error;
use hermes_prelude::*;
use hermes_starknet_chain_components::impls::CanFilterDecodeEvents;
use hermes_starknet_chain_components::traits::{
    CanBuildTransferTokenMessage, CanDeclareContract, CanDeployContract, CanQueryStorageProof,
    CanQueryTokenBalance,
};
use hermes_starknet_chain_components::types::{Erc20Event, StarknetAmount};
use hermes_starknet_chain_context::contexts::{StarknetCairoEncoding, StarknetEventEncoding};
use starknet::core::crypto::pedersen_hash;
use starknet::core::types::U256;
use starknet::macros::selector;
use tracing::info;

use crate::contexts::MadaraChainDriver;
use crate::impls::{init_madara_bootstrap, init_test_runtime};

#[test]
fn test_madara_erc20() -> Result<(), Error> {
    let runtime = init_test_runtime();

    runtime.runtime.clone().block_on(async move {
        let madara_bootstrap = init_madara_bootstrap(&runtime).await?;

        let chain_driver: MadaraChainDriver = madara_bootstrap.bootstrap_chain("madara").await?;

        let chain = &chain_driver.chain;

        let chain_status = chain.query_chain_status().await?;

        info!("chain status: {chain_status}");

        let block = chain.query_block(&chain_status.height).await?;

        info!("block: {block}");

        let erc20_class_hash = {
            let contract_path = std::env::var("ERC20_CONTRACT")?;

            let contract_str: String = runtime.read_file_as_string(&contract_path.into()).await?;

            let contract = serde_json::from_str(&contract_str)?;

            let class_hash = chain.declare_contract(&contract).await?;

            info!("declared class: {:?}", class_hash);

            class_hash
        };

        let event_encoding = StarknetEventEncoding::default();

        event_encoding
            .erc20_hashes
            .set([erc20_class_hash].into())
            .unwrap();

        let initial_supply = 0x1234u32;

        let sender_address = chain_driver.relayer_wallet.account_address;
        let recipient_address = chain_driver.user_wallet_a.account_address;

        let token_address = {
            let calldata = StarknetCairoEncoding.encode(&product![
                "token".to_owned(),
                "token".to_owned(),
                U256::from(initial_supply),
                sender_address,
            ])?;

            let token_address = chain
                .deploy_contract(&erc20_class_hash, false, &calldata)
                .await?;

            info!("deployed ERC20 contract to address: {:?}", token_address);

            token_address
        };

        {
            let total_supply_key = selector!("ERC20_total_supply");

            let storage_proof = chain
                .query_storage_proof(
                    &chain.query_chain_height().await?,
                    &token_address,
                    &[total_supply_key],
                )
                .await?;

            let storage_proof_str = serde_json::to_string_pretty(&storage_proof)?;

            info!(
                total_supply = %initial_supply,
                selector = %total_supply_key.to_hex_string(),
                storage_proof = %storage_proof_str,
                "gotten storage proof for total supply"
            );
        }

        let base_balance_key = selector!("ERC20_balances");

        {
            let address = sender_address.0;
            let balance_key = pedersen_hash(&base_balance_key, &address);

            let storage_proof = chain
                .query_storage_proof(
                    &chain.query_chain_height().await?,
                    &token_address,
                    &[base_balance_key],
                )
                .await?;

            let storage_proof_str = serde_json::to_string_pretty(&storage_proof)?;

            info!(
                address = %address.to_hex_string(),
                selector = %balance_key.to_hex_string(),
                storage_proof = %storage_proof_str,
                "gotten storage proof for sender's balance"
            );
        }

        {
            let address = recipient_address.0;
            let balance_key = pedersen_hash(&base_balance_key, &address);

            let storage_proof = chain
                .query_storage_proof(
                    &chain.query_chain_height().await?,
                    &token_address,
                    &[balance_key],
                )
                .await?;

            let storage_proof_str = serde_json::to_string_pretty(&storage_proof)?;

            info!(
                address = %address.to_hex_string(),
                selector = %balance_key.to_hex_string(),
                storage_proof = %storage_proof_str,
                "gotten storage proof for recipient's balance"
            );
        }

        // {
        //     let address = recipient_address.0;

        //     for i in 1..10 {
        //         let balance_key = pedersen_hash(&base_balance_key, &(address + i));

        //         let storage_proof = chain
        //             .query_storage_proof(
        //                 &chain.query_chain_height().await?,
        //                 &token_address,
        //                 &[balance_key],
        //             )
        //             .await?;

        //         let storage_proof_str = serde_json::to_string_pretty(&storage_proof)?;

        //         info!(
        //             address = %address.to_hex_string(),
        //             selector = %balance_key.to_hex_string(),
        //             storage_proof = %storage_proof_str,
        //             "gotten storage proof for balance at recipient address + {i}"
        //         );
        //     }
        // }

        {
            // Test local ERC20 token transfer
            let sender_address = chain_driver.relayer_wallet.account_address;

            let recipient_address = chain_driver.user_wallet_a.account_address;

            info!("sender address: {:?}", sender_address);
            info!("recipient address: {:?}", recipient_address);

            let sender_balance_a = chain
                .query_token_balance(&token_address, &sender_address)
                .await?;

            info!("sender balance before: {}", sender_balance_a);

            let recipient_balance_a = chain
                .query_token_balance(&token_address, &recipient_address)
                .await?;

            info!("recipient balance before: {}", recipient_balance_a);

            let transfer_amount = 0x123u32.into();

            let message = chain.build_transfer_token_message(
                &recipient_address,
                &StarknetAmount::new(transfer_amount, token_address),
            )?;

            let response = chain.send_message(message).await?;

            info!("performed transfer of {transfer_amount} tokens");

            info!("response: {:?}", response);

            let erc20_events: Vec<Erc20Event> =
                event_encoding.filter_decode_events(&response.events)?;

            info!(
                "events from sending transfer token message: {:?}",
                erc20_events
            );

            match &erc20_events[0] {
                Erc20Event::Transfer(transfer) => {
                    assert_eq!(transfer.from, sender_address);
                    assert_eq!(transfer.to, recipient_address);
                    assert_eq!(transfer.value, transfer_amount);
                }
                _ => {
                    panic!("expected a Transfer event to be emitted");
                }
            }

            let sender_balance_b = chain
                .query_token_balance(&token_address, &sender_address)
                .await?;

            info!("sender balance after transfer: {}", sender_balance_b);

            let recipient_balance_b = chain
                .query_token_balance(&token_address, &recipient_address)
                .await?;

            info!("recipient balance transfer: {}", recipient_balance_b);

            assert_eq!(
                sender_balance_b.quantity,
                sender_balance_a.quantity - transfer_amount
            );
            assert_eq!(
                recipient_balance_b.quantity,
                recipient_balance_a.quantity + transfer_amount
            );
        }

        {
            let total_supply_key = selector!("ERC20_total_supply");

            let storage_proof = chain
                .query_storage_proof(
                    &chain.query_chain_height().await?,
                    &token_address,
                    &[total_supply_key],
                )
                .await?;

            let storage_proof_str = serde_json::to_string_pretty(&storage_proof)?;

            info!(
                total_supply = %initial_supply,
                selector = %total_supply_key.to_hex_string(),
                storage_proof = %storage_proof_str,
                "gotten storage proof for total supply"
            );
        }

        {
            let address = sender_address.0;
            let balance_key = pedersen_hash(&base_balance_key, &address);

            let storage_proof = chain
                .query_storage_proof(
                    &chain.query_chain_height().await?,
                    &token_address,
                    &[balance_key],
                )
                .await?;

            let storage_proof_str = serde_json::to_string_pretty(&storage_proof)?;

            info!(
                address = %address.to_hex_string(),
                selector = %balance_key.to_hex_string(),
                storage_proof = %storage_proof_str,
                "gotten storage proof for sender's balance"
            );
        }

        {
            let address = recipient_address.0;
            let balance_key = pedersen_hash(&base_balance_key, &address);

            let storage_proof = chain
                .query_storage_proof(
                    &chain.query_chain_height().await?,
                    &token_address,
                    &[balance_key],
                )
                .await?;

            let storage_proof_str = serde_json::to_string_pretty(&storage_proof)?;

            info!(
                address = %address.to_hex_string(),
                selector = %balance_key.to_hex_string(),
                storage_proof = %storage_proof_str,
                "gotten storage proof for recipient's balance"
            );
        }

        Ok(())
    })
}
