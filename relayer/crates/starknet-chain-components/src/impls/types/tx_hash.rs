use cgp::prelude::*;
use hermes_cosmos_chain_components::components::transaction::TransactionHashTypeComponent;
use hermes_relayer_components::transaction::traits::types::tx_hash::ProvideTransactionHashType;
use starknet::core::types::Felt;

pub struct ProvideFeltTxHash;

#[cgp_provider(TransactionHashTypeComponent)]
impl<Chain: Async> ProvideTransactionHashType<Chain> for ProvideFeltTxHash {
    type TxHash = Felt;
}
