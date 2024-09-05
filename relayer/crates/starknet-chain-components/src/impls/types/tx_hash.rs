use cgp::core::Async;
use hermes_relayer_components::transaction::traits::types::tx_hash::ProvideTransactionHashType;
use starknet::core::types::Felt;

pub struct ProvideFeltTxHash;

impl<Chain: Async> ProvideTransactionHashType<Chain> for ProvideFeltTxHash {
    type TxHash = Felt;
}
