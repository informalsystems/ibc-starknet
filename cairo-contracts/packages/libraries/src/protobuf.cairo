use cometbft::light_client::{ClientState, Header, Misbehaviour};
use ics23::MerkleProof;
use starknet_ibc_core::channel::ChannelEnd;
use starknet_ibc_core::connection::ConnectionEnd;

#[starknet::interface]
pub trait IProtobuf<TContractState> {
    fn comet_header_decode(self: @TContractState, proto_bytes: Array<u8>) -> Header;

    fn comet_misbehaviour_decode(self: @TContractState, proto_bytes: Array<u8>) -> Misbehaviour;

    fn comet_client_state_encode(self: @TContractState, value: ClientState) -> Array<u8>;

    fn merkle_proof_decode(self: @TContractState, proto_bytes: Array<u8>) -> MerkleProof;

    fn connection_end_encode(self: @TContractState, value: ConnectionEnd) -> Array<u8>;

    fn channel_end_encode(self: @TContractState, value: ChannelEnd) -> Array<u8>;
}

#[starknet::component]
pub mod ProtobufLibComponent {
    use protobuf::types::message::ProtoCodecImpl;
    use super::*;

    #[storage]
    pub struct Storage {}

    #[embeddable_as(ProtobufLib)]
    impl ProtobufLibImpl<
        TContractState, +HasComponent<TContractState>,
    > of super::IProtobuf<ComponentState<TContractState>> {
        fn comet_header_decode(
            self: @ComponentState<TContractState>, proto_bytes: Array<u8>,
        ) -> Header {
            ProtoCodecImpl::decode(proto_bytes.span()).unwrap()
        }

        fn comet_misbehaviour_decode(
            self: @ComponentState<TContractState>, proto_bytes: Array<u8>,
        ) -> Misbehaviour {
            ProtoCodecImpl::decode(proto_bytes.span()).unwrap()
        }

        fn comet_client_state_encode(
            self: @ComponentState<TContractState>, value: ClientState,
        ) -> Array<u8> {
            ProtoCodecImpl::encode(@value)
        }

        fn merkle_proof_decode(
            self: @ComponentState<TContractState>, proto_bytes: Array<u8>,
        ) -> MerkleProof {
            ProtoCodecImpl::decode(proto_bytes.span()).unwrap()
        }

        fn connection_end_encode(
            self: @ComponentState<TContractState>, value: ConnectionEnd,
        ) -> Array<u8> {
            ProtoCodecImpl::encode(@value)
        }

        fn channel_end_encode(
            self: @ComponentState<TContractState>, value: ChannelEnd,
        ) -> Array<u8> {
            ProtoCodecImpl::encode(@value)
        }
    }
}
