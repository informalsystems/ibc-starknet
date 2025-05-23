use hermes_core::test_components::chain_driver::traits::HasChainType;
use hermes_prelude::*;

#[cgp_component {
    provider: IbcContractsDeployer,
}]
#[async_trait]
pub trait CanDeployIbcContracts: HasChainType + HasAsyncErrorType {
    async fn deploy_ibc_contracts(&self, chain: &Self::Chain) -> Result<(), Self::Error>;
}
