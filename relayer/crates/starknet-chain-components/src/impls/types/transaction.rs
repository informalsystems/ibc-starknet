use cgp::core::Async;
use hermes_relayer_components::transaction::traits::types::transaction::ProvideTransactionType;
use starknet::accounts::Call;

pub struct ProvideCallTransaction;

impl<Chain: Async> ProvideTransactionType<Chain> for ProvideCallTransaction {
    type Transaction = Vec<Call>;

    fn tx_size(tx: &Vec<Call>) -> usize {
        tx.len() // stub
    }
}
