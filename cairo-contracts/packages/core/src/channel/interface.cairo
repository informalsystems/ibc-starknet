use starknet_ibc_core::channel::{Packet, MsgRecvPacket, MsgAckPacket, Acknowledgement, ChannelEnd};
use starknet_ibc_core::commitment::CommitmentValue;
use starknet_ibc_core::host::{PortId, ChannelId, Sequence};

#[starknet::interface]
pub trait IChannelHandler<TContractState> {
    fn send_packet(ref self: TContractState, packet: Packet);
    fn recv_packet(ref self: TContractState, msg: MsgRecvPacket);
    fn ack_packet(ref self: TContractState, msg: MsgAckPacket);
}

#[starknet::interface]
pub trait IAppCallback<TContractState> {
    fn on_recv_packet(ref self: TContractState, packet: Packet) -> Acknowledgement;
    fn on_ack_packet(ref self: TContractState, packet: Packet, ack: Acknowledgement);
    fn on_timeout_packet(ref self: TContractState, packet: Packet);
    /// Calls for the JSON representation of the packet data, typically used for
    /// computing the packet commitment.
    fn json_packet_data(self: @TContractState, raw_packet_data: Array<felt252>) -> ByteArray;
}

#[starknet::interface]
pub trait IChannelQuery<TContractState> {
    fn channel_end(self: @TContractState, port_id: PortId, channel_id: ChannelId) -> ChannelEnd;
    fn packet_commitment(
        self: @TContractState, port_id: PortId, channel_id: ChannelId, sequence: Sequence
    ) -> CommitmentValue;
    fn packet_receipt(
        self: @TContractState, port_id: PortId, channel_id: ChannelId, sequence: Sequence
    ) -> bool;
    fn packet_acknowledgement(
        self: @TContractState, port_id: PortId, channel_id: ChannelId, sequence: Sequence
    ) -> CommitmentValue;
    fn next_sequence_send(
        self: @TContractState, port_id: PortId, channel_id: ChannelId
    ) -> Sequence;
    fn next_sequence_recv(
        self: @TContractState, port_id: PortId, channel_id: ChannelId
    ) -> Sequence;
}
