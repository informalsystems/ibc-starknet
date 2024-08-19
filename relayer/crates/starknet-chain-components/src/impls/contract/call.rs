use cgp_core::error::CanRaiseError;
use hermes_test_components::chain::traits::types::address::HasAddressType;
use starknet::core::types::{BlockId, BlockTag, Felt, FunctionCall};
use starknet::providers::{Provider, ProviderError};

use crate::traits::contract::call::ContractCaller;
use crate::traits::provider::HasStarknetProvider;
use crate::traits::types::blob::HasBlobType;
use crate::traits::types::method::HasSelectorType;

pub struct CallStarknetContract;

impl<Chain> ContractCaller<Chain> for CallStarknetContract
where
    Chain: HasAddressType<Address = Felt>
        + HasSelectorType<Selector = Felt>
        + HasBlobType<Blob = Vec<Felt>>
        + HasStarknetProvider
        + CanRaiseError<ProviderError>,
{
    async fn call_contract(
        chain: &Chain,
        contract_address: &Felt,
        entry_point_selector: &Felt,
        calldata: &Vec<Felt>,
    ) -> Result<Vec<Felt>, Chain::Error> {
        let block_id = BlockId::Tag(BlockTag::Pending);

        let res = chain
            .provider()
            .call(
                FunctionCall {
                    contract_address: *contract_address,
                    entry_point_selector: *entry_point_selector,
                    calldata: calldata.clone(),
                },
                block_id,
            )
            .await
            .map_err(Chain::raise_error)?;

        Ok(res)
    }
}
