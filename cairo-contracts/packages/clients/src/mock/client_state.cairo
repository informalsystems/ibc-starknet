use core::num::traits::Zero;
use ics23::ArrayFelt252Store;
use starknet_ibc_clients::mock::MockErrors;
use starknet_ibc_core::client::{Duration, Height, HeightPartialOrd, Status, StatusTrait};

pub impl ArrayByteArrayStore = ics23::StorePackingViaSerde<Array<ByteArray>>;

#[derive(Clone, Debug, Drop, PartialEq, Serde, starknet::Store)]
pub struct MockClientState {
    pub latest_height: Height,
    pub trusting_period: Duration,
    pub unbonding_period: Duration,
    pub max_clock_drift: Duration,
    pub status: Status,
    pub chain_id: ByteArray,
    pub upgrade_path: Array<ByteArray>,
}

#[generate_trait]
pub impl MockClientStateImpl of MockClientStateTrait {
    fn is_non_zero(self: @MockClientState) -> bool {
        !(self.latest_height.is_zero()
            && self.trusting_period.is_zero()
            && self.status.is_expired())
    }

    fn deserialize(client_state: Array<felt252>) -> MockClientState {
        let mut client_state_span = client_state.span();

        let maybe_client_state = Serde::<MockClientState>::deserialize(ref client_state_span);

        assert(maybe_client_state.is_some(), MockErrors::INVALID_CLIENT_STATE);

        maybe_client_state.unwrap()
    }

    fn update(ref self: MockClientState, new_height: Height) {
        if @self.latest_height < @new_height {
            self.latest_height = new_height;
        }
    }

    fn freeze(ref self: MockClientState, freezing_height: Height) {
        self.status = Status::Frozen(freezing_height);
    }

    fn substitute_client_matches(
        self: @MockClientState, other_client_state: MockClientState,
    ) -> bool {
        let mut substitute_client_state = other_client_state;

        substitute_client_state.latest_height = self.latest_height.clone();
        substitute_client_state.trusting_period = self.trusting_period.clone();
        substitute_client_state.status = self.status.clone();
        substitute_client_state.chain_id = self.chain_id.clone();

        @substitute_client_state == self
    }
}
