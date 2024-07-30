use hermes_relayer_components::transaction::traits::types::tx_hash::HasTransactionHashType;
use starknet::accounts::{Account, Call};
use starknet::core::types::Felt;

use crate::traits::account::{CanRaiseAccountErrors, HasStarknetAccount};
use crate::traits::contract::invoke::ContractInvoker;
use crate::traits::provider::HasStarknetProvider;
use crate::traits::types::address::HasAddressType;
use crate::traits::types::blob::HasBlobType;
use crate::traits::types::method::HasMethodSelectorType;

pub struct InvokeStarknetContract;

impl<Chain> ContractInvoker<Chain> for InvokeStarknetContract
where
    Chain: HasAddressType<Address = Felt>
        + HasMethodSelectorType<MethodSelector = Felt>
        + HasBlobType<Blob = Vec<Felt>>
        + HasTransactionHashType<TxHash = Felt>
        + HasStarknetProvider
        + HasStarknetAccount
        + CanRaiseAccountErrors,
{
    async fn invoke_contract(
        chain: &Chain,
        contract_address: &Felt,
        entry_point_selector: &Felt,
        calldata: &Vec<Felt>,
    ) -> Result<Felt, Chain::Error> {
        let account = chain.account();

        let call = Call {
            to: *contract_address,
            selector: *entry_point_selector,
            calldata: calldata.clone(),
        };

        let execution = account.execute_v3(vec![call]);

        let tx_hash = execution
            .send()
            .await
            .map_err(Chain::raise_error)?
            .transaction_hash;

        Ok(tx_hash)
    }
}
