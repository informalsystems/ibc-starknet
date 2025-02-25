use cgp::prelude::*;
use hermes_relayer_components::transaction::traits::types::transaction::{
    ProvideTransactionType, TransactionTypeComponent,
};

use crate::traits::account::HasStarknetAccountType;
use crate::types::transaction::StarknetTransaction;

pub struct UseStarknetTransaction;

#[cgp_provider(TransactionTypeComponent)]
impl<Chain: Async> ProvideTransactionType<Chain> for UseStarknetTransaction
where
    Chain: HasStarknetAccountType,
{
    type Transaction = StarknetTransaction<Chain::Account>;

    fn tx_size(tx: &StarknetTransaction<Chain::Account>) -> usize {
        tx.calls.len() // stub
    }
}
