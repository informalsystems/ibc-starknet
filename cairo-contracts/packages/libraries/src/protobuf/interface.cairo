use cometbft::light_client::{ClientState, Header};
use ics23::MerkleProof;
use starknet_ibc_core::channel::ChannelEnd;
use starknet_ibc_core::connection::ConnectionEnd;

#[starknet::interface]
pub trait IProtobuf<TContractState> {
    fn comet_header_decode(self: @TContractState, proto_bytes: Array<u8>) -> Header;

    fn comet_client_state_encode(self: @TContractState, client_state: ClientState) -> Array<u8>;

    fn merkle_proof_decode(self: @TContractState, proto_bytes: Array<u8>) -> MerkleProof;

    fn connection_end_encode(self: @TContractState, end: ConnectionEnd) -> Array<u8>;

    fn channel_end_encode(self: @TContractState, end: ChannelEnd) -> Array<u8>;
}
