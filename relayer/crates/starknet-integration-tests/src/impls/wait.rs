use core::marker::PhantomData;
use core::time::Duration;
use std::collections::BTreeMap;

use cgp::prelude::HasErrorType;
use hermes_cosmos_test_components::bootstrap::traits::chain::build_chain_driver::ChainDriverBuilder;
use hermes_cosmos_test_components::bootstrap::traits::types::chain_node_config::HasChainNodeConfigType;
use hermes_cosmos_test_components::bootstrap::traits::types::genesis_config::HasChainGenesisConfigType;
use hermes_runtime_components::traits::os::child_process::{ChildProcessOf, HasChildProcessType};
use hermes_runtime_components::traits::runtime::HasRuntime;
use hermes_runtime_components::traits::sleep::CanSleep;
use hermes_test_components::chain::traits::types::wallet::{HasWalletType, Wallet};
use hermes_test_components::chain_driver::traits::types::chain::HasChainType;
use hermes_test_components::driver::traits::types::chain_driver::HasChainDriverType;

pub struct BuildChainDriverAndPause<InBuilder>(pub PhantomData<InBuilder>);

impl<Bootstrap, InBuilder> ChainDriverBuilder<Bootstrap> for BuildChainDriverAndPause<InBuilder>
where
    Bootstrap: HasRuntime<Runtime: HasChildProcessType + CanSleep>
        + HasChainType<Chain: HasWalletType>
        + HasChainDriverType
        + HasChainGenesisConfigType
        + HasChainNodeConfigType
        + HasErrorType,
    InBuilder: ChainDriverBuilder<Bootstrap>,
{
    async fn build_chain_driver(
        bootstrap: &Bootstrap,
        genesis_config: Bootstrap::ChainGenesisConfig,
        chain_node_config: Bootstrap::ChainNodeConfig,
        wallets: BTreeMap<String, Wallet<Bootstrap::Chain>>,
        chain_process: ChildProcessOf<Bootstrap::Runtime>,
    ) -> Result<Bootstrap::ChainDriver, Bootstrap::Error> {
        let chain_driver = InBuilder::build_chain_driver(
            bootstrap,
            genesis_config,
            chain_node_config,
            wallets,
            chain_process,
        )
        .await?;

        bootstrap.runtime().sleep(Duration::from_secs(5)).await;

        Ok(chain_driver)
    }
}
