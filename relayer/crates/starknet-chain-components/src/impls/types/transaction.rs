use cgp::prelude::*;
use hermes_cosmos_chain_components::components::transaction::TransactionTypeComponent;
use hermes_relayer_components::transaction::traits::types::transaction::ProvideTransactionType;
use starknet::accounts::Call;

pub struct ProvideCallTransaction;

#[cgp_provider(TransactionTypeComponent)]
impl<Chain: Async> ProvideTransactionType<Chain> for ProvideCallTransaction {
    type Transaction = Vec<Call>;

    fn tx_size(tx: &Vec<Call>) -> usize {
        tx.len() // stub
    }
}
