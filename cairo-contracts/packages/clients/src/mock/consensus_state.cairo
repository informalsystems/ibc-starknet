use core::num::traits::Zero;
use starknet_ibc_clients::mock::MockErrors;
use starknet_ibc_core::client::{
    Duration, DurationTrait, Status, Timestamp, TimestampImpl, TimestampIntoU128, TimestampZero,
};
use starknet_ibc_core::commitment::{StateRoot, StateRootZero};

#[derive(Clone, Debug, Drop, PartialEq, Serde, starknet::Store)]
pub struct MockConsensusState {
    pub timestamp: Timestamp,
    pub root: StateRoot,
}

pub impl MockConsensusStateZero of Zero<MockConsensusState> {
    fn zero() -> MockConsensusState {
        MockConsensusState { timestamp: TimestampZero::zero(), root: StateRootZero::zero() }
    }

    fn is_zero(self: @MockConsensusState) -> bool {
        self.timestamp.is_zero() && self.root.is_zero()
    }

    fn is_non_zero(self: @MockConsensusState) -> bool {
        !self.is_zero()
    }
}

#[generate_trait]
pub impl MockConsensusStateImpl of MockConsensusStateTrait {
    fn timestamp(self: @MockConsensusState) -> Timestamp {
        *self.timestamp
    }

    fn deserialize(consensus_state: Array<felt252>) -> MockConsensusState {
        let mut consensus_state_span = consensus_state.span();

        let maybe_consensus_state = Serde::<
            MockConsensusState,
        >::deserialize(ref consensus_state_span);

        assert(maybe_consensus_state.is_some(), MockErrors::INVALID_CONSENSUS_STATE);

        maybe_consensus_state.unwrap()
    }

    /// Returns the status of this consensus state given a host timestamp in nanoseconds, trusting
    /// period, and max clock drift between chains.
    fn status(
        self: @MockConsensusState, trusting_period: Duration, max_clock_drift: Duration,
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
            MockErrors::INVALID_HEADER_FROM_FUTURE,
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

    fn protobuf_bytes(self: @MockConsensusState) -> Array<u8> {
        // MockConsensusState does not implement protobuf serialization.
        array![]
    }
}
