use core::marker::PhantomData;

use cgp::prelude::*;
use hermes_chain_type_components::traits::HasAddressType;

#[cgp_component {
  name: ContractAddressQuerierComponent,
  provider: ContractAddressQuerier,
  context: Chain,
}]
#[async_trait]
pub trait CanQueryContractAddress<Tag: Async>: HasAddressType + HasAsyncErrorType {
    async fn query_contract_address(
        &self,
        tag: PhantomData<Tag>,
    ) -> Result<Self::Address, Self::Error>;
}
