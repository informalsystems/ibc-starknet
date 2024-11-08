use core::num::traits::Zero;
use core::sha256::{compute_sha256_byte_array, compute_sha256_u32_array};
use starknet::SyscallResult;
use starknet::storage_access::{Store, StorageBaseAddress};
use starknet_ibc_core::channel::Acknowledgement;
use starknet_ibc_core::client::{Height, Timestamp};
use starknet_ibc_core::commitment::{U32CollectorImpl, IntoArrayU32, array_u32_into_array_u8};

// -----------------------------------------------------------
// Commitment Value
// -----------------------------------------------------------

#[derive(Clone, Debug, Drop, PartialEq, Serde)]
pub struct Commitment {
    pub value: [u32; 8],
}

pub impl FixedU32ArrayIntoCommitment of Into<[u32; 8], Commitment> {
    fn into(self: [u32; 8]) -> Commitment {
        Commitment { value: self }
    }
}

pub impl CommitmentIntoArrayU32 of Into<Commitment, Array<u32>> {
    fn into(self: Commitment) -> Array<u32> {
        let mut value: Array<u32> = ArrayTrait::new();
        value.append_span(self.value.span());
        value
    }
}

pub impl CommitmentZero of Zero<Commitment> {
    fn zero() -> Commitment {
        Commitment { value: [0; 8] }
    }

    fn is_zero(self: @Commitment) -> bool {
        self.value == @[0; 8]
    }

    fn is_non_zero(self: @Commitment) -> bool {
        !self.is_zero()
    }
}

pub impl CommitmentIntoStateValue of Into<Commitment, StateValue> {
    fn into(self: Commitment) -> StateValue {
        let value = array_u32_into_array_u8(self.into());
        StateValue { value }
    }
}

pub impl DigestStore of Store<Commitment> {
    fn read(address_domain: u32, base: StorageBaseAddress) -> SyscallResult<Commitment> {
        match Store::<[u32; 8]>::read(address_domain, base) {
            Result::Ok(value) => Result::Ok(Commitment { value }),
            Result::Err(err) => Result::Err(err),
        }
    }

    fn write(
        address_domain: u32, base: StorageBaseAddress, value: Commitment
    ) -> SyscallResult<()> {
        Store::<[u32; 8]>::write(address_domain, base, value.value)
    }

    fn read_at_offset(
        address_domain: u32, base: StorageBaseAddress, offset: u8
    ) -> SyscallResult<Commitment> {
        match Store::<[u32; 8]>::read_at_offset(address_domain, base, offset) {
            Result::Ok(value) => Result::Ok(Commitment { value }),
            Result::Err(err) => Result::Err(err),
        }
    }

    fn write_at_offset(
        address_domain: u32, base: StorageBaseAddress, offset: u8, value: Commitment
    ) -> SyscallResult<()> {
        Store::<[u32; 8]>::write_at_offset(address_domain, base, offset, value.value)
    }

    fn size() -> u8 {
        Store::<[u32; 8]>::size()
    }
}

pub fn compute_packet_commtiment(
    json_packet_data: @ByteArray, timeout_height: Height, timeout_timestamp: Timestamp
) -> Commitment {
    let mut coll = U32CollectorImpl::init();
    coll.extend(timeout_timestamp);
    coll.extend(timeout_height);
    coll.extend_from_chunk(compute_sha256_byte_array(json_packet_data));
    compute_sha256_u32_array(coll.value(), 0, 0).into()
}

pub fn compute_ack_commitment(ack: Acknowledgement) -> Commitment {
    let (array, last_word, last_word_len) = ack.into_array_u32();
    compute_sha256_u32_array(array, last_word, last_word_len).into()
}

// -----------------------------------------------------------
// State Value
// -----------------------------------------------------------

#[derive(Clone, Debug, Drop, PartialEq, Serde)]
pub struct StateValue {
    pub value: Array<u8>,
}

pub impl ArrayU8IntoStateValue of Into<Array<u8>, StateValue> {
    fn into(self: Array<u8>) -> StateValue {
        StateValue { value: self }
    }
}

pub impl StateValueZero of Zero<StateValue> {
    fn zero() -> StateValue {
        StateValue { value: ArrayTrait::new() }
    }

    fn is_zero(self: @StateValue) -> bool {
        self.value.len() == 0
    }

    fn is_non_zero(self: @StateValue) -> bool {
        self.value.len() > 0
    }
}

// -----------------------------------------------------------
// State Proof
// -----------------------------------------------------------

/// Contains the commitment proof bytes serving to verify membership or
/// non-membership for an element or set of elements, in conjunction with
/// a known commitment root
#[derive(Clone, Debug, Drop, PartialEq, Serde)]
pub struct StateProof {
    pub proof: Array<u8>,
}

pub impl ArrayU8IntoProof of Into<Array<u8>, StateProof> {
    fn into(self: Array<u8>) -> StateProof {
        StateProof { proof: self }
    }
}

pub impl StateProofZero of Zero<StateProof> {
    fn zero() -> StateProof {
        StateProof { proof: ArrayTrait::new() }
    }

    fn is_zero(self: @StateProof) -> bool {
        self.proof.len() == 0
    }

    fn is_non_zero(self: @StateProof) -> bool {
        self.proof.len() > 0
    }
}
