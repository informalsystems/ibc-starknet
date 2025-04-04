use cgp::prelude::*;

#[derive(Debug, HasField, HasFields)]
pub struct CometConsensusState {
    pub timestamp: u64,
    pub root: [u32; 8],
}
