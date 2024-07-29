use cgp_core::error::CanRaiseError;
use starknet::core::types::{BlockId, BlockTag, Felt, FunctionCall};
use starknet::providers::{Provider, ProviderError};

use crate::traits::client::HasJsonRpcClient;
use crate::traits::contract::call::ContractCaller;
use crate::traits::types::address::HasAddressType;
use crate::traits::types::blob::HasBlobType;
use crate::traits::types::method::HasMethodSelectorType;

pub struct CallStarknetContract;

impl<Chain> ContractCaller<Chain> for CallStarknetContract
where
    Chain: HasAddressType<Address = Felt>
        + HasMethodSelectorType<MethodSelector = Felt>
        + HasBlobType<Blob = Vec<Felt>>
        + HasJsonRpcClient
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
            .json_rpc_client()
            .call(
                FunctionCall {
                    contract_address: contract_address.clone(),
                    entry_point_selector: entry_point_selector.clone(),
                    calldata: calldata.clone(),
                },
                block_id,
            )
            .await
            .map_err(Chain::raise_error)?;

        Ok(res)
    }
}