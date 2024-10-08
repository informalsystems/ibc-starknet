use starknet_ibc_core::host::{
    COMMITMENTS_PREFIX, ACKS_PREFIX, PORTS_PREFIX, CHANNELS_PREFIX, SEQUENCES_PREFIX
};
use starknet_ibc_core::host::{ChannelId, PortId, Sequence};
use starknet_ibc_utils::{RemotePathBuilder, RemotePathBuilderImpl};

/// Constructs the commitment path of the counterparty chain for the given port
/// ID, channel ID, and sequence.
pub fn commitment_path(port_id: PortId, channel_id: ChannelId, sequence: Sequence) -> ByteArray {
    let mut key_builder = RemotePathBuilderImpl::init();
    append_prefix(ref key_builder, COMMITMENTS_PREFIX());
    append_port(ref key_builder, port_id);
    append_channel(ref key_builder, channel_id);
    append_sequence(ref key_builder, sequence);
    key_builder.path()
}

/// Constructs the ack path of the counterparty chain for the given port ID,
/// channel ID, and sequence.
pub fn ack_path(port_id: PortId, channel_id: ChannelId, sequence: Sequence) -> ByteArray {
    let mut key_builder = RemotePathBuilderImpl::init();
    append_prefix(ref key_builder, ACKS_PREFIX());
    append_port(ref key_builder, port_id);
    append_channel(ref key_builder, channel_id);
    append_sequence(ref key_builder, sequence);
    key_builder.path()
}

pub fn append_prefix(ref path_builer: RemotePathBuilder, prefix: ByteArray) {
    path_builer.append(prefix);
}

pub fn append_port(ref path_builer: RemotePathBuilder, port_id: PortId) {
    path_builer.append(PORTS_PREFIX());
    path_builer.append(port_id);
}

pub fn append_channel(ref path_builer: RemotePathBuilder, channel_id: ChannelId) {
    path_builer.append(CHANNELS_PREFIX());
    path_builer.append(channel_id);
}

pub fn append_sequence(ref path_builer: RemotePathBuilder, sequence: Sequence) {
    path_builer.append(SEQUENCES_PREFIX());
    path_builer.append(sequence);
}

#[cfg(test)]
mod tests {
    use starknet_ibc_core::host::{ChannelId, PortId, Sequence};
    use starknet_ibc_utils::RemotePathBuilderImpl;
    use super::commitment_path;

    #[test]
    fn test_commitment_path() {
        let port_id = PortId { port_id: "transfer" };
        let channel_id = ChannelId { channel_id: "channel-0" };
        let sequence = Sequence { sequence: 0 };
        let path = commitment_path(port_id, channel_id, sequence);
        assert_eq!(path, "commitments/ports/transfer/channels/channel-0/sequences/0");
    }
}
