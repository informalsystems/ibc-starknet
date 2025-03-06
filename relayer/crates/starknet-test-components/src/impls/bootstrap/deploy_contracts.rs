use std::collections::BTreeMap;

use cgp::extra::runtime::HasRuntimeType;
use cgp::prelude::*;
use hermes_cosmos_test_components::bootstrap::traits::chain::build_chain_driver::{
    ChainDriverBuilder, ChainDriverBuilderComponent,
};
use hermes_cosmos_test_components::bootstrap::traits::types::chain_node_config::HasChainNodeConfigType;
use hermes_cosmos_test_components::bootstrap::traits::types::genesis_config::HasChainGenesisConfigType;
use hermes_runtime_components::traits::os::child_process::{ChildProcessOf, HasChildProcessType};
use hermes_starknet_chain_components::traits::contract::declare::CanDeclareContract;
use hermes_starknet_chain_components::traits::contract::deploy::CanDeployContract;
use hermes_starknet_chain_components::traits::types::blob::HasBlobType;
use hermes_starknet_chain_components::traits::types::contract_class::{
    ContractClassOf, HasContractClassType,
};
use hermes_test_components::chain::traits::types::wallet::HasWalletType;
use hermes_test_components::chain_driver::traits::types::chain::{HasChain, HasChainType};
use hermes_test_components::driver::traits::types::chain_driver::HasChainDriverType;
use starknet::core::types::Felt;

use crate::traits::{CanDeployIbcContracts, IbcContractsDeployer, IbcContractsDeployerComponent};

#[cgp_auto_getter]
pub trait HasIbcContracts: HasChainType<Chain: HasContractClassType> {
    fn erc20_contract(&self) -> &ContractClassOf<Self::Chain>;

    fn ics20_contract(&self) -> &ContractClassOf<Self::Chain>;

    fn ibc_core_contract(&self) -> &ContractClassOf<Self::Chain>;
}

#[cgp_new_provider(IbcContractsDeployerComponent)]
impl<Bootstrap, Chain> IbcContractsDeployer<Bootstrap> for DeployIbcContract
where
    Bootstrap: HasChainType<Chain = Chain> + HasIbcContracts + CanRaiseAsyncError<Chain::Error>,
    Chain:
        CanDeployContract + CanDeclareContract + HasBlobType<Blob = Vec<Felt>> + HasAsyncErrorType,
{
    async fn deploy_ibc_contracts(
        bootstrap: &Bootstrap,
        chain: &Chain,
    ) -> Result<(), Bootstrap::Error> {
        let erc20_class_hash = chain
            .declare_contract(bootstrap.erc20_contract())
            .await
            .map_err(Bootstrap::raise_error)?;

        let ics20_class_hash = chain
            .declare_contract(bootstrap.ics20_contract())
            .await
            .map_err(Bootstrap::raise_error)?;

        let ibc_core_class_hash = chain
            .declare_contract(bootstrap.ibc_core_contract())
            .await
            .map_err(Bootstrap::raise_error)?;

        let ibc_core_address = chain
            .deploy_contract(&ibc_core_class_hash, false, &Vec::new())
            .await
            .map_err(Bootstrap::raise_error)?;

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

        bootstrap.deploy_ibc_contracts(&chain).await?;

        Ok(chain_driver)
    }
}
