use core::marker::PhantomData;
use std::sync::OnceLock;

use cgp::prelude::*;
use hermes_chain_type_components::traits::types::address::HasAddressType;

use crate::traits::queries::address::{ContractAddressQuerier, ContractAddressQuerierComponent};

pub struct GetContractAddressFromField;

#[derive(Debug)]
pub struct ContractAddressNotFound;

#[cgp_provider(ContractAddressQuerierComponent)]
impl<Chain, Tag> ContractAddressQuerier<Chain, Tag> for GetContractAddressFromField
where
    Chain: HasAddressType<Address: Clone>
        + CanRaiseAsyncError<ContractAddressNotFound>
        + HasField<Tag, Value = OnceLock<Chain::Address>>,
    Tag: Async,
{
    async fn query_contract_address(
        chain: &Chain,
        tag: PhantomData<Tag>,
    ) -> Result<Chain::Address, Chain::Error> {
        chain
            .get_field(tag)
            .get()
            .cloned()
            .ok_or_else(|| Chain::raise_error(ContractAddressNotFound))
    }
}
