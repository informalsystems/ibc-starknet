use starknet::ContractAddress;
use starknet_ibc::apps::transfer::types::{MsgTransfer, Packet, Memo};
use starknet_ibc::core::types::{PortId, ChannelId};

#[starknet::interface]
pub trait ISendTransfer<TContractState> {
    fn send_validate(self: @TContractState, msg: MsgTransfer);
    fn send_execute(ref self: TContractState, msg: MsgTransfer);
}

#[starknet::interface]
pub trait IRecvPacket<TContractState> {
    fn recv_validate(self: @TContractState, packet: Packet);
    fn recv_execute(ref self: TContractState, packet: Packet);
}

#[starknet::interface]
pub trait ITransferrable<TContractState> {
    fn can_send(self: @TContractState);
    fn can_receive(self: @TContractState);
}

