use starknet_ibc_core::host::{
    CHANNEL_ENDS_PREFIX, PORTS_PREFIX, CHANNELS_PREFIX, SEQUENCES_PREFIX, RECEIPTS_PREFIX,
    ACKS_PREFIX, NEXT_SEQ_RECV_PREFIX
};
use starknet_ibc_core::host::{ChannelId, PortId, Sequence};
use starknet_ibc_utils::LocalKeyBuilderTrait;
use starknet_ibc_utils::{LocalKeyBuilderImpl, LocalKeyBuilder};

pub fn channel_end_key(port_id: @PortId, channel_id: @ChannelId) -> felt252 {
    let mut key_builder = LocalKeyBuilderImpl::init();
    key_builder.append_serde(@CHANNEL_ENDS_PREFIX());
    append_serde_port(ref key_builder, port_id);
    append_serde_channel(ref key_builder, channel_id);
    key_builder.key()
}

pub fn receipt_key(port_id: @PortId, channel_id: @ChannelId, sequence: @Sequence) -> felt252 {
    let mut key_builder = LocalKeyBuilderImpl::init();
    key_builder.append_serde(@RECEIPTS_PREFIX());
    append_serde_port(ref key_builder, port_id);
    append_serde_channel(ref key_builder, channel_id);
    append_serde_sequence(ref key_builder, sequence);
    key_builder.key()
}

/// Constructs the next receive sequence local key for the given port ID and channel ID.
pub fn next_sequence_recv_key(port_id: @PortId, channel_id: @ChannelId) -> felt252 {
    let mut key_builder = LocalKeyBuilderImpl::init();
    key_builder.append_serde(@NEXT_SEQ_RECV_PREFIX());
    append_serde_port(ref key_builder, port_id);
    append_serde_channel(ref key_builder, channel_id);
    key_builder.key()
}


pub fn ack_key(port_id: @PortId, channel_id: @ChannelId, sequence: @Sequence) -> felt252 {
    let mut key_builder = LocalKeyBuilderImpl::init();
    key_builder.append_serde(@ACKS_PREFIX());
    append_serde_port(ref key_builder, port_id);
    append_serde_channel(ref key_builder, channel_id);
    append_serde_sequence(ref key_builder, sequence);
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
