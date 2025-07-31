use std::time::SystemTime;

use hermes_core::chain_components::traits::CanSendSingleMessage;
use hermes_core::encoding_components::traits::CanEncode;
use hermes_core::relayer_components::transaction::impls::CanSendSingleMessageWithSigner;
use hermes_core::runtime_components::traits::CanReadFileAsString;
use hermes_core::test_components::bootstrap::traits::CanBootstrapChain;
use hermes_cosmos::error::types::Error;
use hermes_cosmos::integration_tests::init::init_test_runtime;
use hermes_starknet_chain_components::impls::{CanFilterDecodeEvents, StarknetMessage};
use hermes_starknet_chain_components::traits::{
    CanBuildTransferTokenMessage, CanDeclareContract, CanDeployContract, CanQueryTokenBalance,
};
use hermes_starknet_chain_components::types::{
    DeployErc20TokenMessage, Erc20Event, StarknetAmount,
};
use hermes_starknet_chain_context::contexts::{StarknetCairoEncoding, StarknetEventEncoding};
use starknet::core::types::U256;
use starknet::macros::selector;
use tracing::info;

use crate::utils::init_starknet_bootstrap;

#[test]
fn test_erc20_transfer() -> Result<(), Error> {
    // ### SETUP START ###
    let runtime = init_test_runtime();

    runtime.runtime.clone().block_on(async move {
        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)?
            .as_secs();

        let bootstrap = init_starknet_bootstrap(&runtime).await?;

        let chain_driver = bootstrap.bootstrap_chain("starknet").await?;

        let chain = &chain_driver.chain;

        let erc20_class_hash = {
            let contract_path = std::env::var("ERC20_CONTRACT")?;

            let contract_str = runtime.read_file_as_string(&contract_path.into()).await?;

            let contract = serde_json::from_str(&contract_str)?;

            let class_hash = chain.declare_contract(&contract).await?;

            info!("declared class: {:?}", class_hash);

            class_hash
        };

        let initial_supply = 1000u32;

        let token_address = {
            let relayer_address = chain_driver.relayer_wallet_1.account_address;

            let deploy_message = DeployErc20TokenMessage {
                name: "token".into(),
                symbol: "token".into(),
                decimals: 18,
                owner: relayer_address,
            };

            let calldata = StarknetCairoEncoding.encode(&deploy_message)?;

            let token_address = chain
                .deploy_contract(&erc20_class_hash, false, &calldata)
                .await?;

            info!("deployed ERC20 contract to address: {:?}", token_address);

            let calldata = StarknetCairoEncoding.encode(&U256::from(initial_supply))?;

            chain
                .send_message(StarknetMessage::new(
                    *token_address,
                    selector!("mint"),
                    calldata,
                ))
                .await?;

            info!("Mint {initial_supply} initial supply to address: {relayer_address}");

            let balance = chain
                .query_token_balance(&token_address, &relayer_address)
                .await?;

            assert_eq!(balance.quantity, initial_supply.into());

            token_address
        };

        {
            // Test local ERC20 token transfer
            let sender_wallet = chain_driver.relayer_wallet_1.clone();
            let sender_address = sender_wallet.account_address;

            let recipient_address = chain_driver.relayer_wallet_2.account_address;

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

            let transfer_amount = 100u32.into();

            let message = chain.build_transfer_token_message(
                &recipient_address,
                &StarknetAmount::new(transfer_amount, token_address),
            )?;

            chain
                .send_message_with_signer(&sender_wallet, message)
                .await?;

            info!("Assert {recipient_address} received {transfer_amount} tokens");

            let balance = chain
                .query_token_balance(&token_address, &recipient_address)
                .await?;

            assert_eq!(balance.quantity, transfer_amount);
        }

        let event_encoding = StarknetEventEncoding::default();

        event_encoding
            .erc20_hashes
            .set([erc20_class_hash].into())
            .unwrap();

        // ### SETUP DONE ###
        {
            // Test local ERC20 token transfer
            let account_address = chain_driver.relayer_wallet_1.account_address;

            let recipient_address = chain_driver.user_wallet_a.account_address;

            info!("sender address: {:?}", account_address);
            info!("recipient address: {:?}", recipient_address);

            let sender_balance_a = chain
                .query_token_balance(&token_address, &account_address)
                .await?;

            info!("sender balance before: {}", sender_balance_a);

            let recipient_balance_a = chain
                .query_token_balance(&token_address, &recipient_address)
                .await?;

            info!("recipient balance before: {}", recipient_balance_a);

            let transfer_amount = 100u32.into();

            let message = chain.build_transfer_token_message(
                &recipient_address,
                &StarknetAmount::new(transfer_amount, token_address),
            )?;

            let response = chain.send_message(message).await?;

            info!("performed transfer of 100 tokens");

            info!("response: {:?}", response);

            let erc20_events: Vec<Erc20Event> =
                event_encoding.filter_decode_events(&response.events)?;

            info!(
                "events from sending transfer token message: {:?}",
                erc20_events
            );

            match &erc20_events[0] {
                Erc20Event::Transfer(transfer) => {
                    assert_eq!(transfer.to, recipient_address);
                    assert_eq!(transfer.value, transfer_amount);
                }
                _ => {
                    panic!("expected a Transfer event to be emitted");
                }
            }

            let sender_balance_b = chain
                .query_token_balance(&token_address, &account_address)
                .await?;

            info!("sender balance after transfer: {}", sender_balance_b);

            let recipient_balance_b = chain
                .query_token_balance(&token_address, &recipient_address)
                .await?;

            info!("recipient balance transfer: {}", recipient_balance_b);

            assert_eq!(
                recipient_balance_b.quantity,
                recipient_balance_a.quantity + transfer_amount
            );
        }

        Ok(())
    })
}
