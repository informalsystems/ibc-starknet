use starknet_ibc_core::host::{ChannelId, PortId, Sequence};
use starknet_ibc_utils::RemotePathBuilderImpl;

pub fn CHANNEL_PREFIX() -> ByteArray {
    "channels"
}

pub fn PORT_PREFIX() -> ByteArray {
    "ports"
}

pub fn SEQUENCE_PREFIX() -> ByteArray {
    "sequences"
}

pub fn COMMITMENT_PATH_PREFIX() -> ByteArray {
    "commitments"
}

/// Constructs the commitment path of the counterparty chain for the given port
/// ID, channel ID, and sequence.
pub fn commitment_path(port_id: PortId, channel_id: ChannelId, sequence: Sequence) -> ByteArray {
    let mut key_builder = RemotePathBuilderImpl::init();
    key_builder.append(COMMITMENT_PATH_PREFIX());
    key_builder.append(PORT_PREFIX());
    key_builder.append(port_id);
    key_builder.append(CHANNEL_PREFIX());
    key_builder.append(channel_id);
    key_builder.append(SEQUENCE_PREFIX());
    key_builder.append(sequence);
    key_builder.path()
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
