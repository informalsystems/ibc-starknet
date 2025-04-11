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

#[cfg(test)]
mod tests {
    mod cometbft;
}
