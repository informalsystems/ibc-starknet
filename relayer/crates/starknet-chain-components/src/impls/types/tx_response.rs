use cgp_core::Async;
use hermes_relayer_components::transaction::traits::types::tx_response::ProvideTxResponseType;
use starknet::core::types::TransactionReceiptWithBlockInfo;

pub struct ProvideTransactionReceipt;

impl<Chain: Async> ProvideTxResponseType<Chain> for ProvideTransactionReceipt {
    type TxResponse = TransactionReceiptWithBlockInfo;
}
