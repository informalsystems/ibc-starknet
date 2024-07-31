use core::time::Duration;
use std::sync::Arc;

use hermes_cosmos_integration_tests::init::init_test_runtime;
use hermes_error::types::Error;
use hermes_runtime_components::traits::sleep::CanSleep;
use hermes_starknet_chain_components::traits::contract::call::CanCallContract;
use hermes_starknet_chain_components::traits::contract::invoke::CanInvokeContract;
use hermes_starknet_chain_components::traits::queries::token_balance::CanQueryTokenBalance;
use hermes_starknet_chain_context::contexts::chain::StarknetChain;
use starknet::accounts::{ExecutionEncoding, SingleOwnerAccount};
use starknet::macros::{felt, selector};
use starknet::providers::jsonrpc::HttpTransport;
use starknet::providers::{JsonRpcClient, Provider};
use starknet::signers::{LocalWallet, SigningKey};
use url::Url;

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
            let json_rpc_url = Url::try_from("http://localhost:5050/")?;

            let signing_key = felt!("0x71d7bb07b9a64f6f78ac4c816aff4da9");

            let account_address =
                felt!("0x64b48806902a367c8598f4f95c305e8c1a1acba5f082d294a43793113115691");

            let rpc_client = Arc::new(JsonRpcClient::new(HttpTransport::new(json_rpc_url)));

            let chain_id = rpc_client.chain_id().await?;

            let account = SingleOwnerAccount::new(
                rpc_client.clone(),
                LocalWallet::from_signing_key(SigningKey::from_secret_scalar(signing_key)),
                account_address,
                chain_id,
                ExecutionEncoding::New,
            );

            let chain = StarknetChain {
                rpc_client,
                account,
            };

            /*
               Test running a query that is equivalent to the following starkli call:

               starkli call \
                   0x49D36570D4E46F48E99674BD3FCC84644DDD6B96F7C741B1562B82F9E004DC7 \
                   balanceOf \
                   0x78662e7352d062084b0010068b99288486c2d8b914f6e2a55ce945f8792c8b1
            */

            let balace_a1 = chain
                .query_token_balance(
                    &felt!("0x04718f5a0fc34cc1af16a1cdee98ffb20c31f5cd61d6ab07201858f4287c938d"),
                    &felt!("0x78662e7352d062084b0010068b99288486c2d8b914f6e2a55ce945f8792c8b1"),
                )
                .await?;

            println!("balance A1: {}", balace_a1);

            let tx_hash = chain
                .invoke_contract(
                    &felt!("0x04718f5a0fc34cc1af16a1cdee98ffb20c31f5cd61d6ab07201858f4287c938d"),
                    &selector!("transfer"),
                    &vec![
                        felt!("0x78662e7352d062084b0010068b99288486c2d8b914f6e2a55ce945f8792c8b1"),
                        felt!("0x100"),
                        felt!("0x0"),
                    ],
                )
                .await?;

            println!("invoke result tx hash: {}", tx_hash);

            runtime.sleep(Duration::from_secs(1)).await;

            let balace_a2 = chain
                .query_token_balance(
                    &felt!("0x04718f5a0fc34cc1af16a1cdee98ffb20c31f5cd61d6ab07201858f4287c938d"),
                    &felt!("0x78662e7352d062084b0010068b99288486c2d8b914f6e2a55ce945f8792c8b1"),
                )
                .await?;

            println!("balance A2: {}", balace_a2);

            <Result<(), Error>>::Ok(())
        })
        .unwrap();
}
