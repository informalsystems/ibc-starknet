pub use starknet_ibc_core::channel::{Packet, ChannelErrors};
pub use starknet_ibc_core::client::{Height, HeightPartialOrd, Timestamp};
use starknet_ibc_utils::ValidateBasicTrait;

#[derive(Clone, Debug, Drop, Serde)]
pub struct MsgRecvPacket {
    pub packet: Packet,
    pub proof_commitment_on_a: Array<felt252>,
    pub proof_height_on_a: Height,
}

#[generate_trait]
pub impl MsgRecvPacketImpl of MsgRecvPacketTrait {
    fn verify_proof_height(self: @MsgRecvPacket, client_latest_height: @Height) {
        assert(self.proof_height_on_a >= client_latest_height, ChannelErrors::INVALID_PROOF_HEIGHT);
    }
}

impl MsgRecvPacketValidateBasicImpl of ValidateBasicTrait<MsgRecvPacket> {
    fn validate_basic(self: @MsgRecvPacket) {
        self.packet.validate_basic();
        assert(!self.proof_commitment_on_a.is_empty(), ChannelErrors::EMPTY_COMMITMENT_PROOF);
    }
}
