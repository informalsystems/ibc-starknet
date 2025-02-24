use cgp::prelude::*;
use hermes_relayer_components::transaction::traits::types::tx_hash::{
    ProvideTransactionHashType, TransactionHashTypeComponent,
};
use starknet::core::types::Felt;

pub struct ProvideFeltTxHash;

#[cgp_provider(TransactionHashTypeComponent)]
impl<Chain: Async> ProvideTransactionHashType<Chain> for ProvideFeltTxHash {
    type TxHash = Felt;
}
