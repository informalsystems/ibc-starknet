use std::collections::BTreeMap;
use std::path::PathBuf;
use std::sync::Arc;

use cgp::core::component::UseDelegate;
use cgp::core::error::{ErrorRaiserComponent, ErrorTypeComponent};
use cgp::core::field::WithField;
use cgp::core::types::WithType;
use cgp::prelude::*;
use hermes_cosmos_test_components::bootstrap::components::cosmos_sdk::{
    ChainGenesisConfigTypeComponent, ChainNodeConfigTypeComponent,
};
use hermes_cosmos_test_components::bootstrap::traits::chain::build_chain_driver::ChainDriverBuilder;
use hermes_cosmos_test_components::bootstrap::traits::chain::start_chain::{
    CanStartChainFullNode, ChainFullNodeStarterComponent,
};
use hermes_cosmos_test_components::bootstrap::traits::fields::chain_command_path::ChainCommandPathGetter;
use hermes_cosmos_test_components::bootstrap::traits::fields::chain_store_dir::ChainStoreDirGetter;
use hermes_error::impls::ProvideHermesError;
use hermes_error::types::HermesError;
use hermes_runtime::types::runtime::HermesRuntime;
use hermes_runtime_components::traits::runtime::{
    HasRuntime, RuntimeGetterComponent, RuntimeTypeComponent,
};
use hermes_starknet_chain_components::types::wallet::StarknetWallet;
use hermes_starknet_chain_context::contexts::chain::StarknetChain;
use hermes_starknet_chain_context::impls::error::HandleStarknetChainError;
use hermes_starknet_test_components::impls::bootstrap::bootstrap_chain::BootstrapStarknetDevnet;
use hermes_starknet_test_components::impls::bootstrap::start_chain::StartStarknetDevnet;
use hermes_starknet_test_components::impls::types::genesis_config::ProvideStarknetGenesisConfigType;
use hermes_starknet_test_components::impls::types::node_config::ProvideStarknetNodeConfigType;
use hermes_starknet_test_components::types::genesis_config::StarknetGenesisConfig;
use hermes_starknet_test_components::types::node_config::StarknetNodeConfig;
use hermes_test_components::bootstrap::traits::chain::{
    CanBootstrapChain, ChainBootstrapperComponent,
};
use hermes_test_components::chain_driver::traits::types::chain::ProvideChainType;
use hermes_test_components::driver::traits::types::chain_driver::ProvideChainDriverType;
use starknet::accounts::{ExecutionEncoding, SingleOwnerAccount};
use starknet::providers::jsonrpc::HttpTransport;
use starknet::providers::{JsonRpcClient, Provider};
use starknet::signers::{LocalWallet, SigningKey};
use tokio::process::Child;
use url::Url;

use crate::contexts::chain_driver::StarknetChainDriver;

#[derive(HasField)]
pub struct StarknetBootstrap {
    pub runtime: HermesRuntime,
    pub chain_command_path: PathBuf,
    pub chain_store_dir: PathBuf,
}

pub struct StarknetBootstrapComponents;

impl HasComponents for StarknetBootstrap {
    type Components = StarknetBootstrapComponents;
}

delegate_components! {
    StarknetBootstrapComponents {
        ErrorTypeComponent: ProvideHermesError,
        ErrorRaiserComponent: UseDelegate<HandleStarknetChainError>,
        RuntimeTypeComponent: WithType<HermesRuntime>,
        RuntimeGetterComponent: WithField<symbol!("runtime")>,
        ChainNodeConfigTypeComponent:
            ProvideStarknetNodeConfigType,
        ChainGenesisConfigTypeComponent:
            ProvideStarknetGenesisConfigType,
        ChainBootstrapperComponent:
            BootstrapStarknetDevnet,
        ChainFullNodeStarterComponent:
            StartStarknetDevnet,
    }
}

impl ProvideChainType<StarknetBootstrap> for StarknetBootstrapComponents {
    type Chain = StarknetChain;
}

impl ProvideChainDriverType<StarknetBootstrap> for StarknetBootstrapComponents {
    type ChainDriver = StarknetChainDriver;
}

impl ChainCommandPathGetter<StarknetBootstrap> for StarknetBootstrapComponents {
    fn chain_command_path(bootstrap: &StarknetBootstrap) -> &PathBuf {
        &bootstrap.chain_command_path
    }
}

impl ChainStoreDirGetter<StarknetBootstrap> for StarknetBootstrapComponents {
    fn chain_store_dir(bootstrap: &StarknetBootstrap) -> &PathBuf {
        &bootstrap.chain_store_dir
    }
}

impl ChainDriverBuilder<StarknetBootstrap> for StarknetBootstrapComponents {
    async fn build_chain_driver(
        bootstrap: &StarknetBootstrap,
        genesis_config: StarknetGenesisConfig,
        node_config: StarknetNodeConfig,
        wallets: BTreeMap<String, StarknetWallet>,
        chain_process: Child,
    ) -> Result<StarknetChainDriver, HermesError> {
        let runtime = bootstrap.runtime.clone();

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

        let json_rpc_url = Url::parse(&format!("http://localhost:{}/", node_config.rpc_port))?;

        let rpc_client = Arc::new(JsonRpcClient::new(HttpTransport::new(json_rpc_url)));

        let chain_id = rpc_client.chain_id().await?;

        let account = SingleOwnerAccount::new(
            rpc_client.clone(),
            LocalWallet::from_signing_key(SigningKey::from_secret_scalar(
                relayer_wallet.signing_key,
            )),
            relayer_wallet.account_address,
            chain_id,
            ExecutionEncoding::New,
        );

        let chain = StarknetChain {
            runtime: runtime.clone(),
            chain_id: chain_id.to_string().parse()?,
            rpc_client,
            account,
            ibc_client_contract_address: None,
            ibc_core_contract_address: None,
            event_encoding: Default::default(),
        };

        let chain_driver = StarknetChainDriver {
            runtime,
            chain,
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

pub trait CanUseStarknetBootstrap:
    HasRuntime<Runtime = HermesRuntime> + CanStartChainFullNode + CanBootstrapChain
{
}

impl CanUseStarknetBootstrap for StarknetBootstrap {}
