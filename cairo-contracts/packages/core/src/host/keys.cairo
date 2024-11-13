use starknet_ibc_core::host::{
    CHANNEL_ENDS_PREFIX, PORTS_PREFIX, CLIENTS_PREFIX, CONNECTIONS_PREFIX, CHANNELS_PREFIX,
    SEQUENCES_PREFIX, COMMITMENTS_PREFIX, RECEIPTS_PREFIX, ACKS_PREFIX, NEXT_SEQ_RECV_PREFIX,
    NEXT_SEQ_SEND_PREFIX, NEXT_SEQ_ACK_PREFIX
};
use starknet_ibc_core::host::{ClientId, ConnectionId, ChannelId, PortId, Sequence};
use starknet_ibc_utils::LocalKeyBuilderTrait;
use starknet_ibc_utils::{LocalKeyBuilderImpl, LocalKeyBuilder};

/// Constructs the client to connections local key for the given client ID.
pub fn client_connection_key(client_id: @ClientId) -> felt252 {
    let mut key_builder = LocalKeyBuilderImpl::init();
    key_builder.append_serde(@CLIENTS_PREFIX());
    key_builder.append_serde(client_id);
    key_builder.append_serde(@CONNECTIONS_PREFIX());
    key_builder.key()
}

/// Constructs the connection end local key for the given connection ID.
pub fn connection_end_key(connection_id: @ConnectionId) -> felt252 {
    let mut key_builder = LocalKeyBuilderImpl::init();
    key_builder.append_serde(@CONNECTIONS_PREFIX());
    key_builder.append_serde(connection_id);
    key_builder.key()
}

/// Constructs the channel end local key for the given port ID and channel ID.
pub fn channel_end_key(port_id: @PortId, channel_id: @ChannelId) -> felt252 {
    let mut key_builder = LocalKeyBuilderImpl::init();
    key_builder.append_serde(@CHANNEL_ENDS_PREFIX());
    append_serde_port(ref key_builder, port_id);
    append_serde_channel(ref key_builder, channel_id);
    key_builder.key()
}

/// Constructs the receipt local key for the given port ID, channel ID, and sequence.
pub fn commitment_key(port_id: @PortId, channel_id: @ChannelId, sequence: @Sequence) -> felt252 {
    let mut key_builder = LocalKeyBuilderImpl::init();
    key_builder.append_serde(@COMMITMENTS_PREFIX());
    append_serde_port(ref key_builder, port_id);
    append_serde_channel(ref key_builder, channel_id);
    append_serde_sequence(ref key_builder, sequence);
    key_builder.key()
}

/// Constructs the receipt local key for the given port ID, channel ID, and sequence.
pub fn receipt_key(port_id: @PortId, channel_id: @ChannelId, sequence: @Sequence) -> felt252 {
    let mut key_builder = LocalKeyBuilderImpl::init();
    key_builder.append_serde(@RECEIPTS_PREFIX());
    append_serde_port(ref key_builder, port_id);
    append_serde_channel(ref key_builder, channel_id);
    append_serde_sequence(ref key_builder, sequence);
    key_builder.key()
}

/// Constructs the acknowledgement local key for the given port ID, channel ID, and sequence.
pub fn ack_key(port_id: @PortId, channel_id: @ChannelId, sequence: @Sequence) -> felt252 {
    let mut key_builder = LocalKeyBuilderImpl::init();
    key_builder.append_serde(@ACKS_PREFIX());
    append_serde_port(ref key_builder, port_id);
    append_serde_channel(ref key_builder, channel_id);
    append_serde_sequence(ref key_builder, sequence);
    key_builder.key()
}

pub fn next_sequence_send_key(port_id: @PortId, channel_id: @ChannelId) -> felt252 {
    let mut key_builder = LocalKeyBuilderImpl::init();
    key_builder.append_serde(@NEXT_SEQ_SEND_PREFIX());
    append_serde_port(ref key_builder, port_id);
    append_serde_channel(ref key_builder, channel_id);
    key_builder.key()
}

/// Constructs the next sequence receive local key for the given port ID and channel ID.
pub fn next_sequence_recv_key(port_id: @PortId, channel_id: @ChannelId) -> felt252 {
    let mut key_builder = LocalKeyBuilderImpl::init();
    key_builder.append_serde(@NEXT_SEQ_RECV_PREFIX());
    append_serde_port(ref key_builder, port_id);
    append_serde_channel(ref key_builder, channel_id);
    key_builder.key()
}

pub fn next_sequence_ack_key(port_id: @PortId, channel_id: @ChannelId) -> felt252 {
    let mut key_builder = LocalKeyBuilderImpl::init();
    key_builder.append_serde(@NEXT_SEQ_ACK_PREFIX());
    append_serde_port(ref key_builder, port_id);
    append_serde_channel(ref key_builder, channel_id);
    key_builder.key()
}


pub fn append_serde_port(ref key_builder: LocalKeyBuilder, port_id: @PortId) {
    key_builder.append_serde(@PORTS_PREFIX());
    key_builder.append_serde(port_id);
}

pub fn append_serde_channel(ref key_builder: LocalKeyBuilder, channel_id: @ChannelId) {
    key_builder.append_serde(@CHANNELS_PREFIX());
    key_builder.append_serde(channel_id);
}

pub fn append_serde_sequence(ref key_builder: LocalKeyBuilder, sequence: @Sequence) {
    key_builder.append_serde(@SEQUENCES_PREFIX());
    key_builder.append_serde(sequence);
}
