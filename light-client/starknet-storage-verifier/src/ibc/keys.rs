use alloc::vec;
use alloc::vec::Vec;

use ibc_core::host::types::identifiers::{ChannelId, ConnectionId, PortId, Sequence};
use starknet_core::types::Felt;
use starknet_crypto_lib::StarknetCryptoFunctions;

use super::utils::{serialize_byte_array, serialize_to_felts};

pub fn connection_key<C: StarknetCryptoFunctions>(
    crypto_lib: &C,
    connection_id: ConnectionId,
) -> Felt {
    let mut raw_path: Vec<Felt> = vec![];
    raw_path.extend(serialize_byte_array(b"connections"));
    raw_path.extend(serialize_byte_array(connection_id.as_bytes()));

    crypto_lib.poseidon_hash_many(&raw_path)
}

pub fn next_sequence_key<C: StarknetCryptoFunctions>(
    crypto_lib: &C,
    prefix: &str,
    port_id: PortId,
    channel_id: ChannelId,
) -> Felt {
    let mut raw_path: Vec<Felt> = vec![];
    raw_path.extend(serialize_byte_array(prefix.as_bytes()));
    raw_path.extend(serialize_byte_array("ports".as_bytes()));
    raw_path.extend(serialize_byte_array(port_id.as_bytes()));
    raw_path.extend(serialize_byte_array("channels".as_bytes()));
    raw_path.extend(serialize_byte_array(channel_id.as_bytes()));

    crypto_lib.poseidon_hash_many(&raw_path)
}

pub fn packet_key<C: StarknetCryptoFunctions>(
    crypto_lib: &C,
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

    crypto_lib.poseidon_hash_many(&raw_path)
}
