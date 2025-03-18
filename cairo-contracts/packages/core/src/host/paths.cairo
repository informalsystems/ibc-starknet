use starknet_ibc_core::host::{
    ACKS_PREFIX, BasePrefix, CHANNELS_PREFIX, CHANNEL_ENDS_PREFIX, COMMITMENTS_PREFIX,
    CONNECTIONS_PREFIX, ChannelId, ConnectionId, NEXT_SEQ_RECV_PREFIX, PORTS_PREFIX, PortId,
    RECEIPTS_PREFIX, SEQUENCES_PREFIX, Sequence,
};
use starknet_ibc_utils::{RemotePathBuilder, RemotePathBuilderImpl};

pub fn connection_path(base: BasePrefix, connection_id: ConnectionId) -> ByteArray {
    let mut builder = RemotePathBuilderImpl::init(base);
    append_prefix(ref builder, CONNECTIONS_PREFIX());
    builder.append(connection_id);
    builder.path()
}

pub fn channel_end_path(base: BasePrefix, port_id: PortId, channel_id: ChannelId) -> ByteArray {
    let mut builder = RemotePathBuilderImpl::init(base);
    append_prefix(ref builder, CHANNEL_ENDS_PREFIX());
    append_port(ref builder, port_id);
    append_channel(ref builder, channel_id);
    builder.path()
}

/// Constructs the commitment path of the counterparty chain for the given port
/// ID, channel ID, and sequence.
pub fn commitment_path(
    base: BasePrefix, port_id: PortId, channel_id: ChannelId, sequence: Sequence,
) -> ByteArray {
    let mut builder = RemotePathBuilderImpl::init(base);
    append_prefix(ref builder, COMMITMENTS_PREFIX());
    append_port(ref builder, port_id);
    append_channel(ref builder, channel_id);
    append_sequence(ref builder, sequence);
    builder.path()
}

/// Constructs the receipt path of the counterparty chain for the given port ID,
/// channel ID, and sequence.
pub fn receipt_path(
    base: BasePrefix, port_id: PortId, channel_id: ChannelId, sequence: Sequence,
) -> ByteArray {
    let mut builder = RemotePathBuilderImpl::init(base);
    append_prefix(ref builder, RECEIPTS_PREFIX());
    append_port(ref builder, port_id);
    append_channel(ref builder, channel_id);
    append_sequence(ref builder, sequence);
    builder.path()
}

/// Constructs the ack path of the counterparty chain for the given port ID,
/// channel ID, and sequence.
pub fn ack_path(
    base: BasePrefix, port_id: PortId, channel_id: ChannelId, sequence: Sequence,
) -> ByteArray {
    let mut builder = RemotePathBuilderImpl::init(base);
    append_prefix(ref builder, ACKS_PREFIX());
    append_port(ref builder, port_id);
    append_channel(ref builder, channel_id);
    append_sequence(ref builder, sequence);
    builder.path()
}

/// Constructs the next sequence send path for the given port ID and channel ID.
pub fn next_sequence_recv_path(
    base: BasePrefix, port_id: PortId, channel_id: ChannelId,
) -> ByteArray {
    let mut builder = RemotePathBuilderImpl::init(base);
    append_prefix(ref builder, NEXT_SEQ_RECV_PREFIX());
    append_port(ref builder, port_id);
    append_channel(ref builder, channel_id);
    builder.path()
}

pub fn append_prefix(ref path_builder: RemotePathBuilder, prefix: ByteArray) {
    path_builder.append(prefix);
}

pub fn append_port(ref path_builder: RemotePathBuilder, port_id: PortId) {
    path_builder.append(PORTS_PREFIX());
    path_builder.append(port_id);
}

pub fn append_channel(ref path_builder: RemotePathBuilder, channel_id: ChannelId) {
    path_builder.append(CHANNELS_PREFIX());
    path_builder.append(channel_id);
}

pub fn append_sequence(ref path_builder: RemotePathBuilder, sequence: Sequence) {
    path_builder.append(SEQUENCES_PREFIX());
    path_builder.append(sequence);
}

#[cfg(test)]
mod tests {
    use starknet_ibc_core::host::{BasePrefixZero, ChannelId, PortId, Sequence};
    use starknet_ibc_testkit::dummies::IBC_PREFIX;
    use starknet_ibc_utils::RemotePathBuilderImpl;
    use super::commitment_path;

    #[test]
    fn test_commitment_path() {
        let port_id = PortId { port_id: "transfer" };
        let channel_id = ChannelId { channel_id: "channel-0" };
        let sequence = Sequence { sequence: 0 };

        let path = commitment_path(
            BasePrefixZero::zero(), port_id.clone(), channel_id.clone(), sequence.clone(),
        );
        assert_eq!(path, "commitments/ports/transfer/channels/channel-0/sequences/0");

        let path = commitment_path(IBC_PREFIX(), port_id, channel_id, sequence);
        assert_eq!(path, "Ibc/commitments/ports/transfer/channels/channel-0/sequences/0");
    }
}
