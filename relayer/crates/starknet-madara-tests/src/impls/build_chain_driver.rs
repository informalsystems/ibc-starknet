use alloc::collections::BTreeMap;
use alloc::sync::Arc;
use std::sync::OnceLock;

use cgp::extra::runtime::HasRuntime;
use cgp::prelude::*;
use futures::lock::Mutex;
use hermes_cosmos_chain_components::types::key_types::secp256k1::Secp256k1KeyPair;
use hermes_cosmos_test_components::bootstrap::traits::chain::build_chain_driver::{
    ChainDriverBuilder, ChainDriverBuilderComponent,
};
use hermes_cosmos_test_components::bootstrap::traits::fields::chain_store_dir::HasChainStoreDir;
use hermes_cosmos_test_components::bootstrap::traits::types::chain_node_config::HasChainNodeConfigType;
use hermes_cosmos_test_components::bootstrap::traits::types::genesis_config::HasChainGenesisConfigType;
use hermes_runtime::types::error::TokioRuntimeError;
use hermes_runtime::types::runtime::HermesRuntime;
use hermes_runtime_components::traits::fs::create_dir::CanCreateDir;
use hermes_runtime_components::traits::fs::write_file::CanWriteStringToFile;
use hermes_runtime_components::traits::sleep::CanSleep;
use hermes_starknet_chain_components::types::wallet::StarknetWallet;
use hermes_starknet_test_components::types::genesis_config::StarknetGenesisConfig;
use hermes_starknet_test_components::types::node_config::StarknetNodeConfig;
use hermes_test_components::chain::traits::types::wallet::HasWalletType;
use hermes_test_components::chain_driver::traits::types::chain::HasChainType;
use hermes_test_components::driver::traits::types::chain_driver::HasChainDriverType;
use ibc::core::host::types::error::IdentifierError;
use reqwest::Client;
use starknet_v13::providers::jsonrpc::HttpTransport;
use starknet_v13::providers::{JsonRpcClient, Provider, ProviderError};
use tokio::process::Child;
use url::{ParseError, Url};

use crate::contexts::{MadaraChain, MadaraChainDriver, MadaraChainFields};

#[cgp_new_provider(ChainDriverBuilderComponent)]
impl<Bootstrap> ChainDriverBuilder<Bootstrap> for BuildMadaraChainDriver
where
    Bootstrap: HasRuntime<Runtime = HermesRuntime>
        + HasChainStoreDir
        + HasChainDriverType<ChainDriver = MadaraChainDriver>
        + HasChainType
        + HasChainGenesisConfigType<ChainGenesisConfig = StarknetGenesisConfig>
        + HasChainNodeConfigType<ChainNodeConfig = StarknetNodeConfig>
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
    ) -> Result<MadaraChainDriver, Bootstrap::Error> {
        let runtime = bootstrap.runtime().clone();

        let chain_store_dir = bootstrap.chain_store_dir().clone();

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

        let relayer_wallet = wallets
            .get("relayer")
            .ok_or_else(|| Bootstrap::raise_error("expect relayer wallet to be present"))?
            .clone();

        let user_wallet_a = wallets
            .get("user-a")
            .ok_or_else(|| Bootstrap::raise_error("expect relayer wallet to be present"))?
            .clone();

        let user_wallet_b = wallets
            .get("user-b")
            .ok_or_else(|| Bootstrap::raise_error("expect relayer wallet to be present"))?
            .clone();

        let json_rpc_url = Url::parse(&format!(
            "http://{}:{}/",
            node_config.rpc_addr, node_config.rpc_port
        ))
        .map_err(Bootstrap::raise_error)?;

        let starknet_client =
            Arc::new(JsonRpcClient::new(HttpTransport::new(json_rpc_url.clone())));

        let rpc_client = Client::new();

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
                &relayer_wallet.signing_key.to_bytes_be(),
                bip39::Language::English,
            )
            .expect("valid mnemonic")
            .phrase(),
            &"m/84'/0'/0'/0/0".parse().expect("valid hdpath"),
            "strk",
        )
        .expect("valid key pair");

        let chain = MadaraChain {
            fields: Arc::new(MadaraChainFields {
                runtime: runtime.clone(),
                chain_id: chain_id
                    .to_string()
                    .parse()
                    .map_err(Bootstrap::raise_error)?,
                starknet_client,
                rpc_client,
                json_rpc_url,
                ibc_client_contract_address: OnceLock::new(),
                ibc_core_contract_address: OnceLock::new(),
                ibc_ics20_contract_address: OnceLock::new(),
                event_encoding: Default::default(),
                proof_signer,
                poll_interval: core::time::Duration::from_millis(200),
                block_time: core::time::Duration::from_secs(1),
                nonce_mutex: Arc::new(Mutex::new(())),
                signer: relayer_wallet.clone(),
            }),
        };

        let chain_driver = MadaraChainDriver {
            runtime,
            chain,
            chain_store_dir,
            genesis_config,
            node_config,
            wallets,
            chain_processes,
            relayer_wallet,
            user_wallet_a,
            user_wallet_b,
        };

        Ok(chain_driver)
    }
}
