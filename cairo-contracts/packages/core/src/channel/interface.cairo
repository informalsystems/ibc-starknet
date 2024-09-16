use starknet_ibc_core::channel::{Packet, MsgRecvPacket, Acknowledgement};

#[starknet::interface]
pub trait IChannelHandler<TContractState> {
    fn recv_packet(ref self: TContractState, msg: MsgRecvPacket);
}

#[starknet::interface]
pub trait IAppCallback<TContractState> {
    fn on_recv_packet(ref self: TContractState, packet: Packet) -> Acknowledgement;
    fn on_acknowledgement_packet(ref self: TContractState, packet: Packet, ack: Acknowledgement);
    fn on_timeout_packet(ref self: TContractState, packet: Packet);
}
