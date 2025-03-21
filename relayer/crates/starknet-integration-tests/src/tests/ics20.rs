use core::marker::PhantomData;
use std::time::SystemTime;

use cgp::extra::run::CanRun;
use cgp::prelude::*;
use hermes_chain_components::traits::queries::chain_status::CanQueryChainStatus;
use hermes_cosmos_chain_components::types::config::gas::dynamic_gas_config::DynamicGasConfig;
use hermes_cosmos_chain_components::types::config::gas::eip_type::EipQueryType;
use hermes_cosmos_integration_tests::init::init_test_runtime;
use hermes_cosmos_relayer::contexts::build::CosmosBuilder;
use hermes_cosmos_relayer::contexts::chain::CosmosChain;
use hermes_cosmos_test_components::chain::impls::transfer::amount::derive_ibc_denom;
use hermes_cosmos_test_components::chain::types::amount::Amount;
use hermes_cosmos_test_components::chain::types::denom::Denom as IbcDenom;
use hermes_encoding_components::traits::decode::CanDecode;
use hermes_encoding_components::traits::encode::CanEncode;
use hermes_error::types::Error;
use hermes_relayer_components::chain::traits::send_message::CanSendSingleMessage;
use hermes_starknet_chain_components::impls::types::address::StarknetAddress;
use hermes_starknet_chain_components::impls::types::message::StarknetMessage;
use hermes_starknet_chain_components::traits::contract::call::CanCallContract;
use hermes_starknet_chain_components::traits::queries::token_balance::CanQueryTokenBalance;
use hermes_starknet_chain_components::types::amount::StarknetAmount;
use hermes_starknet_chain_components::types::messages::ibc::denom::{
    Denom, PrefixedDenom, TracePrefix,
};
use hermes_starknet_chain_context::contexts::chain::StarknetChain;
use hermes_starknet_chain_context::contexts::encoding::cairo::StarknetCairoEncoding;
use hermes_starknet_relayer::contexts::builder::StarknetBuilder;
use hermes_test_components::chain::traits::assert::eventual_amount::CanAssertEventualAmount;
use hermes_test_components::chain::traits::queries::balance::CanQueryBalance;
use hermes_test_components::chain::traits::transfer::ibc_transfer::CanIbcTransferToken;
use hermes_test_components::setup::traits::driver::CanBuildTestDriver;
use ibc::core::host::types::identifiers::PortId as IbcPortId;
use poseidon::Poseidon3Hasher;
use starknet::core::types::Call;
use starknet::macros::selector;
use tracing::info;

use crate::contexts::osmosis_bootstrap::OsmosisBootstrap;
use crate::contexts::setup::StarknetTestSetup;
use crate::contexts::test_driver::StarknetTestDriver;
use crate::utils::{init_starknet_bootstrap, load_wasm_client};

#[test]
fn test_starknet_ics20_contract() -> Result<(), Error> {
    let runtime = init_test_runtime();

    runtime.runtime.clone().block_on(async move {
        let wasm_client_code_path = std::env::var("STARKNET_WASM_CLIENT_PATH")
            .expect("Wasm blob for Starknet light client is required");

        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)?
            .as_secs();

        let starknet_bootstrap = init_starknet_bootstrap(&runtime).await?;

        let (wasm_code_hash, wasm_client_byte_code) =
            load_wasm_client(&wasm_client_code_path).await?;

        let cosmos_builder = CosmosBuilder::new_with_default(runtime.clone());

        let starknet_builder = StarknetBuilder::new(runtime.clone(), cosmos_builder.clone(), None);

        let cosmos_bootstrap = OsmosisBootstrap {
            runtime: runtime.clone(),
            cosmos_builder,
            should_randomize_identifiers: true,
            chain_store_dir: format!("./test-data/{timestamp}/osmosis").into(),
            chain_command_path: "osmosisd".into(),
            account_prefix: "osmo".into(),
            staking_denom_prefix: "stake".into(),
            transfer_denom_prefix: "coin".into(),
            wasm_client_byte_code,
            governance_proposal_authority: "osmo10d07y265gmmuvt4z0w9aw880jnsr700jjeq4qp".into(), // TODO: don't hard code this
            dynamic_gas: Some(DynamicGasConfig {
                multiplier: 1.1,
                max: 1.6,
                eip_query_type: EipQueryType::Osmosis,
                denom: "stake".to_owned(),
            }),
        };

        let setup = StarknetTestSetup::new_with_defaults(
            starknet_bootstrap,
            cosmos_bootstrap,
            starknet_builder,
            wasm_code_hash,
        );

        let test_driver: StarknetTestDriver = setup.build_driver().await?;

        let starknet_chain_driver = &test_driver.starknet_chain_driver;

        let cosmos_chain_driver = &test_driver.cosmos_chain_driver;

        let starknet_chain = &starknet_chain_driver.chain;

        let cosmos_chain = &cosmos_chain_driver.chain;

        let ics20_port = &test_driver.port_id_a;

        let starknet_channel_id = &test_driver.channel_id_a;

        let cosmos_channel_id = &test_driver.channel_id_b;

        {
            let birelay = test_driver.relay_driver.birelay.clone();

            runtime.runtime.spawn(async move {
                let _ = birelay.run().await;
            });
        }

        // submit ics20 transfer to Cosmos

        let wallet_cosmos_a = &cosmos_chain_driver.user_wallet_a;
        let address_cosmos_a = &wallet_cosmos_a.address;
        let wallet_starknet_b = &starknet_chain_driver.user_wallet_b;
        let address_starknet_b = &wallet_starknet_b.account_address;
        let transfer_quantity = 1_000u128;
        let denom_cosmos = &cosmos_chain_driver.genesis_config.transfer_denom;

        let balance_cosmos_a_step_0 = cosmos_chain
            .query_balance(address_cosmos_a, denom_cosmos)
            .await?;

        info!(
            "cosmos balance before transfer: {}",
            balance_cosmos_a_step_0
        );

        let denom = PrefixedDenom {
            trace_path: vec![TracePrefix {
                port_id: ics20_port.to_string(),
                channel_id: starknet_channel_id.to_string(),
            }],
            base: Denom::Hosted(denom_cosmos.to_string()),
        };

        let cairo_encoding = StarknetCairoEncoding;
        let ics20_contract_address = *starknet_chain.ibc_ics20_contract_address.get().unwrap();

        let ics20_token_address: StarknetAddress = {
            let calldata = cairo_encoding.encode(&product![denom.clone()])?;

            let message = StarknetMessage {
                call: Call {
                    to: *ics20_contract_address,
                    selector: selector!("create_ibc_token"),
                    calldata,
                },
                counterparty_height: None,
            };

            let message_response = starknet_chain.send_message(message).await?;

            cairo_encoding.decode(&message_response.result)?
        };

        info!("ics20 token address: {:?}", ics20_token_address);

        let expected_ics20_token_address: StarknetAddress = {
            let ibc_prefixed_denom = PrefixedDenom {
                trace_path: vec![TracePrefix {
                    port_id: ics20_port.to_string(),
                    channel_id: starknet_channel_id.to_string(),
                }],
                base: Denom::Hosted(denom_cosmos.to_string()),
            };

            let mut denom_serialized = vec![];

            {
                // https://github.com/informalsystems/ibc-starknet/blob/06cb7587557e6f3bef323abe7b5d9c3ab35bd97a/cairo-contracts/packages/apps/src/transfer/types.cairo#L120-L130
                for trace_prefix in &ibc_prefixed_denom.trace_path {
                    denom_serialized.extend(cairo_encoding.encode(trace_prefix)?);
                }

                denom_serialized.extend(cairo_encoding.encode(&ibc_prefixed_denom.base)?);
            }

            // https://github.com/informalsystems/ibc-starknet/blob/06cb7587557e6f3bef323abe7b5d9c3ab35bd97a/cairo-contracts/packages/utils/src/utils.cairo#L35
            let ibc_prefixed_denom_key = Poseidon3Hasher::digest(&denom_serialized);

            let calldata = cairo_encoding.encode(&product![ibc_prefixed_denom_key])?;

            let output = starknet_chain
                .call_contract(
                    &ics20_contract_address,
                    &selector!("ibc_token_address"),
                    &calldata,
                    None,
                )
                .await?;

            let token_address: Option<StarknetAddress> = cairo_encoding.decode(&output)?;

            token_address.unwrap()
        };

        assert_eq!(ics20_token_address, expected_ics20_token_address);

        let _packet = cosmos_chain
            .ibc_transfer_token(
                PhantomData::<StarknetChain>,
                cosmos_channel_id,
                &IbcPortId::transfer(),
                wallet_cosmos_a,
                address_starknet_b,
                &Amount::new(transfer_quantity, denom_cosmos.clone()),
                &None,
                &starknet_chain.query_chain_status().await?,
            )
            .await?;

        // cosmos_to_starknet_relay.relay_packet(&packet).await?;

        let balance_cosmos_a_step_1 = cosmos_chain
            .query_balance(address_cosmos_a, denom_cosmos)
            .await?;

        info!("cosmos balance after transfer: {}", balance_cosmos_a_step_1);

        assert_eq!(
            balance_cosmos_a_step_0.quantity,
            balance_cosmos_a_step_1.quantity + transfer_quantity
        );

        let balance_starknet_b_step_1 =
            StarknetAmount::new(transfer_quantity.into(), ics20_token_address);

        starknet_chain
            .assert_eventual_amount(address_starknet_b, &balance_starknet_b_step_1)
            .await?;

        let _packet = starknet_chain
            .ibc_transfer_token(
                PhantomData::<CosmosChain>,
                starknet_channel_id,
                &IbcPortId::transfer(),
                wallet_starknet_b,
                address_cosmos_a,
                &StarknetAmount::new(transfer_quantity.into(), ics20_token_address),
                &None,
                &cosmos_chain.query_chain_status().await?,
            )
            .await?;

        cosmos_chain
            .assert_eventual_amount(address_cosmos_a, &balance_cosmos_a_step_0)
            .await?;

        let balance_starknet_b_step_2 = starknet_chain
            .query_token_balance(&ics20_token_address, address_starknet_b)
            .await?;

        info!(
            "starknet balance after transfer back: {}",
            balance_starknet_b_step_2
        );

        assert_eq!(balance_starknet_b_step_2.quantity, 0u64.into());

        // send starknet erc20 token to cosmos

        let erc20_token_address = &starknet_chain_driver.genesis_config.transfer_denom;

        info!("erc20 token address: {:?}", erc20_token_address);

        let balance_starknet_step_0 = starknet_chain
            .query_token_balance(erc20_token_address, address_starknet_b)
            .await?;

        info!("erc20 balance on starknet: {}", balance_starknet_step_0);

        let packet = starknet_chain
            .ibc_transfer_token(
                PhantomData::<CosmosChain>,
                starknet_channel_id,
                &IbcPortId::transfer(),
                wallet_starknet_b,
                address_cosmos_a,
                &StarknetAmount::new(transfer_quantity.into(), *erc20_token_address),
                &None,
                &cosmos_chain.query_chain_status().await?,
            )
            .await?;

        let cosmos_ibc_denom = derive_ibc_denom(
            ics20_port,
            cosmos_channel_id,
            &IbcDenom::base(&erc20_token_address.to_string()),
        )?;

        info!("cosmos ibc denom: {:?}", cosmos_ibc_denom);

        cosmos_chain
            .assert_eventual_amount(
                address_cosmos_a,
                &Amount::new(transfer_quantity, cosmos_ibc_denom.clone()),
            )
            .await?;

        let balance_starknet_relayer_step_3 = starknet_chain
            .query_token_balance(erc20_token_address, address_starknet_b)
            .await?;

        info!(
            "starknet balance after transfer from starknet: {}",
            balance_starknet_relayer_step_3
        );

        assert_eq!(
            balance_starknet_relayer_step_3.quantity,
            balance_starknet_step_0.quantity - transfer_quantity.into()
        );

        // send the tokens back to starknet

        let _packet = cosmos_chain
            .ibc_transfer_token(
                PhantomData::<StarknetChain>,
                cosmos_channel_id,
                &IbcPortId::transfer(),
                wallet_cosmos_a,
                address_starknet_b,
                &Amount::new(transfer_quantity, cosmos_ibc_denom.clone()),
                &None,
                &starknet_chain.query_chain_status().await?,
            )
            .await?;

        let balance_cosmos_a_step_4 = cosmos_chain
            .query_balance(address_cosmos_a, &cosmos_ibc_denom)
            .await?;

        info!(
            "cosmos balance after transfer back to starknet: {}",
            balance_cosmos_a_step_4
        );

        assert_eq!(balance_cosmos_a_step_4.quantity, 0u64.into());

        starknet_chain
            .assert_eventual_amount(address_starknet_b, &balance_starknet_step_0)
            .await?;

        Ok(())
    })
}
