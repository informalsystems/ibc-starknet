use starknet::core::types::{TransactionReceiptWithBlockInfo, TransactionTrace};

#[derive(Debug)]
pub struct TxResponse {
    pub receipt: TransactionReceiptWithBlockInfo,
    pub trace: TransactionTrace,
}
