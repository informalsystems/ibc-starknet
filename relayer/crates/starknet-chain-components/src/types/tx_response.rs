use starknet::core::types::{
    ExecutionResources, ExecutionResult, RevertedInvocation, TransactionReceipt,
    TransactionReceiptWithBlockInfo, TransactionTrace,
};

#[derive(Debug)]
pub struct TxResponse {
    pub receipt: TransactionReceiptWithBlockInfo,
    pub trace: TransactionTrace,
}

impl TxResponse {
    pub fn is_reverted(&self) -> Option<RevertedInvocation> {
        match &self.receipt.receipt {
            TransactionReceipt::Invoke(receipt) => {
                if let ExecutionResult::Reverted { ref reason } = receipt.execution_result {
                    Some(RevertedInvocation {
                        revert_reason: reason.clone(),
                    })
                } else {
                    None
                }
            }
            TransactionReceipt::L1Handler(receipt) => {
                if let ExecutionResult::Reverted { ref reason } = receipt.execution_result {
                    Some(RevertedInvocation {
                        revert_reason: reason.clone(),
                    })
                } else {
                    None
                }
            }
            TransactionReceipt::Declare(receipt) => {
                if let ExecutionResult::Reverted { ref reason } = receipt.execution_result {
                    Some(RevertedInvocation {
                        revert_reason: reason.clone(),
                    })
                } else {
                    None
                }
            }
            TransactionReceipt::Deploy(receipt) => {
                if let ExecutionResult::Reverted { ref reason } = receipt.execution_result {
                    Some(RevertedInvocation {
                        revert_reason: reason.clone(),
                    })
                } else {
                    None
                }
            }
            TransactionReceipt::DeployAccount(receipt) => {
                if let ExecutionResult::Reverted { ref reason } = receipt.execution_result {
                    Some(RevertedInvocation {
                        revert_reason: reason.clone(),
                    })
                } else {
                    None
                }
            }
        }
    }

    pub fn execution_resources(&self) -> &ExecutionResources {
        match &self.receipt.receipt {
            TransactionReceipt::Invoke(receipt) => &receipt.execution_resources,
            TransactionReceipt::L1Handler(receipt) => &receipt.execution_resources,
            TransactionReceipt::Declare(receipt) => &receipt.execution_resources,
            TransactionReceipt::Deploy(receipt) => &receipt.execution_resources,
            TransactionReceipt::DeployAccount(receipt) => &receipt.execution_resources,
        }
    }
}
