use starknet::core::types::{Felt, U256};


pub enum Erc20Events {
    Transfer(TransferEvent),
    Approval(ApprovalEvent),
}

pub struct TransferEvent {
    pub from: Felt,
    pub to: Felt,
    pub value: U256,
}

pub struct ApprovalEvent {
    pub owner: Felt,
    pub spender: Felt,
    pub value: U256,
}