use starknet_ibc::clients::tendermint::{TendermintErrors, TendermintConsensusState};
use starknet_ibc::core::client::Height;

#[derive(Clone, Debug, Drop, Hash, PartialEq, Serde, starknet::Store)]
pub struct TendermintHeader {
    pub trusted_height: Height,
    pub signed_header: SignedHeader,
}

pub trait TendermintHeaderTrait {
    fn deserialize(header: Array<felt252>,) -> TendermintHeader;
}

pub impl TendermintHeaderImpl of TendermintHeaderTrait {
    fn deserialize(header: Array<felt252>,) -> TendermintHeader {
        let mut header_span = header.span();

        let maybe_header = Serde::<TendermintHeader>::deserialize(ref header_span);

        assert(maybe_header.is_some(), TendermintErrors::INVALID_HEADER);

        maybe_header.unwrap()
    }
}

#[derive(Clone, Debug, Drop, Hash, PartialEq, Serde, starknet::Store)]
pub struct SignedHeader {
    pub time: u64,
    pub root: felt252,
}

pub impl TendermintHeaderIntoConsensusState of Into<TendermintHeader, TendermintConsensusState> {
    fn into(self: TendermintHeader) -> TendermintConsensusState {
        TendermintConsensusState {
            timestamp: self.signed_header.time, root: self.signed_header.root,
        }
    }
}
