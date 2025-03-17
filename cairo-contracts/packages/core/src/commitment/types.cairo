use core::num::traits::Zero;
use core::sha256::{compute_sha256_byte_array, compute_sha256_u32_array};
use ics23::{IntoArrayU32, array_u32_into_array_u8};
use starknet::SyscallResult;
use starknet::storage_access::{StorageBaseAddress, Store};
use starknet_ibc_core::channel::Acknowledgement;
use starknet_ibc_core::client::{Height, Timestamp};
use starknet_ibc_core::commitment::U32CollectorImpl;

// -----------------------------------------------------------
// Commitment Value
// -----------------------------------------------------------

#[derive(Clone, Debug, Drop, PartialEq, Serde, starknet::Store)]
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
        let value = array_u32_into_array_u8(self.into(), 0, 0);
        StateValue { value }
    }
}

pub fn compute_packet_commitment(
    json_packet_data: @ByteArray, timeout_height: Height, timeout_timestamp: Timestamp,
) -> Commitment {
    let mut coll = U32CollectorImpl::init();
    // ibc-go uses nanosecs
    // https://github.com/cosmos/ibc-go/blob/98d7e7550a23ecf8d96ce042ab11ef857b184f2a/proto/ibc/core/channel/v1/channel.proto#L179-L180
    coll.extend(timeout_timestamp.timestamp);
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

// -----------------------------------------------------------
// State Root
// -----------------------------------------------------------

#[derive(Clone, Debug, Drop, PartialEq, Serde, starknet::Store)]
pub struct StateRoot {
    pub root: Array<u8>,
}

pub impl StateRootZero of Zero<StateRoot> {
    fn zero() -> StateRoot {
        StateRoot { root: ArrayTrait::new() }
    }

    fn is_zero(self: @StateRoot) -> bool {
        self.root.len() == 0
    }

    fn is_non_zero(self: @StateRoot) -> bool {
        self.root.len() > 0
    }
}

pub impl StoreU8Array of Store<Array<u8>> {
    fn read(address_domain: u32, base: StorageBaseAddress) -> SyscallResult<Array<u8>> {
        Self::read_at_offset(address_domain, base, 0)
    }

    fn write(address_domain: u32, base: StorageBaseAddress, value: Array<u8>) -> SyscallResult<()> {
        Self::write_at_offset(address_domain, base, 0, value)
    }

    fn read_at_offset(
        address_domain: u32, base: StorageBaseAddress, mut offset: u8,
    ) -> SyscallResult<Array<u8>> {
        let mut arr: Array<u8> = array![];

        let len: u8 = Store::<u8>::read_at_offset(address_domain, base, offset)
            .expect('Storage Span too large');
        offset += 1;

        let exit = 2 * len + offset;
        loop {
            if offset >= exit {
                break;
            }

            let value = Store::<u8>::read_at_offset(address_domain, base, offset).unwrap();
            arr.append(value);
            offset += Store::<u8>::size();
        };

        Result::Ok(arr)
    }

    fn write_at_offset(
        address_domain: u32, base: StorageBaseAddress, mut offset: u8, mut value: Array<u8>,
    ) -> SyscallResult<()> {
        let len: u8 = value.len().try_into().expect('Storage - Span too large');
        Store::<u8>::write_at_offset(address_domain, base, offset, len).unwrap();
        offset += 1;

        while let Option::Some(element) = value.pop_front() {
            Store::<u8>::write_at_offset(address_domain, base, offset, element).unwrap();
            offset += Store::<u8>::size();
        };

        Result::Ok(())
    }

    fn size() -> u8 {
        // StateRoot proof contains 32 u8 elements
        32 * Store::<u8>::size()
    }
}
