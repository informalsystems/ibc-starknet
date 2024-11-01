use core::num::traits::Zero;
use starknet_ibc_core::channel::{Acknowledgement, Packet, ChannelErrors};
use starknet_ibc_core::client::{Height, HeightPartialOrd};
use starknet_ibc_core::commitment::CommitmentProof;
use starknet_ibc_utils::ValidateBasic;

#[derive(Clone, Debug, Drop, Serde)]
pub struct MsgRecvPacket {
    pub packet: Packet,
    pub proof_commitment_on_a: CommitmentProof,
    pub proof_height_on_a: Height,
}

impl MsgRecvPacketValidateBasicImpl of ValidateBasic<MsgRecvPacket> {
    fn validate_basic(self: @MsgRecvPacket) {
        self.packet.validate_basic();
        assert(self.proof_commitment_on_a.is_non_zero(), ChannelErrors::EMPTY_COMMITMENT_PROOF);
    }
}

#[derive(Clone, Debug, Drop, Serde)]
pub struct MsgAckPacket {
    pub packet: Packet,
    pub acknowledgement: Acknowledgement,
    pub proof_ack_on_a: CommitmentProof,
    pub proof_height_on_a: Height
}

impl MsgAcknowledgePacketValidateBasicImpl of ValidateBasic<MsgAckPacket> {
    fn validate_basic(self: @MsgAckPacket) {
        self.packet.validate_basic();
        assert(self.acknowledgement.is_non_zero(), ChannelErrors::EMPTY_ACK);
        assert(self.proof_ack_on_a.is_non_zero(), ChannelErrors::EMPTY_ACK_PROOF);
    }
}
