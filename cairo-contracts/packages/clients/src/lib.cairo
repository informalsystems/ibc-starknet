pub mod cometbft {
    mod client_state;
    mod component;
    mod consensus_state;
    mod errors;
    mod header;

    pub use client_state::{CometClientState, CometClientStateImpl, CometClientStateTrait};
    pub use cometbft::light_client::Header as CometHeader;
    pub use component::CometClientComponent;
    pub use consensus_state::{
        CometConsensusState, CometConsensusStateImpl, CometConsensusStateStore,
        CometConsensusStateToStore, CometConsensusStateTrait, StoreToCometConsensusState,
    };
    pub use errors::CometErrors;
    pub use header::{CometHeaderImpl, CometHeaderIntoConsensusState, CometHeaderTrait};
}

#[cfg(test)]
mod tests {
    mod cometbft;
}
