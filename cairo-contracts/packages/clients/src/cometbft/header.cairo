use cometbft::light_client::Header as CometHeader;
use ibc_utils::bytes::{ByteArrayIntoArrayU8, SpanU8IntoByteArray};
use ibc_utils::numeric::u32_from_big_endian;
use starknet_ibc_clients::cometbft::{CometConsensusState, CometErrors};
use starknet_ibc_core::client::{TimestampImpl, U64IntoTimestamp};
use starknet_ibc_core::commitment::StateRoot;
use starknet_ibc_lib::protobuf::{IProtobufDispatcherTrait, IProtobufLibraryDispatcher};

#[generate_trait]
pub impl CometHeaderImpl of CometHeaderTrait {
    fn deserialize(header: Array<felt252>) -> CometHeader {
        let mut header_span = header.span();

        let maybe_byte_array = Serde::<ByteArray>::deserialize(ref header_span);

        assert(maybe_byte_array.is_some(), CometErrors::INVALID_HEADER);

        // let maybe_header = ProtoCodecImpl::decode::<
        //     CometHeader,
        // >(ByteArrayIntoArrayU8::into(maybe_byte_array.unwrap()).span());

        // assert(maybe_header.is_some(), CometErrors::INVALID_HEADER);

        // maybe_header.unwrap()

        IProtobufLibraryDispatcher { class_hash: 'protobuf-class-hash'.try_into().unwrap() }
            .comet_header_decode(ByteArrayIntoArrayU8::into(maybe_byte_array.unwrap()))
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

        assert(root_u8.len() == 32, 'Invalid CometBFT root hash');

        let mut root_span = root_u8.span();

        let r0 = u32_from_big_endian((*root_span.multi_pop_front().unwrap()).unbox());
        let r1 = u32_from_big_endian((*root_span.multi_pop_front().unwrap()).unbox());
        let r2 = u32_from_big_endian((*root_span.multi_pop_front().unwrap()).unbox());
        let r3 = u32_from_big_endian((*root_span.multi_pop_front().unwrap()).unbox());
        let r4 = u32_from_big_endian((*root_span.multi_pop_front().unwrap()).unbox());
        let r5 = u32_from_big_endian((*root_span.multi_pop_front().unwrap()).unbox());
        let r6 = u32_from_big_endian((*root_span.multi_pop_front().unwrap()).unbox());
        let r7 = u32_from_big_endian((*root_span.multi_pop_front().unwrap()).unbox());

        let root = [r0, r1, r2, r3, r4, r5, r6, r7];

        CometConsensusState { timestamp, root: StateRoot { root }, next_validators_hash }
    }
}
