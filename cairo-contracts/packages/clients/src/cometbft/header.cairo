use cometbft::light_client::Header as CometHeader;
use starknet_ibc_clients::cometbft::{CometConsensusState, CometErrors};
use starknet_ibc_core::client::{TimestampImpl, U64IntoTimestamp};
use starknet_ibc_core::commitment::StateRoot;

fn from_u8Array_to_u32Array(mut data: Span<u8>) -> Array<u32> {
    assert!(data.len() % 4 == 0);
    let mut result = array![];
    while let Option::Some(vals) = data.multi_pop_front() {
        let [val1, val2, val3, val4] = (*vals).unbox();
        let mut value = val1.into() * 0x1000000;
        value = value + val2.into() * 0x10000;
        value = value + val3.into() * 0x100;
        value = value + val4.into();
        result.append(value);
    }
    result
}

#[generate_trait]
pub impl CometHeaderImpl of CometHeaderTrait {
    fn deserialize(header: Array<felt252>) -> CometHeader {
        let mut header_span = header.span();

        let maybe_header = Serde::<CometHeader>::deserialize(ref header_span);

        assert(maybe_header.is_some(), CometErrors::INVALID_HEADER);

        maybe_header.unwrap()
    }
}

pub impl CometHeaderIntoConsensusState of Into<CometHeader, CometConsensusState> {
    fn into(self: CometHeader) -> CometConsensusState {
        let proto_ts = self.signed_header.header.time;
        let root_u8 = self.signed_header.header.app_hash;
        let next_validators_hash = self.signed_header.header.next_validators_hash;

        let timestamp = TimestampImpl::from_seconds_and_nanos(
            proto_ts.seconds.try_into().unwrap(), proto_ts.nanos.try_into().unwrap(),
        );

        let root_u32 = from_u8Array_to_u32Array(root_u8.span());

        let root = [
            *root_u32[0], *root_u32[1], *root_u32[2], *root_u32[3], *root_u32[4], *root_u32[5],
            *root_u32[6], *root_u32[7],
        ];

        CometConsensusState { timestamp, root: StateRoot { root }, next_validators_hash }
    }
}
