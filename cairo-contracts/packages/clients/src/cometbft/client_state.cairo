use cometbft::utils::Fraction;
use core::num::traits::Zero;
use ics23::{ArrayFelt252Store, ProofSpec};
use starknet_ibc_clients::cometbft::CometErrors;
use starknet_ibc_core::client::{Duration, Height, HeightPartialOrd, Status, StatusTrait};

pub impl ArrayProofSpecStore = ics23::StorePackingViaSerde<Array<ProofSpec>>;
pub impl ArrayByteArrayStore = ics23::StorePackingViaSerde<Array<ByteArray>>;

#[derive(Clone, Debug, Drop, PartialEq, Serde, starknet::Store)]
pub struct CometClientState {
    pub latest_height: Height,
    pub trusting_period: Duration,
    pub unbonding_period: Duration,
    pub max_clock_drift: Duration,
    pub trust_level: Fraction,
    pub status: Status,
    pub chain_id: ByteArray,
    pub proof_spec: Array<ProofSpec>,
    pub upgrade_path: Array<ByteArray>,
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

    fn protobuf_bytes(self: @CometClientState) -> ByteArray {
        // FIXME: convert to cometbft type to encode to protobuf bytes
        ""
    }
}
