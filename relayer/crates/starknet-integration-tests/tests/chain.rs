use hermes_cosmos_integration_tests::init::init_test_runtime;
use hermes_error::types::Error;
use hermes_starknet_chain_components::traits::contract::call::CanCallContract;
use hermes_starknet_chain_context::contexts::chain::StarknetChain;
use starknet::macros::{felt, selector};

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
            let chain = StarknetChain::new("http://localhost:5050/".try_into().unwrap());

            /*
               Test running a query that is equivalent to the following starkli call:

               starkli call \
                   0x49D36570D4E46F48E99674BD3FCC84644DDD6B96F7C741B1562B82F9E004DC7 \
                   balanceOf \
                   0x78662e7352d062084b0010068b99288486c2d8b914f6e2a55ce945f8792c8b1
            */

            let result = chain
                .call_contract(
                    &felt!("0x04718f5a0fc34cc1af16a1cdee98ffb20c31f5cd61d6ab07201858f4287c938d"),
                    &selector!("balance_of"),
                    &vec![felt!(
                        "0x78662e7352d062084b0010068b99288486c2d8b914f6e2a55ce945f8792c8b1"
                    )],
                )
                .await?;

            println!("query balance_of result: {:?}", result);

            <Result<(), Error>>::Ok(())
        })
        .unwrap();
}
