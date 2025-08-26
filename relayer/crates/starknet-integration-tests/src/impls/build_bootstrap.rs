use alloc::collections::BTreeMap;
use alloc::sync::Arc;
use core::time::Duration;
use std::env::var;
use std::sync::OnceLock;

use cgp::extra::runtime::HasRuntime;
use futures::lock::Mutex;
use hermes_core::chain_type_components::impls::BatchConfig;
use hermes_core::runtime_components::traits::{CanCreateDir, CanSleep, CanWriteStringToFile};
use hermes_core::test_components::chain::traits::HasWalletType;
use hermes_core::test_components::chain_driver::traits::{HasChainCommandPath, HasChainType};
use hermes_core::test_components::driver::traits::HasChainDriverType;
use hermes_cosmos::runtime::types::error::TokioRuntimeError;
use hermes_cosmos::runtime::types::runtime::HermesRuntime;
use hermes_cosmos_core::chain_components::types::Secp256k1KeyPair;
use hermes_cosmos_core::test_components::bootstrap::traits::{
    ChainDriverBuilder, ChainDriverBuilderComponent, HasChainGenesisConfigType,
    HasChainNodeConfigType, HasChainStoreDir,
};
use hermes_prelude::*;
use hermes_starknet_chain_components::impls::{
    StarknetChainConfig, StarknetContractAddresses, StarknetContractClasses,
};
use hermes_starknet_chain_components::types::StarknetWallet;
use hermes_starknet_chain_context::contexts::{StarknetChain, StarknetChainFields};
use hermes_starknet_test_components::types::{StarknetGenesisConfig, StarknetNodeConfig};
use ibc::core::host::types::error::IdentifierError;
use starknet::providers::jsonrpc::HttpTransport;
use starknet::providers::{JsonRpcClient, Provider, ProviderError};
use tokio::process::Child;
use url::{ParseError, Url};

use crate::contexts::StarknetChainDriver;

#[cgp_new_provider(ChainDriverBuilderComponent)]
impl<Bootstrap> ChainDriverBuilder<Bootstrap> for BuildStarknetChainDriver
where
    Bootstrap: HasRuntime<Runtime = HermesRuntime>
        + HasChainStoreDir
        + HasChainDriverType<ChainDriver = StarknetChainDriver>
        + HasChainType
        + HasChainGenesisConfigType<ChainGenesisConfig = StarknetGenesisConfig>
        + HasChainNodeConfigType<ChainNodeConfig = StarknetNodeConfig>
        + HasChainCommandPath
        + CanRaiseAsyncError<TokioRuntimeError>
        + CanRaiseAsyncError<ProviderError>
        + CanRaiseAsyncError<ParseError>
        + CanRaiseAsyncError<IdentifierError>
        + CanRaiseAsyncError<toml::ser::Error>
        + CanRaiseAsyncError<&'static str>,
    Bootstrap::Chain: HasWalletType<Wallet = StarknetWallet>,
{
    async fn build_chain_driver(
        bootstrap: &Bootstrap,
        genesis_config: StarknetGenesisConfig,
        node_config: StarknetNodeConfig,
        wallets: BTreeMap<String, StarknetWallet>,
        chain_processes: Vec<Child>,
    ) -> Result<StarknetChainDriver, Bootstrap::Error> {
        let runtime = bootstrap.runtime().clone();

        let chain_store_dir = bootstrap.chain_store_dir().clone();

        let chain_command_path = bootstrap.chain_command_path().clone();

        runtime
            .create_dir(&chain_store_dir.join("wallets"))
            .await
            .map_err(Bootstrap::raise_error)?;

        for (name, wallet) in wallets.iter() {
            let wallet_str = toml::to_string_pretty(wallet).map_err(Bootstrap::raise_error)?;
            let wallet_path = chain_store_dir.join(format!("wallets/{name}.toml"));

            runtime
                .write_string_to_file(&wallet_path, &wallet_str)
                .await
                .map_err(Bootstrap::raise_error)?;
        }

        let relayer_wallet_1 = wallets
            .get("relayer")
            .ok_or_else(|| Bootstrap::raise_error("expect relayer wallet to be present"))?
            .clone();

        let relayer_wallet_2 = wallets
            .get("relayer-2")
            .ok_or_else(|| Bootstrap::raise_error("expect relayer-2 wallet to be present"))?
            .clone();

        let user_wallet_a = wallets
            .get("user-a")
            .ok_or_else(|| Bootstrap::raise_error("expect user A wallet to be present"))?
            .clone();

        let user_wallet_b = wallets
            .get("user-b")
            .ok_or_else(|| Bootstrap::raise_error("expect user B wallet to be present"))?
            .clone();

        let json_rpc_url = Url::parse(&format!(
            "http://{}:{}/",
            node_config.rpc_addr, node_config.rpc_port
        ))
        .map_err(Bootstrap::raise_error)?;

        let starknet_client =
            Arc::new(JsonRpcClient::new(HttpTransport::new(json_rpc_url.clone())));

        let rpc_client = ureq::agent();

        let feeder_gateway_url = Url::parse(&format!(
            "http://{}:{}/",
            node_config.rpc_addr,
            node_config.rpc_port + 1
        ))
        .map_err(Bootstrap::raise_error)?;

        // Wait for the chain to be ready.
        for _ in 0..10 {
            match starknet_client.block_number().await {
                Ok(_) => break,
                Err(_) => runtime.sleep(core::time::Duration::from_secs(1)).await,
            }
        }

        let chain_id = starknet_client
            .chain_id()
            .await
            .map_err(Bootstrap::raise_error)?;

        let proof_signer = Secp256k1KeyPair::from_mnemonic(
            bip39::Mnemonic::from_entropy(
                &relayer_wallet_1.signing_key.to_bytes_be(),
                bip39::Language::English,
            )
            .expect("valid mnemonic")
            .phrase(),
            &"m/84'/0'/0'/0/0".parse().expect("valid hdpath"),
            "strk",
        )
        .expect("valid key pair");

        let client_refresh_rate = var("STARKNET_REFRESH_RATE")
            .map(|refresh_str| {
                Duration::from_secs(
                    refresh_str
                        .parse::<u64>()
                        .expect("failed to parse {refresh_str} to seconds"),
                )
            })
            .ok();
        let relayer_wallet_path_1 = chain_store_dir
            .join("wallets/relayer.toml")
            .display()
            .to_string();

        let relayer_wallet_path_2 = chain_store_dir
            .join("wallets/relayer-2.toml")
            .display()
            .to_string();

        let contract_classes = StarknetContractClasses {
            erc20: None,
            ics20: None,
            ibc_client: None,
        };
        let contract_addresses = StarknetContractAddresses {
            ibc_client: None,
            ibc_core: None,
            ibc_ics20: None,
        };

        let block_time = core::time::Duration::from_secs(1);
        let poll_interval = core::time::Duration::from_millis(200);

        let ed25519_attestator_addresses = var("ED25519_ATTESTATORS")
            .map(|attestator_list| {
                attestator_list
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .collect::<Vec<_>>()
            })
            .ok();

        let chain_config = StarknetChainConfig {
            json_rpc_url: format!("http://{}:{}/", node_config.rpc_addr, node_config.rpc_port),
            feeder_gateway_url: format!(
                "http://{}:{}/",
                node_config.rpc_addr,
                node_config.rpc_port + 1
            ),
            relayer_wallet_1: relayer_wallet_path_1,
            relayer_wallet_2: relayer_wallet_path_2,
            ed25519_attestator_addresses,
            poll_interval,
            block_time,
            contract_addresses,
            contract_classes,
            batch_config: Some(BatchConfig {
                max_message_count: 300,
                max_tx_size: 1000000,
                buffer_size: 1000000,
                max_delay: Duration::from_secs(1),
                sleep_time: Duration::from_millis(100),
            }),
        };

        let chain = StarknetChain {
            fields: Arc::new(StarknetChainFields {
                runtime: runtime.clone(),
                chain_id: chain_id
                    .to_string()
                    .parse()
                    .map_err(Bootstrap::raise_error)?,
                ed25519_attestator_addresses: chain_config.ed25519_attestator_addresses.clone(),
                chain_config,
                starknet_client,
                rpc_client,
                json_rpc_url,
                feeder_gateway_url,
                ibc_client_contract_address: OnceLock::new(),
                ibc_core_contract_address: OnceLock::new(),
                ibc_ics20_contract_address: OnceLock::new(),
                event_encoding: Default::default(),
                poll_interval,
                block_time,
                nonce_mutex: Arc::new(Mutex::new(())),
                signers: vec![relayer_wallet_1.clone(), relayer_wallet_2.clone()],
                client_refresh_rate,
                signer_mutex: Arc::new(Mutex::new(0)),
            }),
        };

        let chain_driver = StarknetChainDriver {
            runtime,
            chain,
            chain_store_dir,
            chain_command_path,
            genesis_config,
            node_config,
            wallets,
            chain_processes,
            relayer_wallet_1,
            relayer_wallet_2,
            user_wallet_a,
            user_wallet_b,
        };

        Ok(chain_driver)
    }
}
