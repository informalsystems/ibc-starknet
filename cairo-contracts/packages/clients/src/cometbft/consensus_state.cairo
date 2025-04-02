use core::num::traits::Zero;
use starknet_ibc_clients::cometbft::CometErrors;
use starknet_ibc_core::client::{
    Duration, DurationTrait, Status, Timestamp, TimestampImpl, TimestampIntoU128,
};
use starknet_ibc_core::commitment::StateRoot;

#[derive(Clone, Debug, Drop, PartialEq, Serde, starknet::Store)]
pub struct CometConsensusState {
    pub timestamp: Timestamp,
    pub root: StateRoot,
    pub next_validators_hash: ByteArray,
}

#[generate_trait]
pub impl CometConsensusStateImpl of CometConsensusStateTrait {
    fn is_non_zero(self: @CometConsensusState) -> bool {
        !(self.root.is_zero() && self.timestamp.is_zero())
    }

    fn timestamp(self: @CometConsensusState) -> Timestamp {
        *self.timestamp
    }

    fn deserialize(consensus_state: Array<felt252>) -> CometConsensusState {
        let mut consensus_state_span = consensus_state.span();

        let maybe_consensus_state = Serde::<
            CometConsensusState,
        >::deserialize(ref consensus_state_span);

        assert(maybe_consensus_state.is_some(), CometErrors::INVALID_CONSENSUS_STATE);

        maybe_consensus_state.unwrap()
    }

    /// Returns the status of this consensus state given a host timestamp in nanoseconds, trusting
    /// period, and max clock drift between chains.
    fn status(
        self: @CometConsensusState, trusting_period: Duration, max_clock_drift: Duration,
    ) -> Status {
        // Consider `clock_drift` on Starknet with large `block_time`.
        //
        // `block_timestamp` is when the sequencer started building the block, which may be in the
        // past. In that case, without the `clock_drift`, the untrusted consensus state would be
        // considered from future and fail.
        //
        // PS. the following check is always successful once a consensus state is validated and
        // accepted as trusted. Because the `block_timestamp` is always increasing.

        let self_timestamp: u128 = self.timestamp().into();
        let host_timestamp: u128 = TimestampImpl::host().into();

        assert(
            host_timestamp + max_clock_drift.as_nanos() >= self_timestamp,
            CometErrors::INVALID_HEADER_FROM_FUTURE,
        );

        // The header is in future but within the `max_clock_drift` period.
        if host_timestamp < self_timestamp {
            return Status::Active;
        }

        if host_timestamp - self_timestamp < trusting_period.as_nanos() {
            Status::Active
        } else {
            Status::Expired
        }
    }
}
