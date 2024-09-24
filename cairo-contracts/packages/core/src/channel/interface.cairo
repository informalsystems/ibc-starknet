use starknet_ibc_core::channel::{Packet, MsgRecvPacket, Acknowledgement, ChannelEnd};
use starknet_ibc_core::host::{PortId, ChannelId, Sequence};

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

#[starknet::interface]
pub trait IChannelQuery<TContractState> {
    fn channel_end(self: @TContractState, port_id: PortId, channel_id: ChannelId) -> ChannelEnd;
    fn packet_receipt(
        self: @TContractState, port_id: PortId, channel_id: ChannelId, sequence: Sequence
    ) -> bool;
    fn packet_acknowledgement(
        self: @TContractState, port_id: PortId, channel_id: ChannelId, sequence: Sequence
    ) -> felt252;
    fn next_sequence_recv(
        self: @TContractState, port_id: PortId, channel_id: ChannelId
    ) -> Sequence;
}
