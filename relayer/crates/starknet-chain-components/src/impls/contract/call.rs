use cgp::prelude::*;
use hermes_chain_components::traits::HasHeightType;
use hermes_chain_type_components::traits::HasAddressType;
use starknet::core::types::{BlockId, BlockTag, Felt, FunctionCall};
use starknet::providers::{Provider, ProviderError};

use crate::impls::types::address::StarknetAddress;
use crate::traits::client::HasStarknetClient;
use crate::traits::contract::call::{ContractCaller, ContractCallerComponent};
use crate::traits::types::blob::HasBlobType;
use crate::traits::types::method::HasSelectorType;

pub struct CallStarknetContract;

#[cgp_provider(ContractCallerComponent)]
impl<Chain> ContractCaller<Chain> for CallStarknetContract
where
    Chain: HasAddressType<Address = StarknetAddress>
        + HasSelectorType<Selector = Felt>
        + HasBlobType<Blob = Vec<Felt>>
        + HasStarknetClient<Client: Provider>
        + HasHeightType<Height = u64>
        + CanRaiseAsyncError<ProviderError>,
{
    async fn call_contract(
        chain: &Chain,
        contract_address: &StarknetAddress,
        entry_point_selector: &Felt,
        calldata: &Vec<Felt>,
        height: Option<&u64>,
    ) -> Result<Vec<Felt>, Chain::Error> {
        let block_id = match height {
            Some(height) => BlockId::Number(*height),
            None => BlockId::Tag(BlockTag::Latest),
        };

        let res = Provider::call(
            chain.provider(),
            FunctionCall {
                contract_address: **contract_address,
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
