use starknet_ibc::clients::tendermint::TendermintErrors;
use starknet_ibc::core::client::{Height, Status};

#[derive(Clone, Debug, Drop, Hash, PartialEq, Serde, starknet::Store)]
pub struct TendermintConsensusState {
    pub timestamp: u64,
    pub root: felt252,
}

pub trait TendermintConsensusStateTrait {
    fn deserialize(consensus_state: Array<felt252>,) -> TendermintConsensusState;
    fn status(self: @TendermintConsensusState, host_timestamp: u64, trusting_period: u64) -> Status;
}

pub impl TendermintConsensusStateImpl of TendermintConsensusStateTrait {
    fn deserialize(consensus_state: Array<felt252>,) -> TendermintConsensusState {
        let mut consensus_state_span = consensus_state.span();

        let maybe_consensus_state = Serde::<
            TendermintConsensusState
        >::deserialize(ref consensus_state_span);

        assert(maybe_consensus_state.is_some(), TendermintErrors::INVALID_CLIENT_STATE);

        maybe_consensus_state.unwrap()
    }

    fn status(
        self: @TendermintConsensusState, host_timestamp: u64, trusting_period: u64
    ) -> Status {
        let elapsed_time = host_timestamp - *self.timestamp;

        if elapsed_time < trusting_period {
            Status::Active
        } else {
            Status::Expired
        }
    }
}
