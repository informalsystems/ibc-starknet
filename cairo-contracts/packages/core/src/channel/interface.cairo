use starknet_ibc_core::channel::{
    Acknowledgement, AppVersion, ChannelEnd, ChannelOrdering, MsgAckPacket, MsgChanOpenAck,
    MsgChanOpenConfirm, MsgChanOpenInit, MsgChanOpenTry, MsgRecvPacket, MsgTimeoutPacket, Packet,
};
use starknet_ibc_core::commitment::Commitment;
use starknet_ibc_core::host::{ChannelId, ConnectionId, PortId, Sequence};

#[starknet::interface]
pub trait IChannelHandler<TContractState> {
    fn chan_open_init(ref self: TContractState, msg: MsgChanOpenInit) -> ChannelId;
    fn chan_open_try(ref self: TContractState, msg: MsgChanOpenTry) -> ChannelId;
    fn chan_open_ack(ref self: TContractState, msg: MsgChanOpenAck);
    fn chan_open_confirm(ref self: TContractState, msg: MsgChanOpenConfirm);
    fn send_packet(ref self: TContractState, packet: Packet);
    fn recv_packet(ref self: TContractState, msg: MsgRecvPacket);
    fn ack_packet(ref self: TContractState, msg: MsgAckPacket);
    fn timeout_packet(ref self: TContractState, msg: MsgTimeoutPacket);
}

#[starknet::interface]
pub trait IAppCallback<TContractState> {
    fn on_chan_open_init(
        ref self: TContractState,
        port_id_on_a: PortId,
        chan_id_on_a: ChannelId,
        conn_id_on_a: ConnectionId,
        port_id_on_b: PortId,
        version_proposal: AppVersion,
        ordering: ChannelOrdering,
    ) -> AppVersion;
    fn on_chan_open_try(
        ref self: TContractState,
        port_id_on_b: PortId,
        chan_id_on_b: ChannelId,
        conn_id_on_b: ConnectionId,
        port_id_on_a: PortId,
        version_on_a: AppVersion,
        ordering: ChannelOrdering,
    ) -> AppVersion;
    fn on_chan_open_ack(
        ref self: TContractState,
        port_id_on_a: PortId,
        chan_id_on_a: ChannelId,
        version_on_b: AppVersion,
    );
    fn on_chan_open_confirm(
        ref self: TContractState, port_id_on_b: PortId, chan_id_on_b: ChannelId,
    );
    fn on_recv_packet(ref self: TContractState, packet: Packet) -> Acknowledgement;
    fn on_ack_packet(ref self: TContractState, packet: Packet, ack: Acknowledgement);
    fn on_timeout_packet(ref self: TContractState, packet: Packet);
    /// Calls for the JSON representation of the packet data, typically used for
    /// computing the packet commitment.
    fn json_packet_data(self: @TContractState, raw_packet_data: Array<felt252>) -> Array<u8>;
}

#[starknet::interface]
pub trait IChannelQuery<TContractState> {
    fn channel_end(self: @TContractState, port_id: PortId, channel_id: ChannelId) -> ChannelEnd;
    fn packet_commitment(
        self: @TContractState, port_id: PortId, channel_id: ChannelId, sequence: Sequence,
    ) -> Option<Commitment>;
    fn packet_receipt(
        self: @TContractState, port_id: PortId, channel_id: ChannelId, sequence: Sequence,
    ) -> bool;
    fn packet_acknowledgement(
        self: @TContractState, port_id: PortId, channel_id: ChannelId, sequence: Sequence,
    ) -> Commitment;
    /// Returns all committed packet sequences pending finalization on the counterparty chain.
    fn packet_commitment_sequences(
        self: @TContractState, port_id: PortId, channel_id: ChannelId,
    ) -> Array<Sequence>;
    /// Returns sequences from the given list that are acknowledged (finalized) on this chain.
    fn packet_ack_sequences(
        self: @TContractState, port_id: PortId, channel_id: ChannelId, sequences: Array<Sequence>,
    ) -> Array<Sequence>;
    fn unreceived_packet_sequences(
        self: @TContractState, port_id: PortId, channel_id: ChannelId, sequences: Array<Sequence>,
    ) -> Array<Sequence>;
    fn unreceived_ack_sequences(
        self: @TContractState, port_id: PortId, channel_id: ChannelId, sequences: Array<Sequence>,
    ) -> Array<Sequence>;
    fn next_sequence_send(
        self: @TContractState, port_id: PortId, channel_id: ChannelId,
    ) -> Sequence;
    fn next_sequence_recv(
        self: @TContractState, port_id: PortId, channel_id: ChannelId,
    ) -> Sequence;
}
