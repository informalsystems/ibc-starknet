pub use core::sha256::{compute_sha256_byte_array, compute_sha256_u32_array,};
use starknet_ibc_core::channel::Acknowledgement;
use starknet_ibc_core::client::{Height, Timestamp};
use starknet_ibc_utils::{U32CollectorImpl, array_u8_into_array_u32};

pub fn compute_packet_commtiment(
    json_packet_data: @ByteArray, timeout_height: Height, timeout_timestamp: Timestamp
    ) -> [u32; 8] {
    let mut coll = U32CollectorImpl::init();
    coll.extend(timeout_timestamp);
    coll.extend(timeout_height);
    coll.extend_from_chunk(compute_sha256_byte_array(json_packet_data));
    compute_sha256_u32_array(coll.value(), 0, 0)
}

pub fn compute_ack_commitment(ack: Acknowledgement) -> [u32; 8] {
    compute_sha256_u32_array(array_u8_into_array_u32(ack.ack), 0, 0)
}
