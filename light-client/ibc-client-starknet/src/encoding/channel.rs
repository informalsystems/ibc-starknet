use alloc::vec;
use alloc::vec::Vec;

use ibc_core::channel::types::proto::v1::Channel;
use starknet_crypto::Felt;

use crate::encoding::utils::packed_bytes_to_felt;

pub fn channel_to_felts(channel: &Channel) -> Vec<Felt> {
    let mut felts = vec![];

    // 1. state
    felts.push(Felt::from(channel.state));

    // 2. ordering
    felts.push(Felt::ZERO);

    // 3. counterparty
    felts.push(Felt::ZERO);
    if let Some(counterparty) = channel.counterparty.as_ref() {
        let port_id_felt = packed_bytes_to_felt(counterparty.port_id.as_bytes());
        felts.push(port_id_felt);
        felts.push(Felt::from(counterparty.port_id.len()));
        felts.push(Felt::ZERO);
        let channel_id_felt = packed_bytes_to_felt(counterparty.channel_id.as_bytes());
        felts.push(channel_id_felt);
        felts.push(Felt::from(counterparty.channel_id.len()));
    }
    felts.push(Felt::ZERO);

    // 4. connection_hops
    for connection_id in channel.connection_hops.iter() {
        let connection_id_felt = packed_bytes_to_felt(connection_id.as_bytes());
        felts.push(connection_id_felt);
        felts.push(Felt::from(connection_id.len()));
    }
    felts.push(Felt::ZERO);

    // 5. version
    let prefix_felt = packed_bytes_to_felt(channel.version.as_bytes());
    felts.push(prefix_felt);
    felts.push(Felt::from(channel.version.len()));

    felts
}
