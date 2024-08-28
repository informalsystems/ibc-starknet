use starknet_ibc_client_cometbft::{CometErrors, CometConsensusState};
use starknet_ibc_core_client::Height;

#[derive(Clone, Debug, Drop, Hash, PartialEq, Serde, starknet::Store)]
pub struct CometHeader {
    pub trusted_height: Height,
    pub signed_header: SignedHeader,
}

pub trait CometHeaderTrait {
    fn deserialize(header: Array<felt252>,) -> CometHeader;
}

pub impl CometHeaderImpl of CometHeaderTrait {
    fn deserialize(header: Array<felt252>,) -> CometHeader {
        let mut header_span = header.span();

        let maybe_header = Serde::<CometHeader>::deserialize(ref header_span);

        assert(maybe_header.is_some(), CometErrors::INVALID_HEADER);

        maybe_header.unwrap()
    }
}

#[derive(Clone, Debug, Drop, Hash, PartialEq, Serde, starknet::Store)]
pub struct SignedHeader {
    pub time: u64,
    pub root: felt252,
}

pub impl CometHeaderIntoConsensusState of Into<CometHeader, CometConsensusState> {
    fn into(self: CometHeader) -> CometConsensusState {
        CometConsensusState { timestamp: self.signed_header.time, root: self.signed_header.root, }
    }
}
