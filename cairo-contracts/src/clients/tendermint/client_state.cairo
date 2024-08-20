use starknet_ibc::clients::tendermint::ICS07Errors;
use starknet_ibc::core::client::{Height, Status};

#[derive(Clone, Debug, Drop, Hash, PartialEq, Serde, starknet::Store)]
pub struct TendermintClientState {
    pub latest_height: Height,
    pub trusting_period: u64,
    pub status: Status,
}

pub trait TendermintClientStateTrait {
    fn deserialize(client_state: Array<felt252>,) -> TendermintClientState;
    fn update(ref self: TendermintClientState, new_height: Height);
    fn freeze(ref self: TendermintClientState, freezing_height: Height);
}

pub impl TendermintClientStateImpl of TendermintClientStateTrait {
    fn deserialize(client_state: Array<felt252>,) -> TendermintClientState {
        let mut client_state_span = client_state.span();

        let maybe_client_state = Serde::<TendermintClientState>::deserialize(ref client_state_span);

        assert(maybe_client_state.is_some(), ICS07Errors::INVALID_CLIENT_STATE);

        maybe_client_state.unwrap()
    }

    fn update(ref self: TendermintClientState, new_height: Height) {
        if self.latest_height.clone() < new_height.clone() {
            self.latest_height = new_height;
        }
    }

    fn freeze(ref self: TendermintClientState, freezing_height: Height) {
        self.status = Status::Frozen(freezing_height);
    }
}
