use std::collections::BTreeMap;

use cgp::extra::runtime::HasRuntimeType;
use cgp::prelude::*;
use hermes_cosmos_test_components::bootstrap::traits::chain::build_chain_driver::{
    ChainDriverBuilder, ChainDriverBuilderComponent,
};
use hermes_cosmos_test_components::bootstrap::traits::types::chain_node_config::HasChainNodeConfigType;
use hermes_cosmos_test_components::bootstrap::traits::types::genesis_config::HasChainGenesisConfigType;
use hermes_runtime_components::traits::os::child_process::{ChildProcessOf, HasChildProcessType};
use hermes_test_components::chain::traits::types::wallet::HasWalletType;
use hermes_test_components::chain_driver::traits::types::chain::{HasChain, HasChainType};
use hermes_test_components::driver::traits::types::chain_driver::HasChainDriverType;

use crate::traits::{CanDeployIbcContracts, IbcContractsDeployer, IbcContractsDeployerComponent};

pub trait HasIbcContracts {}

#[cgp_new_provider(IbcContractsDeployerComponent)]
impl<Bootstrap, Chain> IbcContractsDeployer<Bootstrap> for DeployIbcContract
where
    Bootstrap: HasChainType<Chain = Chain> + CanRaiseAsyncError<Chain::Error>,
    Chain: HasAsyncErrorType,
{
    async fn deploy_ibc_contracts(
        bootstrap: &Bootstrap,
        chain: &Chain,
    ) -> Result<(), Bootstrap::Error> {
        Ok(())
    }
}

#[cgp_new_provider(ChainDriverBuilderComponent)]
impl<Bootstrap, InBuilder, Chain> ChainDriverBuilder<Bootstrap>
    for BuildChainAndDeployIbcContracts<InBuilder>
where
    Bootstrap: HasRuntimeType<Runtime: HasChildProcessType>
        + HasChainDriverType<Chain = Chain>
        + HasChainGenesisConfigType
        + HasChainNodeConfigType
        + CanDeployIbcContracts
        + HasAsyncErrorType,
    Bootstrap::ChainDriver: HasChain<Chain = Chain>,
    Chain: HasWalletType,
    InBuilder: ChainDriverBuilder<Bootstrap>,
{
    async fn build_chain_driver(
        bootstrap: &Bootstrap,
        genesis_config: Bootstrap::ChainGenesisConfig,
        chain_node_config: Bootstrap::ChainNodeConfig,
        wallets: BTreeMap<String, Chain::Wallet>,
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

        let chain = chain_driver.chain();

        bootstrap.deploy_ibc_contracts(chain).await?;

        Ok(chain_driver)
    }
}
