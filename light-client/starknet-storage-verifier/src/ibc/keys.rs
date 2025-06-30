use ibc_core::host::types::identifiers::{ChannelId, ConnectionId, PortId, Sequence};
use starknet_crypto::{poseidon_hash_many, Felt};

use super::utils::{serialize_byte_array, serialize_to_felts};

pub fn connection_key(connection_id: ConnectionId) -> Felt {
    let mut raw_path: Vec<Felt> = vec![];
    raw_path.extend(serialize_byte_array(b"connections"));
    raw_path.extend(serialize_byte_array(connection_id.as_bytes()));

    poseidon_hash_many(&raw_path)
}

pub fn next_sequence_key(prefix: &str, port_id: PortId, channel_id: ChannelId) -> Felt {
    let mut raw_path: Vec<Felt> = vec![];
    raw_path.extend(serialize_byte_array(prefix.as_bytes()));
    raw_path.extend(serialize_byte_array("ports".as_bytes()));
    raw_path.extend(serialize_byte_array(port_id.as_bytes()));
    raw_path.extend(serialize_byte_array("channels".as_bytes()));
    raw_path.extend(serialize_byte_array(channel_id.as_bytes()));

    poseidon_hash_many(&raw_path)
}

pub fn packet_key(
    prefix: &str,
    port_id: PortId,
    channel_id: ChannelId,
    sequence: Sequence,
) -> Felt {
    let mut raw_path: Vec<Felt> = vec![];
    raw_path.extend(serialize_byte_array(prefix.as_bytes()));
    raw_path.extend(serialize_byte_array("ports".as_bytes()));
    raw_path.extend(serialize_byte_array(port_id.as_bytes()));
    raw_path.extend(serialize_byte_array("channels".as_bytes()));
    raw_path.extend(serialize_byte_array(channel_id.as_bytes()));
    raw_path.extend(serialize_byte_array("sequences".as_bytes()));
    raw_path.extend(serialize_to_felts(sequence.to_vec().as_slice()));

    poseidon_hash_many(&raw_path)
}
