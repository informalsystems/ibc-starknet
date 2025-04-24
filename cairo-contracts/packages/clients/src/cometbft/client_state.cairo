use cometbft::utils::Fraction;
use core::num::traits::Zero;
use ics23::ProofSpec;
use starknet::SyscallResult;
use starknet::storage_access::{StorageBaseAddress, Store};
use starknet_ibc_clients::cometbft::CometErrors;
use starknet_ibc_core::client::{Duration, Height, HeightPartialOrd, Status, StatusTrait};

#[derive(Clone, Debug, Drop, PartialEq, Serde, starknet::Store)]
pub struct CometClientState {
    pub latest_height: Height,
    pub trusting_period: Duration,
    pub unbonding_period: Duration,
    pub max_clock_drift: Duration,
    pub trust_level: Fraction,
    pub status: Status,
    pub chain_id: ByteArray,
    // the first element may be empty
    pub upgrade_path: [ByteArray; 2],
    pub proof_spec: Array<ProofSpec>,
}

pub impl StoreProofSpecArray of Store<Array<ProofSpec>> {
    fn read(address_domain: u32, base: StorageBaseAddress) -> SyscallResult<Array<ProofSpec>> {
        Self::read_at_offset(address_domain, base, 0)
    }

    fn write(
        address_domain: u32, base: StorageBaseAddress, value: Array<ProofSpec>,
    ) -> SyscallResult<()> {
        Self::write_at_offset(address_domain, base, 0, value)
    }

    fn read_at_offset(
        address_domain: u32, base: StorageBaseAddress, mut offset: u8,
    ) -> SyscallResult<Array<ProofSpec>> {
        let mut arr: Array<ProofSpec> = array![];

        let len: u8 = Store::<u8>::read_at_offset(address_domain, base, offset)
            .expect('Storage Span too large');
        offset += 1;

        let exit = Store::<ProofSpec>::size() * len + offset;
        while offset < exit {
            let value = Store::<ProofSpec>::read_at_offset(address_domain, base, offset).unwrap();
            arr.append(value);
            offset += Store::<ProofSpec>::size();
        }

        Result::Ok(arr)
    }

    fn write_at_offset(
        address_domain: u32, base: StorageBaseAddress, mut offset: u8, mut value: Array<ProofSpec>,
    ) -> SyscallResult<()> {
        let len: u8 = value.len().try_into().expect('Storage - Span too large');
        Store::<u8>::write_at_offset(address_domain, base, offset, len).unwrap();
        offset += 1;

        while let Option::Some(element) = value.pop_front() {
            Store::<ProofSpec>::write_at_offset(address_domain, base, offset, element).unwrap();
            offset += Store::<ProofSpec>::size();
        }

        Result::Ok(())
    }

    // FIXME: Use correct size
    fn size() -> u8 {
        10 * Store::<ProofSpec>::size()
    }
}

#[generate_trait]
pub impl CometClientStateImpl of CometClientStateTrait {
    fn is_non_zero(self: @CometClientState) -> bool {
        !(self.latest_height.is_zero()
            && self.trusting_period.is_zero()
            && self.status.is_expired())
    }

    fn deserialize(client_state: Array<felt252>) -> CometClientState {
        let mut client_state_span = client_state.span();

        let maybe_client_state = Serde::<CometClientState>::deserialize(ref client_state_span);

        assert(maybe_client_state.is_some(), CometErrors::INVALID_CLIENT_STATE);

        maybe_client_state.unwrap()
    }

    fn update(ref self: CometClientState, new_height: Height) {
        if @self.latest_height < @new_height {
            self.latest_height = new_height;
        }
    }

    fn freeze(ref self: CometClientState, freezing_height: Height) {
        self.status = Status::Frozen(freezing_height);
    }

    fn substitute_client_matches(
        self: @CometClientState, other_client_state: CometClientState,
    ) -> bool {
        let mut substitute_client_state = other_client_state;

        substitute_client_state.latest_height = self.latest_height.clone();
        substitute_client_state.trusting_period = self.trusting_period.clone();
        substitute_client_state.status = self.status.clone();
        substitute_client_state.chain_id = self.chain_id.clone();

        @substitute_client_state == self
    }
}
