use cgp::prelude::*;
use hermes_relayer_components::transaction::traits::submit_tx::{
    TxSubmitter, TxSubmitterComponent,
};
use hermes_relayer_components::transaction::traits::types::transaction::HasTransactionType;
use hermes_relayer_components::transaction::traits::types::tx_hash::HasTransactionHashType;
use starknet::accounts::Account;
use starknet::core::types::Felt;

use crate::traits::account::{CanRaiseAccountErrors, HasStarknetAccount};
use crate::traits::provider::HasStarknetProvider;
use crate::types::transaction::StarknetTransaction;

#[cgp_new_provider(TxSubmitterComponent)]
impl<Chain> TxSubmitter<Chain> for SubmitCallTransaction
where
    Chain: HasTransactionType<Transaction = StarknetTransaction>
        + HasTransactionHashType<TxHash = Felt>
        + HasStarknetProvider
        + HasStarknetAccount
        + CanRaiseAccountErrors,
{
    async fn submit_tx(
        chain: &Chain,
        transcation: &StarknetTransaction,
    ) -> Result<Felt, Chain::Error> {
        let account = chain.account();

        let execution = account.execute_v3(transcation.calls.clone());

        let tx_hash = execution
            .send()
            .await
            .map_err(Chain::raise_error)?
            .transaction_hash;

        Ok(tx_hash)
    }
}
