use cometbft::light_client::{ClientState, Header};
use ics23::MerkleProof;
use starknet_ibc_core::channel::ChannelEnd;
use starknet_ibc_core::connection::ConnectionEnd;

#[starknet::interface]
pub trait IProtobuf<TContractState> {
    fn comet_header_decode(self: @TContractState, proto_bytes: Array<u8>) -> Header;

    fn comet_client_state_encode(self: @TContractState, value: ClientState) -> Array<u8>;

    fn merkle_proof_decode(self: @TContractState, proto_bytes: Array<u8>) -> MerkleProof;

    fn connection_end_encode(self: @TContractState, value: ConnectionEnd) -> Array<u8>;

    fn channel_end_encode(self: @TContractState, value: ChannelEnd) -> Array<u8>;
}

#[starknet::contract]
pub mod IProtobufLib {
    use protobuf::types::message::ProtoCodecImpl;
    use super::*;

    #[storage]
    struct Storage {}

    #[abi(embed_v0)]
    impl IProtobufImpl of super::IProtobuf<ContractState> {
        fn comet_header_decode(self: @ContractState, proto_bytes: Array<u8>) -> Header {
            ProtoCodecImpl::decode(proto_bytes.span()).unwrap()
        }

        fn comet_client_state_encode(self: @ContractState, value: ClientState) -> Array<u8> {
            ProtoCodecImpl::encode(@value)
        }

        fn merkle_proof_decode(self: @ContractState, proto_bytes: Array<u8>) -> MerkleProof {
            ProtoCodecImpl::decode(proto_bytes.span()).unwrap()
        }

        fn connection_end_encode(self: @ContractState, value: ConnectionEnd) -> Array<u8> {
            ProtoCodecImpl::encode(@value)
        }

        fn channel_end_encode(self: @ContractState, value: ChannelEnd) -> Array<u8> {
            ProtoCodecImpl::encode(@value)
        }
    }
}
