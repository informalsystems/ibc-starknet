use cgp::prelude::*;
use hermes_relayer_components::transaction::traits::submit_tx::{
    TxSubmitter, TxSubmitterComponent,
};
use hermes_relayer_components::transaction::traits::types::transaction::HasTransactionType;
use hermes_relayer_components::transaction::traits::types::tx_hash::HasTransactionHashType;
use starknet::accounts::Account;
use starknet::core::types::Felt;

use crate::traits::account::{CanRaiseAccountErrors, HasStarknetAccountType};
use crate::traits::provider::HasStarknetProvider;
use crate::types::transaction::StarknetTransaction;

pub struct SubmitCallTransaction;

#[cgp_provider(TxSubmitterComponent)]
impl<Chain> TxSubmitter<Chain> for SubmitCallTransaction
where
    Chain: HasTransactionType<Transaction = StarknetTransaction<Chain::Account>>
        + HasTransactionHashType<TxHash = Felt>
        + HasStarknetProvider
        + HasStarknetAccountType
        + CanRaiseAccountErrors,
{
    async fn submit_tx(
        chain: &Chain,
        transcation: &StarknetTransaction<Chain::Account>,
    ) -> Result<Felt, Chain::Error> {
        let account = &transcation.account;

        let execution = account.execute_v3(transcation.calls.clone());

        let tx_hash = execution
            .send()
            .await
            .map_err(Chain::raise_error)?
            .transaction_hash;

        Ok(tx_hash)
    }
}
