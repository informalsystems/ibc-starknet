use std::time::SystemTime;

use hermes_cosmos_integration_tests::init::init_test_runtime;
use hermes_encoding_components::traits::encoder::CanEncode;
use hermes_encoding_components::traits::has_encoding::HasEncoding;
use hermes_error::types::Error;
use hermes_relayer_components::chain::traits::send_message::CanSendSingleMessage;
use hermes_runtime_components::traits::fs::read_file::CanReadFileAsString;
use hermes_starknet_chain_components::traits::contract::declare::CanDeclareContract;
use hermes_starknet_chain_components::traits::contract::deploy::CanDeployContract;
use hermes_starknet_chain_components::types::messages::ibc::denom::{Denom, PrefixedDenom};
use hermes_starknet_chain_components::types::messages::ibc::height::Height;
use hermes_starknet_chain_components::types::messages::ibc::ibc_transfer::IbcTransferMessage;
use hermes_starknet_chain_components::types::messages::ibc::packet::Packet;
use hermes_starknet_integration_tests::contexts::bootstrap::StarknetBootstrap;
use hermes_test_components::bootstrap::traits::chain::CanBootstrapChain;
use starknet::accounts::Call;
use starknet::macros::selector;

#[test]
fn test_starknet_ics20_contract() {
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

            let encoding = chain.encoding();

            let erc20_class_hash = {
                let contract_path = std::env::var("ERC20_CONTRACT")?;

                let contract_str = runtime.read_file_as_string(&contract_path.into()).await?;

                let contract = serde_json::from_str(&contract_str)?;

                let class_hash = chain.declare_contract(&contract).await?;

                println!("declared ERC20 class: {:?}", class_hash);

                class_hash
            };

            let ics20_class_hash = {
                let contract_path = std::env::var("ICS20_CONTRACT")?;

                let contract_str = runtime.read_file_as_string(&contract_path.into()).await?;

                let contract = serde_json::from_str(&contract_str)?;

                let class_hash = chain.declare_contract(&contract).await?;

                println!("declared ICS20 class: {:?}", class_hash);

                class_hash
            };

            let ics20_contract_address = {
                let calldata = encoding.encode(&erc20_class_hash)?;

                let contract_address = chain
                    .deploy_contract(&ics20_class_hash, false, &calldata)
                    .await?;

                println!("deployed ICS20 contract to address: {:?}", contract_address);

                contract_address
            };

            {
                let account_address = chain_driver.relayer_wallet.account_address;

                let recipient_address = chain_driver.user_wallet_a.account_address;

                let transfer_message = IbcTransferMessage {
                    denom: PrefixedDenom {
                        trace_path: Vec::new(),
                        base: Denom::Hosted("uatom".into()),
                    },
                    amount: 99u32.into(),
                    sender: account_address,
                    receiver: recipient_address,
                    memo: "".into(),
                };

                let packet_data = encoding.encode(&transfer_message)?;

                let packet = Packet {
                    sequence: 1,
                    src_port_id: "transfer".into(),
                    src_channel_id: "channel-1".into(),
                    dst_port_id: "transfer".into(),
                    dst_channel_id: "channel-1".into(),
                    data: packet_data,
                    timeout_height: Height {
                        revision_number: 0,
                        revision_height: 100,
                    },
                    timeout_timestamp: 0,
                };

                let calldata = encoding.encode(&packet)?;

                let message = Call {
                    to: ics20_contract_address,
                    selector: selector!("recv_execute"),
                    calldata,
                };

                let events = chain.send_message(message).await?;

                println!("recv_execute events: {:?}", events);
            }

            <Result<(), Error>>::Ok(())
        })
        .unwrap();
}
