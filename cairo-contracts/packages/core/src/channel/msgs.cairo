pub use starknet_ibc_core::channel::{Packet, ChannelErrors};
pub use starknet_ibc_core::client::{Height, HeightPartialOrd, Timestamp};
use starknet_ibc_utils::ValidateBasic;

#[derive(Clone, Debug, Drop, Serde)]
pub struct MsgRecvPacket {
    pub packet: Packet,
    pub proof_commitment_on_a: Array<u8>,
    pub proof_height_on_a: Height,
}

impl MsgRecvPacketValidateBasicImpl of ValidateBasic<MsgRecvPacket> {
    fn validate_basic(self: @MsgRecvPacket) {
        self.packet.validate_basic();
        assert(!self.proof_commitment_on_a.is_empty(), ChannelErrors::EMPTY_COMMITMENT_PROOF);
    }
}
