use cgp::prelude::*;
use hermes_cosmos_chain_components::components::transaction::TxSubmitterComponent;
use hermes_relayer_components::transaction::traits::submit_tx::TxSubmitter;
use hermes_relayer_components::transaction::traits::types::transaction::HasTransactionType;
use hermes_relayer_components::transaction::traits::types::tx_hash::HasTransactionHashType;
use starknet::accounts::{Account, Call};
use starknet::core::types::Felt;

use crate::traits::account::{CanRaiseAccountErrors, HasStarknetAccount};
use crate::traits::provider::HasStarknetProvider;

pub struct SubmitCallTransaction;

#[cgp_provider(TxSubmitterComponent)]
impl<Chain> TxSubmitter<Chain> for SubmitCallTransaction
where
    Chain: HasTransactionType<Transaction = Vec<Call>>
        + HasTransactionHashType<TxHash = Felt>
        + HasStarknetProvider
        + HasStarknetAccount
        + CanRaiseAccountErrors,
{
    async fn submit_tx(chain: &Chain, messages: &Vec<Call>) -> Result<Felt, Chain::Error> {
        let account = chain.account();

        let execution = account.execute_v3(messages.clone());

        let tx_hash = execution
            .send()
            .await
            .map_err(Chain::raise_error)?
            .transaction_hash;

        Ok(tx_hash)
    }
}
