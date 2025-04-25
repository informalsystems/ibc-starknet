pub mod cometbft {
    mod client_state;
    mod component;
    mod consensus_state;
    mod errors;
    mod header;
    mod message;
    mod misbehaviour;

    pub use client_state::{CometClientState, CometClientStateImpl, CometClientStateTrait};
    pub use cometbft::light_client::Header as CometHeader;
    pub use component::CometClientComponent;
    pub use consensus_state::{
        CometConsensusState, CometConsensusStateImpl, CometConsensusStateStore,
        CometConsensusStateToStore, CometConsensusStateTrait, CometConsensusStateZero,
        StoreToCometConsensusState,
    };
    pub use errors::CometErrors;
    pub use header::{CometHeaderImpl, CometHeaderIntoConsensusState, CometHeaderTrait};
    pub use message::ClientMessage;
    pub use misbehaviour::{Misbehaviour, MisbehaviourImpl, MisbehaviourTrait};
}

pub mod mock {
    mod client_state;
    mod component;
    mod consensus_state;
    mod errors;
    mod header;

    pub use client_state::{MockClientState, MockClientStateImpl, MockClientStateTrait};
    pub use component::MockClientComponent;
    pub use consensus_state::{
        MockConsensusState, MockConsensusStateImpl, MockConsensusStateTrait, MockConsensusStateZero,
    };
    pub use errors::MockErrors;
    pub use header::{
        MockHeader, MockHeaderImpl, MockHeaderIntoConsensusState, MockHeaderTrait, SignedHeader,
    };
}

#[cfg(test)]
mod tests {
    mod cometbft;
}
