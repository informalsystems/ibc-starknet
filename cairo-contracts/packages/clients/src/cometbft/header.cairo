use starknet_ibc_clients::cometbft::{CometErrors, CometConsensusState};
use starknet_ibc_core::client::{Height, U64IntoTimestamp};
use starknet_ibc_core::commitment::StateRoot;

#[derive(Clone, Debug, Drop, PartialEq, Serde, starknet::Store)]
pub struct CometHeader {
    pub trusted_height: Height,
    pub signed_header: SignedHeader,
}

#[generate_trait]
pub impl CometHeaderImpl of CometHeaderTrait {
    fn deserialize(header: Array<felt252>,) -> CometHeader {
        let mut header_span = header.span();

        let maybe_header = Serde::<CometHeader>::deserialize(ref header_span);

        assert(maybe_header.is_some(), CometErrors::INVALID_HEADER);

        maybe_header.unwrap()
    }
}

#[derive(Clone, Debug, Drop, PartialEq, Serde, starknet::Store)]
pub struct SignedHeader {
    pub height: Height,
    pub time: u64,
    pub root: StateRoot,
}

pub impl CometHeaderIntoConsensusState of Into<CometHeader, CometConsensusState> {
    fn into(self: CometHeader) -> CometConsensusState {
        CometConsensusState {
            timestamp: self.signed_header.time.into(), root: self.signed_header.root
        }
    }
}
