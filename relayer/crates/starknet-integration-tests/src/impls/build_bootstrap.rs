use alloc::collections::BTreeMap;
use alloc::sync::Arc;
use std::sync::OnceLock;

use cgp::extra::runtime::HasRuntime;
use futures::lock::Mutex;
use hermes_core::runtime_components::traits::{CanCreateDir, CanSleep, CanWriteStringToFile};
use hermes_core::test_components::chain::traits::HasWalletType;
use hermes_core::test_components::chain_driver::traits::HasChainType;
use hermes_core::test_components::driver::traits::HasChainDriverType;
use hermes_cosmos::chain_components::types::Secp256k1KeyPair;
use hermes_cosmos::runtime::types::error::TokioRuntimeError;
use hermes_cosmos::runtime::types::runtime::HermesRuntime;
use hermes_cosmos::test_components::bootstrap::traits::{
    ChainDriverBuilder, ChainDriverBuilderComponent, HasChainGenesisConfigType,
    HasChainNodeConfigType, HasChainStoreDir,
};
use hermes_prelude::*;
use hermes_starknet_chain_components::types::StarknetWallet;
use hermes_starknet_chain_context::contexts::{StarknetChain, StarknetChainFields};
use hermes_starknet_test_components::types::genesis_config::StarknetGenesisConfig;
use hermes_starknet_test_components::types::node_config::StarknetNodeConfig;
use ibc::core::host::types::error::IdentifierError;
use starknet::accounts::{ExecutionEncoding, SingleOwnerAccount};
use starknet::providers::jsonrpc::HttpTransport;
use starknet::providers::{JsonRpcClient, Provider, ProviderError};
use starknet::signers::{LocalWallet, SigningKey};
use tokio::process::Child;
use url::{ParseError, Url};

use crate::contexts::chain_driver::StarknetChainDriver;

#[cgp_new_provider(ChainDriverBuilderComponent)]
impl<Bootstrap> ChainDriverBuilder<Bootstrap> for BuildStarknetChainDriver
where
    Bootstrap: HasRuntime<Runtime = HermesRuntime>
        + HasChainStoreDir
        + HasChainDriverType<ChainDriver = StarknetChainDriver>
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
    ) -> Result<StarknetChainDriver, Bootstrap::Error> {
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

        let starknet_client = Arc::new(JsonRpcClient::new(HttpTransport::new(json_rpc_url)));

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

        let account = SingleOwnerAccount::new(
            starknet_client.clone(),
            LocalWallet::from_signing_key(SigningKey::from_secret_scalar(
                relayer_wallet.signing_key,
            )),
            *relayer_wallet.account_address,
            chain_id,
            ExecutionEncoding::New,
        );

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

        let chain = StarknetChain {
            fields: Arc::new(StarknetChainFields {
                runtime: runtime.clone(),
                chain_id: chain_id
                    .to_string()
                    .parse()
                    .map_err(Bootstrap::raise_error)?,
                starknet_client,
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

        let chain_driver = StarknetChainDriver {
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
