use std::collections::BTreeMap;
use std::path::PathBuf;
use std::sync::{Arc, OnceLock};

use cgp::core::component::UseDelegate;
use cgp::core::error::{ErrorRaiserComponent, ErrorTypeProviderComponent};
use cgp::core::field::WithField;
use cgp::core::types::WithType;
use cgp::prelude::*;
use futures::lock::Mutex;
use hermes_cosmos_chain_components::types::key_types::secp256k1::Secp256k1KeyPair;
use hermes_cosmos_test_components::bootstrap::impls::chain::build_wait::BuildAndWaitChainDriver;
use hermes_cosmos_test_components::bootstrap::traits::chain::build_chain_driver::{
    ChainDriverBuilder, ChainDriverBuilderComponent,
};
use hermes_cosmos_test_components::bootstrap::traits::chain::start_chain::ChainFullNodeStarterComponent;
use hermes_cosmos_test_components::bootstrap::traits::fields::chain_command_path::{
    ChainCommandPathGetter, ChainCommandPathGetterComponent,
};
use hermes_cosmos_test_components::bootstrap::traits::fields::chain_store_dir::{
    ChainStoreDirGetter, ChainStoreDirGetterComponent,
};
use hermes_cosmos_test_components::bootstrap::traits::types::chain_node_config::ChainNodeConfigTypeComponent;
use hermes_cosmos_test_components::bootstrap::traits::types::genesis_config::ChainGenesisConfigTypeComponent;
use hermes_error::impls::UseHermesError;
use hermes_error::types::HermesError;
use hermes_logger::UseHermesLogger;
use hermes_logging_components::traits::has_logger::{
    GlobalLoggerGetterComponent, LoggerGetterComponent, LoggerTypeProviderComponent,
};
use hermes_runtime::types::runtime::HermesRuntime;
use hermes_runtime_components::traits::fs::create_dir::CanCreateDir;
use hermes_runtime_components::traits::fs::write_file::CanWriteStringToFile;
use hermes_runtime_components::traits::runtime::{
    RuntimeGetterComponent, RuntimeTypeProviderComponent,
};
use hermes_runtime_components::traits::sleep::CanSleep;
use hermes_starknet_chain_components::types::wallet::StarknetWallet;
use hermes_starknet_chain_context::contexts::chain::{StarknetChain, StarknetChainFields};
use hermes_starknet_chain_context::impls::error::HandleStarknetChainError;
use hermes_starknet_test_components::impls::bootstrap::bootstrap_chain::BootstrapStarknetDevnet;
use hermes_starknet_test_components::impls::bootstrap::deploy_contracts::{
    BuildChainAndDeployIbcContracts, DeployIbcContract,
};
use hermes_starknet_test_components::impls::bootstrap::start_chain::StartStarknetDevnet;
use hermes_starknet_test_components::impls::types::genesis_config::ProvideStarknetGenesisConfigType;
use hermes_starknet_test_components::impls::types::node_config::ProvideStarknetNodeConfigType;
use hermes_starknet_test_components::traits::IbcContractsDeployerComponent;
use hermes_starknet_test_components::types::genesis_config::StarknetGenesisConfig;
use hermes_starknet_test_components::types::node_config::StarknetNodeConfig;
use hermes_test_components::bootstrap::traits::chain::ChainBootstrapperComponent;
use hermes_test_components::chain_driver::traits::types::chain::{
    ChainTypeComponent, ProvideChainType,
};
use hermes_test_components::driver::traits::types::chain_driver::{
    ChainDriverTypeComponent, ProvideChainDriverType,
};
use starknet::accounts::{ExecutionEncoding, SingleOwnerAccount};
use starknet::core::types::contract::SierraClass;
use starknet::providers::jsonrpc::HttpTransport;
use starknet::providers::{JsonRpcClient, Provider};
use starknet::signers::{LocalWallet, SigningKey};
use tokio::process::Child;
use url::Url;

use crate::contexts::chain_driver::StarknetChainDriver;

#[cgp_context(StarknetBootstrapComponents)]
#[derive(HasField)]
pub struct StarknetBootstrap {
    pub runtime: HermesRuntime,
    pub chain_command_path: PathBuf,
    pub chain_store_dir: PathBuf,
    pub erc20_contract: SierraClass,
    pub ics20_contract: SierraClass,
    pub ibc_core_contract: SierraClass,
    pub comet_client_contract: SierraClass,
}

delegate_components! {
    StarknetBootstrapComponents {
        ErrorTypeProviderComponent: UseHermesError,
        ErrorRaiserComponent: UseDelegate<HandleStarknetChainError>,
        RuntimeTypeProviderComponent: WithType<HermesRuntime>,
        RuntimeGetterComponent: WithField<symbol!("runtime")>,
        [
            LoggerTypeProviderComponent,
            LoggerGetterComponent,
            GlobalLoggerGetterComponent,
        ]:
            UseHermesLogger,
        ChainNodeConfigTypeComponent:
            ProvideStarknetNodeConfigType,
        ChainGenesisConfigTypeComponent:
            ProvideStarknetGenesisConfigType,
        ChainBootstrapperComponent:
            BootstrapStarknetDevnet,
        ChainFullNodeStarterComponent:
            StartStarknetDevnet,
        IbcContractsDeployerComponent:
            DeployIbcContract,
        ChainDriverBuilderComponent:
            BuildChainAndDeployIbcContracts<BuildAndWaitChainDriver<BuildStarknetChainDriver>>,
    }
}

#[cgp_provider(ChainTypeComponent)]
impl ProvideChainType<StarknetBootstrap> for StarknetBootstrapComponents {
    type Chain = StarknetChain;
}

#[cgp_provider(ChainDriverTypeComponent)]
impl ProvideChainDriverType<StarknetBootstrap> for StarknetBootstrapComponents {
    type ChainDriver = StarknetChainDriver;
}

#[cgp_provider(ChainCommandPathGetterComponent)]
impl ChainCommandPathGetter<StarknetBootstrap> for StarknetBootstrapComponents {
    fn chain_command_path(bootstrap: &StarknetBootstrap) -> &PathBuf {
        &bootstrap.chain_command_path
    }
}

#[cgp_provider(ChainStoreDirGetterComponent)]
impl ChainStoreDirGetter<StarknetBootstrap> for StarknetBootstrapComponents {
    fn chain_store_dir(bootstrap: &StarknetBootstrap) -> &PathBuf {
        &bootstrap.chain_store_dir
    }
}

#[cgp_new_provider(ChainDriverBuilderComponent)]
impl ChainDriverBuilder<StarknetBootstrap> for BuildStarknetChainDriver {
    async fn build_chain_driver(
        bootstrap: &StarknetBootstrap,
        genesis_config: StarknetGenesisConfig,
        node_config: StarknetNodeConfig,
        wallets: BTreeMap<String, StarknetWallet>,
        chain_process: Child,
    ) -> Result<StarknetChainDriver, HermesError> {
        let runtime = bootstrap.runtime.clone();

        let chain_store_dir = bootstrap.chain_store_dir.clone();

        runtime.create_dir(&chain_store_dir.join("wallets")).await?;

        for (name, wallet) in wallets.iter() {
            let wallet_str = toml::to_string_pretty(wallet)?;
            let wallet_path = chain_store_dir.join(format!("wallets/{name}.toml"));

            runtime
                .write_string_to_file(&wallet_path, &wallet_str)
                .await?;
        }

        let relayer_wallet = wallets
            .get("relayer")
            .ok_or_else(|| StarknetBootstrap::raise_error("expect relayer wallet to be present"))?
            .clone();

        let user_wallet_a = wallets
            .get("user-a")
            .ok_or_else(|| StarknetBootstrap::raise_error("expect relayer wallet to be present"))?
            .clone();

        let user_wallet_b = wallets
            .get("user-b")
            .ok_or_else(|| StarknetBootstrap::raise_error("expect relayer wallet to be present"))?
            .clone();

        let json_rpc_url = Url::parse(&format!(
            "http://{}:{}/",
            node_config.rpc_addr, node_config.rpc_port
        ))?;

        let rpc_client = Arc::new(JsonRpcClient::new(HttpTransport::new(json_rpc_url)));

        // Wait for the chain to be ready.
        for _ in 0..10 {
            match rpc_client.block_number().await {
                Ok(_) => break,
                Err(_) => runtime.sleep(core::time::Duration::from_secs(1)).await,
            }
        }

        let chain_id = rpc_client.chain_id().await?;

        let account = SingleOwnerAccount::new(
            rpc_client.clone(),
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
                chain_id: chain_id.to_string().parse()?,
                rpc_client,
                account: Arc::new(account),
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
            chain_process: Some(chain_process),
            relayer_wallet,
            user_wallet_a,
            user_wallet_b,
        };

        Ok(chain_driver)
    }
}

check_components! {
    CanUseStarknetBootstrap for StarknetBootstrap {
        ChainFullNodeStarterComponent,
        ChainBootstrapperComponent,
        ChainDriverBuilderComponent,
        IbcContractsDeployerComponent,
    }
}
