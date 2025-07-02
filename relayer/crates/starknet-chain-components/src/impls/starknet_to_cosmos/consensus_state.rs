use hermes_core::chain_components::traits::{
    ConsensusStateFieldComponent, ConsensusStateFieldGetter, HasConsensusStateType, HasTimeType,
};
use hermes_prelude::*;
use tendermint::Time;

use crate::types::CometConsensusState;

pub struct ProvideCometConsensusState;

#[cgp_provider(ConsensusStateFieldComponent)]
impl<Chain, Counterparty> ConsensusStateFieldGetter<Chain, Counterparty>
    for ProvideCometConsensusState
where
    Chain: HasConsensusStateType<Counterparty, ConsensusState = CometConsensusState>,
    Counterparty: HasTimeType<Time = Time>,
{
    fn consensus_state_timestamp(consensus_state: &CometConsensusState) -> Counterparty::Time {
        let timestamp = consensus_state.timestamp;
        let (secs, nanos) = (timestamp / 1_000_000_000, timestamp % 1_000_000_000);
        Time::from_unix_timestamp(secs as i64, nanos as u32)
            .expect("failed to convert timestamp in CometConsensusState")
    }
}
