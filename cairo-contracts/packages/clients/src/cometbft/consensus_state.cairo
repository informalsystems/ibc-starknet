use core::num::traits::Zero;
use starknet_ibc_clients::cometbft::CometErrors;
use starknet_ibc_core::client::{Status, Timestamp};
use starknet_ibc_core::commitment::StateRoot;

#[derive(Clone, Debug, Drop, PartialEq, Serde, starknet::Store)]
pub struct CometConsensusState {
    pub timestamp: Timestamp,
    pub root: StateRoot,
}

#[generate_trait]
pub impl CometConsensusStateImpl of CometConsensusStateTrait {
    fn is_non_zero(self: @CometConsensusState) -> bool {
        !(self.root.is_zero() && self.timestamp.is_zero())
    }

    fn timestamp(self: @CometConsensusState) -> u64 {
        *self.timestamp.timestamp
    }

    fn deserialize(consensus_state: Array<felt252>) -> CometConsensusState {
        let mut consensus_state_span = consensus_state.span();

        let maybe_consensus_state = Serde::<
            CometConsensusState,
        >::deserialize(ref consensus_state_span);

        assert(maybe_consensus_state.is_some(), CometErrors::INVALID_CONSENSUS_STATE);

        maybe_consensus_state.unwrap()
    }

    fn status(
        self: @CometConsensusState, host_timestamp: u64, trusting_period: u64, max_clock_drift: u64,
    ) -> Status {
        // Consider `clock_drift` on Starknet with large `block_time`.
        //
        // `block_timestamp` is when the sequencer started building the block, which may be in the
        // past. In that case, without the `clock_drift`, the untrusted consensus state would be
        // considered from future and fail.
        //
        // PS. the following check is always successful once a consensus state is validated and
        // accepted as trusted. Because the `block_timestamp` is always increasing.
        assert(
            host_timestamp + max_clock_drift >= self.timestamp(),
            CometErrors::HEADER_TIMESTAMP_FROM_FUTURE,
        );

        let elapsed_time = host_timestamp - self.timestamp();

        if elapsed_time < trusting_period {
            Status::Active
        } else {
            Status::Expired
        }
    }
}
