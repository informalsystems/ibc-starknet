use core::num::traits::Zero;
use core::sha256::{compute_sha256_byte_array, compute_sha256_u32_array};
use starknet::SyscallResult;
use starknet::storage_access::{Store, StorageBaseAddress};
use starknet_ibc_core::channel::Acknowledgement;
use starknet_ibc_core::client::{Height, Timestamp};
use starknet_ibc_utils::{U32CollectorImpl, array_u8_into_array_u32};

// -----------------------------------------------------------
// Commitment Value
// -----------------------------------------------------------

#[derive(Clone, Debug, Drop, PartialEq, Serde)]
pub struct CommitmentValue {
    pub value: [u32; 8],
}

pub impl FixedU32ArrayIntoCommitmentValue of Into<[u32; 8], CommitmentValue> {
    fn into(self: [u32; 8]) -> CommitmentValue {
        CommitmentValue { value: self }
    }
}

pub impl CommitmentValueZero of Zero<CommitmentValue> {
    fn zero() -> CommitmentValue {
        CommitmentValue { value: [0; 8] }
    }

    fn is_zero(self: @CommitmentValue) -> bool {
        self.value == @[0; 8]
    }

    fn is_non_zero(self: @CommitmentValue) -> bool {
        !self.is_zero()
    }
}

pub impl DigestStore of Store<CommitmentValue> {
    fn read(address_domain: u32, base: StorageBaseAddress) -> SyscallResult<CommitmentValue> {
        match Store::<[u32; 8]>::read(address_domain, base) {
            Result::Ok(value) => Result::Ok(CommitmentValue { value }),
            Result::Err(err) => Result::Err(err),
        }
    }

    fn write(
        address_domain: u32, base: StorageBaseAddress, value: CommitmentValue
    ) -> SyscallResult<()> {
        Store::<[u32; 8]>::write(address_domain, base, value.value)
    }

    fn read_at_offset(
        address_domain: u32, base: StorageBaseAddress, offset: u8
    ) -> SyscallResult<CommitmentValue> {
        match Store::<[u32; 8]>::read_at_offset(address_domain, base, offset) {
            Result::Ok(value) => Result::Ok(CommitmentValue { value }),
            Result::Err(err) => Result::Err(err),
        }
    }

    fn write_at_offset(
        address_domain: u32, base: StorageBaseAddress, offset: u8, value: CommitmentValue
    ) -> SyscallResult<()> {
        Store::<[u32; 8]>::write_at_offset(address_domain, base, offset, value.value)
    }

    fn size() -> u8 {
        Store::<[u32; 8]>::size()
    }
}

pub fn compute_packet_commtiment(
    json_packet_data: @ByteArray, timeout_height: Height, timeout_timestamp: Timestamp
) -> CommitmentValue {
    let mut coll = U32CollectorImpl::init();
    coll.extend(timeout_timestamp);
    coll.extend(timeout_height);
    coll.extend_from_chunk(compute_sha256_byte_array(json_packet_data));
    compute_sha256_u32_array(coll.value(), 0, 0).into()
}

pub fn compute_ack_commitment(ack: Acknowledgement) -> CommitmentValue {
    compute_sha256_u32_array(array_u8_into_array_u32(ack.ack), 0, 0).into()
}

// -----------------------------------------------------------
// Commitment Proof
// -----------------------------------------------------------

/// Contains the commitment proof bytes serving to verify membership or
/// non-membership for an element or set of elements, in conjunction with
/// a known commitment root
#[derive(Clone, Debug, Drop, PartialEq, Serde)]
pub struct CommitmentProof {
    pub proof: Array<u8>,
}

pub impl ArrayU8IntoProof of Into<Array<u8>, CommitmentProof> {
    fn into(self: Array<u8>) -> CommitmentProof {
        CommitmentProof { proof: self }
    }
}

pub impl CommitmentProofZero of Zero<CommitmentProof> {
    fn zero() -> CommitmentProof {
        CommitmentProof { proof: ArrayTrait::new() }
    }

    fn is_zero(self: @CommitmentProof) -> bool {
        self.proof.len() == 0
    }

    fn is_non_zero(self: @CommitmentProof) -> bool {
        self.proof.len() > 0
    }
}

