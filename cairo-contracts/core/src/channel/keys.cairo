use starknet_ibc_core::host::{ChannelId, PortId};
use starknet_ibc_utils::KeyBuilderImpl;

pub fn channel_end_key(port_id: @PortId, channel_id: @ChannelId) -> felt252 {
    let mut key_builder = KeyBuilderImpl::init();
    key_builder.append_serde(port_id);
    key_builder.append_serde(channel_id);
    key_builder.compute_key()
}

pub fn packet_receipt_key(port_id: @PortId, channel_id: @ChannelId, sequence: @u64) -> felt252 {
    let mut key_builder = KeyBuilderImpl::init();
    key_builder.append_serde(port_id);
    key_builder.append_serde(channel_id);
    key_builder.append_serde(sequence);
    key_builder.compute_key()
}
