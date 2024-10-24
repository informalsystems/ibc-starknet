use core::marker::PhantomData;

use cgp::prelude::*;
use hermes_chain_type_components::traits::types::address::HasAddressType;

#[derive_component(ContractAddressQuerierComponent, ContractAddressQuerier<Chain>)]
#[async_trait]
pub trait CanQueryContractAddress<Tag: Async>: HasAddressType + HasErrorType {
    async fn query_contract_address(
        &self,
        tag: PhantomData<Tag>,
    ) -> Result<Self::Address, Self::Error>;
}
