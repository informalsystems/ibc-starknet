use cgp::prelude::*;
use hermes_test_components::chain_driver::traits::types::chain::HasChainType;

#[cgp_component {
    provider: IbcContractsDeployer,
}]
#[async_trait]
pub trait CanDeployIbcContracts: HasChainType + HasAsyncErrorType {
    async fn deploy_ibc_contracts(&self, chain: &Self::Chain) -> Result<(), Self::Error>;
}
