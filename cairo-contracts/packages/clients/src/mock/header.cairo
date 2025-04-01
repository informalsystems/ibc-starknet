use starknet_ibc_clients::mock::{MockConsensusState, MockErrors};
use starknet_ibc_core::client::{Height, Timestamp, U64IntoTimestamp};
use starknet_ibc_core::commitment::StateRoot;

#[derive(Clone, Debug, Drop, PartialEq, Serde, starknet::Store)]
pub struct MockHeader {
    pub trusted_height: Height,
    pub signed_header: SignedHeader,
}

#[generate_trait]
pub impl MockHeaderImpl of MockHeaderTrait {
    fn deserialize(header: Array<felt252>) -> MockHeader {
        let mut header_span = header.span();

        let maybe_header = Serde::<MockHeader>::deserialize(ref header_span);

        assert(maybe_header.is_some(), MockErrors::INVALID_HEADER);

        maybe_header.unwrap()
    }
}

#[derive(Clone, Debug, Drop, PartialEq, Serde, starknet::Store)]
pub struct SignedHeader {
    pub height: Height,
    pub timestamp: Timestamp,
    pub root: StateRoot,
}

pub impl MockHeaderIntoConsensusState of Into<MockHeader, MockConsensusState> {
    fn into(self: MockHeader) -> MockConsensusState {
        MockConsensusState {
            timestamp: self.signed_header.timestamp, root: self.signed_header.root,
        }
    }
}
