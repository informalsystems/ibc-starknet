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
    type Transaction = StarknetTransaction;

    fn tx_size(tx: &StarknetTransaction) -> usize {
        tx.calls.len() // stub
    }
}
