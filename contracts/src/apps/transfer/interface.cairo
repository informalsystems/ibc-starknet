use starknet::ContractAddress;
use starknet_ibc::apps::transfer::types::MsgTransfer;

#[starknet::interface]
pub trait ITransfer<TContractState> {
    fn send_transfer(self: @TContractState, msg: MsgTransfer,);
}

