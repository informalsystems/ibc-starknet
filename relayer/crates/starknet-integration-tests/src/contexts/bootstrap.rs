use std::collections::BTreeMap;
use std::path::PathBuf;
use std::sync::Arc;

use cgp::core::component::UseDelegate;
use cgp::core::error::{ErrorRaiserComponent, ErrorTypeProviderComponent};
use cgp::core::field::WithField;
use cgp::core::types::WithType;
use cgp::prelude::*;
use futures::lock::Mutex;
use hermes_cosmos_chain_components::types::key_types::secp256k1::Secp256k1KeyPair;
use hermes_cosmos_test_components::bootstrap::traits::chain::build_chain_driver::{
    ChainDriverBuilder, ChainDriverBuilderComponent,
};
use hermes_cosmos_test_components::bootstrap::traits::chain::start_chain::{
    CanStartChainFullNode, ChainFullNodeStarterComponent,
};
use hermes_cosmos_test_components::bootstrap::traits::fields::chain_command_path::{
    ChainCommandPathGetter, ChainCommandPathGetterComponent,
};
use hermes_cosmos_test_components::bootstrap::traits::fields::chain_store_dir::{
    ChainStoreDirGetter, ChainStoreDirGetterComponent,
};
use hermes_cosmos_test_components::bootstrap::traits::types::chain_node_config::ChainNodeConfigTypeComponent;
use hermes_cosmos_test_components::bootstrap::traits::types::genesis_config::ChainGenesisConfigTypeComponent;
use hermes_error::impls::ProvideHermesError;
use hermes_error::types::HermesError;
use hermes_runtime::types::runtime::HermesRuntime;
use hermes_runtime_components::traits::runtime::{
    HasRuntime, RuntimeGetterComponent, RuntimeTypeProviderComponent,
};
use hermes_starknet_chain_components::types::wallet::StarknetWallet;
use hermes_starknet_chain_context::contexts::chain::{StarknetChain, StarknetChainFields};
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
use hermes_test_components::chain_driver::traits::types::chain::{
    ChainTypeComponent, ProvideChainType,
};
use hermes_test_components::driver::traits::types::chain_driver::{
    ChainDriverTypeComponent, ProvideChainDriverType,
};
use starknet::accounts::{ExecutionEncoding, SingleOwnerAccount};
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
}

delegate_components! {
    StarknetBootstrapComponents {
        ErrorTypeProviderComponent: ProvideHermesError,
        ErrorRaiserComponent: UseDelegate<HandleStarknetChainError>,
        RuntimeTypeProviderComponent: WithType<HermesRuntime>,
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

#[cgp_provider(ChainDriverBuilderComponent)]
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

        let json_rpc_url = Url::parse(&format!(
            "http://{}:{}/",
            node_config.rpc_addr, node_config.rpc_port
        ))?;

        let rpc_client = Arc::new(JsonRpcClient::new(HttpTransport::new(json_rpc_url)));

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
                ibc_client_contract_address: None,
                ibc_core_contract_address: None,
                event_encoding: Default::default(),
                proof_signer,
                poll_interval: core::time::Duration::from_millis(200),
                nonce_mutex: Arc::new(Mutex::new(())),
            }),
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
