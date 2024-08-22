use std::time::SystemTime;

use hermes_cosmos_integration_tests::init::init_test_runtime;
use hermes_encoding_components::traits::encode::CanEncode;
use hermes_encoding_components::traits::has_encoding::HasEncoding;
use hermes_error::types::Error;
use hermes_relayer_components::chain::traits::send_message::CanSendSingleMessage;
use hermes_runtime_components::traits::fs::read_file::CanReadFileAsString;
use hermes_starknet_chain_components::traits::contract::declare::CanDeclareContract;
use hermes_starknet_chain_components::traits::contract::deploy::CanDeployContract;
use hermes_starknet_chain_components::traits::event::CanParseEvents;
use hermes_starknet_chain_components::traits::messages::transfer::CanBuildTransferTokenMessage;
use hermes_starknet_chain_components::traits::queries::token_balance::CanQueryTokenBalance;
use hermes_starknet_chain_components::types::amount::StarknetAmount;
use hermes_starknet_chain_components::types::events::erc20::Erc20Event;
use hermes_starknet_chain_components::types::messages::erc20::deploy::DeployErc20TokenMessage;
use hermes_starknet_integration_tests::contexts::bootstrap::StarknetBootstrap;
use hermes_test_components::bootstrap::traits::chain::CanBootstrapChain;

#[test]
fn test_erc20_transfer() {
    let runtime = init_test_runtime();

    runtime
        .runtime
        .clone()
        .block_on(async move {
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

            let chain_driver = bootstrap.bootstrap_chain("starknet").await?;

            let chain = &chain_driver.chain;

            let token_address = {
                // Test deployment of ERC20 contract
                let contract_path = std::env::var("ERC20_CONTRACT")?;

                let contract_str = runtime.read_file_as_string(&contract_path.into()).await?;

                let contract = serde_json::from_str(&contract_str)?;

                let class_hash = chain.declare_contract(&contract).await?;

                println!("declared class: {:?}", class_hash);

                let relayer_address = chain_driver.relayer_wallet.account_address;

                let deploy_message = DeployErc20TokenMessage {
                    name: "token".into(),
                    symbol: "token".into(),
                    fixed_supply: 1000u32.into(),
                    recipient: relayer_address,
                    owner: relayer_address,
                };

                let calldata = chain.encoding().encode(&deploy_message)?;

                let token_address = chain.deploy_contract(&class_hash, false, &calldata).await?;

                println!("deployed ERC20 contract to address: {:?}", token_address);

                let balance = chain
                    .query_token_balance(&token_address, &relayer_address)
                    .await?;

                println!("initial balance: {}", balance);

                assert_eq!(balance.quantity, 1000u32.into());

                token_address
            };

            {
                // Test local ERC20 token transfer
                let account_address = chain_driver.relayer_wallet.account_address;

                let recipient_address = chain_driver.user_wallet_a.account_address;

                println!("sender address: {:?}", account_address);
                println!("recipient address: {:?}", recipient_address);

                let sender_balance_a = chain
                    .query_token_balance(&token_address, &account_address)
                    .await?;

                println!("sender balance before: {}", sender_balance_a);

                let recipient_balance_a = chain
                    .query_token_balance(&token_address, &recipient_address)
                    .await?;

                println!("recipient balance before: {}", recipient_balance_a);

                let transfer_amount = 100u32.into();

                let message = chain.build_transfer_token_message(
                    &recipient_address,
                    &StarknetAmount::new(transfer_amount, token_address),
                )?;

                let events = chain.send_message(message).await?;

                println!("performed transfer of 100 tokens");

                let erc20_events: Vec<Erc20Event> = chain.parse_events(&events)?;

                println!(
                    "events from sending transfer token message: {:?}",
                    erc20_events
                );

                match &erc20_events[0] {
                    Erc20Event::Transfer(transfer) => {
                        assert_eq!(transfer.from, account_address);
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

                println!("sender balance after transfer: {}", sender_balance_b);

                let recipient_balance_b = chain
                    .query_token_balance(&token_address, &recipient_address)
                    .await?;

                println!("recipient balance transfer: {}", recipient_balance_b);

                assert_eq!(
                    sender_balance_b.quantity,
                    sender_balance_a.quantity - transfer_amount
                );
                assert_eq!(
                    recipient_balance_b.quantity,
                    recipient_balance_a.quantity + transfer_amount
                );
            }

            <Result<(), Error>>::Ok(())
        })
        .unwrap();
}
