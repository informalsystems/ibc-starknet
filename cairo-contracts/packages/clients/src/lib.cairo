pub mod cometbft {
    mod client_state;
    mod component;
    mod consensus_state;
    mod errors;
    mod header;

    pub use client_state::{CometClientState, CometClientStateImpl, CometClientStateTrait};
    pub use component::CometClientComponent;
    pub use consensus_state::{
        CometConsensusState, CometConsensusStateImpl, CometConsensusStateTrait,
        CometConsensusStateZero,
    };
    pub use errors::CometErrors;
    pub use header::{
        CometHeader, CometHeaderImpl, CometHeaderIntoConsensusState, CometHeaderTrait, SignedHeader,
    };
}

pub mod mock {
    mod client_state;
    mod component;
    mod consensus_state;
    mod errors;
    mod header;

    pub use client_state::{MockClientState, MockClientStateImpl, MockClientStateTrait};
    pub use component::MockClientComponent;
    pub use consensus_state::{MockConsensusState, MockConsensusStateImpl, MockConsensusStateTrait};
    pub use errors::MockErrors;
    pub use header::{
        MockHeader, MockHeaderImpl, MockHeaderIntoConsensusState, MockHeaderTrait, SignedHeader,
    };
}

#[cfg(test)]
mod tests {
    mod cometbft;
}
