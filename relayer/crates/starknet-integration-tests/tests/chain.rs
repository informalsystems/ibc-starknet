use hermes_cosmos_integration_tests::init::init_test_runtime;
use hermes_error::types::Error;
use hermes_relayer_components::chain::traits::send_message::CanSendMessages;
use hermes_starknet_chain_components::traits::messages::transfer::CanBuildTransferTokenMessage;
use hermes_starknet_chain_components::traits::queries::token_balance::CanQueryTokenBalance;
use hermes_starknet_chain_components::types::amount::StarknetAmount;
use hermes_starknet_integration_tests::contexts::bootstrap::StarknetBootstrap;
use hermes_test_components::bootstrap::traits::chain::CanBootstrapChain;
use starknet::macros::felt;

// Note: the test needs to be run with starknet-devnet-rs with the seed 0:
//
// $ starknet-devnet --seed 0
#[test]
fn test_starknet_chain_client() {
    let runtime = init_test_runtime();

    runtime
        .runtime
        .clone()
        .block_on(async move {
            let chain_command_path = std::env::var("STARKNET_BIN")
                .unwrap_or("starknet-devnet".into())
                .into();

            let bootstrap = StarknetBootstrap {
                runtime: runtime.clone(),
                chain_command_path,
                chain_store_dir: "./test-data".into(),
            };

            let chain_driver = bootstrap.bootstrap_chain("starknet").await?;

            let chain = &chain_driver.chain;

            let account_address =
                felt!("0x64b48806902a367c8598f4f95c305e8c1a1acba5f082d294a43793113115691");

            let token_address =
                felt!("0x49D36570D4E46F48E99674BD3FCC84644DDD6B96F7C741B1562B82F9E004DC7");

            let recipient_address =
                felt!("0x78662e7352d062084b0010068b99288486c2d8b914f6e2a55ce945f8792c8b1");

            let sender_balance_a = chain
                .query_token_balance(&token_address, &account_address)
                .await?;

            println!("sender balance before: {}", sender_balance_a);

            let recipient_balance_a = chain
                .query_token_balance(&token_address, &recipient_address)
                .await?;

            println!("recipient balance before: {}", recipient_balance_a);

            let message = chain.build_transfer_token_message(
                &recipient_address,
                &StarknetAmount::new(100u32.into(), token_address),
            );

            let events = chain.send_messages(vec![message]).await?;

            println!("events from sending transfer token message: {:?}", events);

            println!("performed transfer of 100 ETH");

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
                sender_balance_a.quantity - 100u32.into()
            );
            assert_eq!(
                recipient_balance_b.quantity,
                recipient_balance_a.quantity + 100u32.into()
            );

            <Result<(), Error>>::Ok(())
        })
        .unwrap();
}
