use cometbft::light_client::Header as CometHeader;
use ibc_utils::bytes::ByteArrayIntoArrayU8;
use starknet_ibc_lib::protobuf::{IProtobufDispatcherTrait, IProtobufLibraryDispatcher};

#[derive(Drop)]
pub struct Misbehaviour {
    pub header_1: CometHeader,
    pub header_2: CometHeader,
}

#[generate_trait]
pub impl MisbehaviourImpl of MisbehaviourTrait {
    fn deserialize(misbehaviour_header: Array<felt252>) -> Misbehaviour {
        let mut span = misbehaviour_header.span();

        let (header_1_bytes, header_2_bytes) = Serde::<
            (ByteArray, ByteArray),
        >::deserialize(ref span)
            .unwrap();

        // let header_1 = ProtoCodecImpl::decode::<
        //     CometHeader,
        // >(ByteArrayIntoArrayU8::into(header_1_bytes).span())
        //     .unwrap();
        // let header_2 = ProtoCodecImpl::decode::<
        //     CometHeader,
        // >(ByteArrayIntoArrayU8::into(header_2_bytes).span())
        //     .unwrap();

        let header_1 = IProtobufLibraryDispatcher {
            class_hash: 'protobuf-class-hash'.try_into().unwrap(),
        }
            .comet_header_decode(ByteArrayIntoArrayU8::into(header_1_bytes));

        let header_2 = IProtobufLibraryDispatcher {
            class_hash: 'protobuf-class-hash'.try_into().unwrap(),
        }
            .comet_header_decode(ByteArrayIntoArrayU8::into(header_2_bytes));

        Misbehaviour { header_1, header_2 }
    }


    fn validate_basic(self: @Misbehaviour) {
        assert(
            self
                .header_1
                .signed_header
                .header
                .chain_id == self
                .header_2
                .signed_header
                .header
                .chain_id,
            'Chain ID must match',
        );

        assert(
            self.header_1.signed_header.header.height >= self.header_2.signed_header.header.height,
            'H1.height < H2.height',
        );
    }

    fn verify(self: @Misbehaviour) -> bool {
        if self.header_1.signed_header.header.height == self.header_2.signed_header.header.height {
            self
                .header_1
                .signed_header
                .commit
                .block_id != self
                .header_2
                .signed_header
                .commit
                .block_id
        } else {
            self.header_1.signed_header.header.time <= self.header_2.signed_header.header.time
        }
    }
}

