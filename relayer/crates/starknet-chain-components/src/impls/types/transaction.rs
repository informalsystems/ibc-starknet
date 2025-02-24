use cgp::prelude::*;
use hermes_relayer_components::transaction::traits::types::transaction::{
    ProvideTransactionType, TransactionTypeComponent,
};
use starknet::accounts::Call;

pub struct ProvideCallTransaction;

#[cgp_provider(TransactionTypeComponent)]
impl<Chain: Async> ProvideTransactionType<Chain> for ProvideCallTransaction {
    type Transaction = Vec<Call>;

    fn tx_size(tx: &Vec<Call>) -> usize {
        tx.len() // stub
    }
}
