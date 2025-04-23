use starknet_ibc_core::host::{
    ACKS_PREFIX, BasePrefix, CHANNELS_PREFIX, CHANNEL_ENDS_PREFIX, COMMITMENTS_PREFIX,
    CONNECTIONS_PREFIX, ChannelId, ConnectionId, NEXT_SEQ_RECV_PREFIX, PORTS_PREFIX, PortId,
    RECEIPTS_PREFIX, SEQUENCES_PREFIX, Sequence, UPGRADED_CLIENT_STATE_SUFFIX,
    UPGRADED_CONSENSUS_STATE_SUFFIX, UPGRADED_IBC_STATE_PREFIX,
};
use starknet_ibc_utils::{RemotePathBuilder, RemotePathBuilderImpl};

pub fn connection_path(base: BasePrefix, connection_id: ConnectionId) -> Array<ByteArray> {
    let mut paths = array![];
    let mut builder_prefix = RemotePathBuilderImpl::init(base);
    let prefix = builder_prefix.path();
    let mut builder = RemotePathBuilderImpl::init(CONNECTIONS_PREFIX());
    builder.append(connection_id);
    let path = builder.path();
    paths.append(prefix);
    paths.append(path);
    paths
}

pub fn channel_end_path(
    base: BasePrefix, port_id: PortId, channel_id: ChannelId,
) -> Array<ByteArray> {
    let mut paths = array![];
    let mut builder_prefix = RemotePathBuilderImpl::init(base);
    let prefix = builder_prefix.path();
    let mut builder = RemotePathBuilderImpl::init(CHANNEL_ENDS_PREFIX());
    append_port(ref builder, port_id);
    append_channel(ref builder, channel_id);
    let path = builder.path();
    paths.append(prefix);
    paths.append(path);
    paths
}

/// Constructs the commitment path of the counterparty chain for the given port
/// ID, channel ID, and sequence.
pub fn commitment_path(
    base: BasePrefix, port_id: PortId, channel_id: ChannelId, sequence: Sequence,
) -> Array<ByteArray> {
    let mut paths = array![];
    let mut builder_prefix = RemotePathBuilderImpl::init(base);
    let prefix = builder_prefix.path();
    let mut builder = RemotePathBuilderImpl::init(COMMITMENTS_PREFIX());
    append_port(ref builder, port_id);
    append_channel(ref builder, channel_id);
    append_sequence(ref builder, sequence);
    let path = builder.path();
    paths.append(prefix);
    paths.append(path);
    paths
}

/// Constructs the receipt path of the counterparty chain for the given port ID,
/// channel ID, and sequence.
pub fn receipt_path(
    base: BasePrefix, port_id: PortId, channel_id: ChannelId, sequence: Sequence,
) -> Array<ByteArray> {
    let mut paths = array![];
    let mut builder_prefix = RemotePathBuilderImpl::init(base);
    let prefix = builder_prefix.path();
    let mut builder = RemotePathBuilderImpl::init(RECEIPTS_PREFIX());
    append_port(ref builder, port_id);
    append_channel(ref builder, channel_id);
    append_sequence(ref builder, sequence);
    let path = builder.path();
    paths.append(prefix);
    paths.append(path);
    paths
}

/// Constructs the ack path of the counterparty chain for the given port ID,
/// channel ID, and sequence.
pub fn ack_path(
    base: BasePrefix, port_id: PortId, channel_id: ChannelId, sequence: Sequence,
) -> Array<ByteArray> {
    let mut paths = array![];
    let mut builder_prefix = RemotePathBuilderImpl::init(base);
    let prefix = builder_prefix.path();
    let mut builder = RemotePathBuilderImpl::init(ACKS_PREFIX());
    append_port(ref builder, port_id);
    append_channel(ref builder, channel_id);
    append_sequence(ref builder, sequence);
    let path = builder.path();
    paths.append(prefix);
    paths.append(path);
    paths
}

/// Constructs the next sequence send path for the given port ID and channel ID.
pub fn next_sequence_recv_path(
    base: BasePrefix, port_id: PortId, channel_id: ChannelId,
) -> Array<ByteArray> {
    let mut paths = array![];
    let mut builder_prefix = RemotePathBuilderImpl::init(base);
    let prefix = builder_prefix.path();
    let mut builder = RemotePathBuilderImpl::init(NEXT_SEQ_RECV_PREFIX());
    append_port(ref builder, port_id);
    append_channel(ref builder, channel_id);
    let path = builder.path();
    paths.append(prefix);
    paths.append(path);
    paths
}

/// Constructs the client upgrade path for the given height
pub fn client_upgrade_path(base: BasePrefix, height: u64) -> Array<ByteArray> {
    let mut builder_prefix = RemotePathBuilderImpl::init(base);
    let prefix = builder_prefix.path();
    let mut builder = RemotePathBuilderImpl::init(UPGRADED_IBC_STATE_PREFIX());
    builder.append(format!("{}", height));
    builder.append(UPGRADED_CLIENT_STATE_SUFFIX());
    let path = builder.path();
    let mut paths = array![];
    paths.append(prefix);
    paths.append(path);
    paths
}

/// Constructs the consensus upgrade path for the given height
pub fn consensus_upgrade_path(base: BasePrefix, height: u64) -> Array<ByteArray> {
    let mut builder_prefix = RemotePathBuilderImpl::init(base);
    let prefix = builder_prefix.path();
    let mut builder = RemotePathBuilderImpl::init(UPGRADED_IBC_STATE_PREFIX());
    builder.append(format!("{}", height));
    builder.append(UPGRADED_CONSENSUS_STATE_SUFFIX());
    let path = builder.path();
    let mut paths = array![];
    paths.append(prefix);
    paths.append(path);
    paths
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
    use starknet_ibc_testkit::dummies::{
        CHANNEL_END, CHANNEL_ID, CONNECTION_END, IBC_PREFIX, PORT_ID, SEQUENCE,
    };
    use starknet_ibc_utils::RemotePathBuilderImpl;
    use super::{
        ack_path, channel_end_path, commitment_path, connection_path, next_sequence_recv_path,
        receipt_path,
    };

    #[test]
    fn test_connection_path() {
        let connection_end = CONNECTION_END(0);

        let paths = connection_path(
            connection_end.counterparty.prefix.clone(),
            connection_end.counterparty.connection_id.clone(),
        );
        assert_eq!(paths, array!["ibc", "connections/connection-0"]);
    }

    #[test]
    fn test_channel_path() {
        let connection_end = CONNECTION_END(0);
        let channel_end = CHANNEL_END(0);

        let paths = channel_end_path(
            connection_end.counterparty.prefix.clone(),
            channel_end.remote.port_id.clone(),
            channel_end.remote.channel_id.clone(),
        );
        assert_eq!(paths, array!["ibc", "channelEnds/ports/transfer/channels/channel-0"]);
    }

    #[test]
    fn test_commitment_path() {
        let port_id = PortId { port_id: "transfer" };
        let channel_id = ChannelId { channel_id: "channel-0" };
        let sequence = Sequence { sequence: 0 };

        let paths = commitment_path(
            BasePrefixZero::zero(), port_id.clone(), channel_id.clone(), sequence.clone(),
        );
        // TODO: what is the expected behaviour if the prefix is empty?
        assert_eq!(paths, array!["", "commitments/ports/transfer/channels/channel-0/sequences/0"]);

        let paths = commitment_path(IBC_PREFIX(), port_id, channel_id, sequence);
        assert_eq!(
            paths, array!["ibc", "commitments/ports/transfer/channels/channel-0/sequences/0"],
        );
    }

    #[test]
    fn test_receipt_path() {
        let connection_end = CONNECTION_END(0);

        let paths = receipt_path(
            connection_end.counterparty.prefix.clone(), PORT_ID(), CHANNEL_ID(1), SEQUENCE(3),
        );
        assert_eq!(paths, array!["ibc", "receipts/ports/transfer/channels/channel-1/sequences/3"]);
    }

    #[test]
    fn test_ack_path() {
        let connection_end = CONNECTION_END(0);

        let paths = ack_path(
            connection_end.counterparty.prefix.clone(), PORT_ID(), CHANNEL_ID(2), SEQUENCE(4),
        );
        assert_eq!(paths, array!["ibc", "acks/ports/transfer/channels/channel-2/sequences/4"]);
    }

    #[test]
    fn test_next_sequence_recv_path() {
        let connection_end = CONNECTION_END(0);

        let paths = next_sequence_recv_path(
            connection_end.counterparty.prefix.clone(), PORT_ID(), CHANNEL_ID(3),
        );
        assert_eq!(paths, array!["ibc", "nextSequenceRecv/ports/transfer/channels/channel-3"]);
    }

    #[test]
    fn test_client_upgrade_path() {
        let paths = super::client_upgrade_path(IBC_PREFIX(), 1);
        assert_eq!(paths, array!["ibc", "upgradedIBCState/1/upgradedClient"]);
    }

    #[test]
    fn test_consensus_upgrade_path() {
        let paths = super::consensus_upgrade_path(IBC_PREFIX(), 1);
        assert_eq!(paths, array!["ibc", "upgradedIBCState/1/upgradedConsState"]);
    }
}
