use core::num::traits::Zero;
use starknet_ibc_clients::cometbft::CometErrors;
use starknet_ibc_core::client::Status;

#[derive(Clone, Debug, Drop, Hash, PartialEq, Serde, starknet::Store)]
pub struct CometConsensusState {
    pub timestamp: u64,
    pub root: felt252,
}

#[generate_trait]
pub impl CometConsensusStateImpl of CometConsensusStateTrait {
    fn is_zero(self: @CometConsensusState) -> bool {
        self.root.is_zero() && self.timestamp.is_zero()
    }

    fn deserialize(consensus_state: Array<felt252>,) -> CometConsensusState {
        let mut consensus_state_span = consensus_state.span();

        let maybe_consensus_state = Serde::<
            CometConsensusState
        >::deserialize(ref consensus_state_span);

        assert(maybe_consensus_state.is_some(), CometErrors::INVALID_CLIENT_STATE);

        maybe_consensus_state.unwrap()
    }

    fn status(self: @CometConsensusState, host_timestamp: u64, trusting_period: u64) -> Status {
        assert(host_timestamp >= *self.timestamp, CometErrors::INVALID_HEADER_TIMESTAMP);

        let elapsed_time = host_timestamp - *self.timestamp;

        if elapsed_time < trusting_period {
            Status::Active
        } else {
            Status::Expired
        }
    }
}
