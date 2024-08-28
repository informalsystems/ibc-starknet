use starknet_ibc_client_cometbft::CometErrors;
use starknet_ibc_core_client::{Height, Status};

#[derive(Clone, Debug, Drop, Hash, PartialEq, Serde, starknet::Store)]
pub struct CometClientState {
    pub latest_height: Height,
    pub trusting_period: u64,
    pub status: Status,
}

pub trait CometClientStateTrait {
    fn deserialize(client_state: Array<felt252>,) -> CometClientState;
    fn update(ref self: CometClientState, new_height: Height);
    fn freeze(ref self: CometClientState, freezing_height: Height);
}

pub impl CometClientStateImpl of CometClientStateTrait {
    fn deserialize(client_state: Array<felt252>,) -> CometClientState {
        let mut client_state_span = client_state.span();

        let maybe_client_state = Serde::<CometClientState>::deserialize(ref client_state_span);

        assert(maybe_client_state.is_some(), CometErrors::INVALID_CLIENT_STATE);

        maybe_client_state.unwrap()
    }

    fn update(ref self: CometClientState, new_height: Height) {
        if self.latest_height.clone() < new_height.clone() {
            self.latest_height = new_height;
        }
    }

    fn freeze(ref self: CometClientState, freezing_height: Height) {
        self.status = Status::Frozen(freezing_height);
    }
}
