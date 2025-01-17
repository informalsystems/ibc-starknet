use core::marker::PhantomData;

use cgp::prelude::*;
use hermes_chain_type_components::traits::types::address::HasAddressType;

use crate::traits::queries::address::ContractAddressQuerier;

pub struct GetContractAddressFromField;

#[derive(Debug)]
pub struct ContractAddressNotFound;

impl<Chain, Tag> ContractAddressQuerier<Chain, Tag> for GetContractAddressFromField
where
    Chain: HasAddressType<Address: Clone>
        + CanRaiseAsyncError<ContractAddressNotFound>
        + HasField<Tag, Value = Option<Chain::Address>>,
    Tag: Async,
{
    async fn query_contract_address(
        chain: &Chain,
        tag: PhantomData<Tag>,
    ) -> Result<Chain::Address, Chain::Error> {
        chain
            .get_field(tag)
            .clone()
            .ok_or_else(|| Chain::raise_error(ContractAddressNotFound))
    }
}
