use core::num::traits::Zero;
use starknet_ibc_core::channel::{
    Acknowledgement, Packet, ChannelErrors, ChannelOrdering, ChannelVersion
};
use starknet_ibc_core::client::{Height, HeightPartialOrd};
use starknet_ibc_core::commitment::StateProof;
use starknet_ibc_core::host::Sequence;
use starknet_ibc_core::host::{ConnectionId, ChannelId, ChannelIdTrait, PortId, PortIdTrait};
use starknet_ibc_utils::ValidateBasic;

#[derive(Clone, Debug, Drop, Serde)]
pub struct MsgChanOpenInit {
    pub port_id_on_a: PortId,
    pub connection_hops_on_a: Array<ConnectionId>,
    pub version_on_a: ChannelVersion,
    pub port_id_on_b: PortId,
    pub ordering: ChannelOrdering,
}

pub impl MsgChanOpenInitValidateBasic of ValidateBasic<MsgChanOpenInit> {
    fn validate_basic(self: @MsgChanOpenInit) {
        assert(!self.port_id_on_a.is_zero(), ChannelErrors::MISSING_PORT_ID);
        assert(self.connection_hops_on_a.len() > 0, ChannelErrors::MISSING_CONNECTION_ID);
        assert(!self.port_id_on_b.is_zero(), ChannelErrors::MISSING_PORT_ID);
    }
}

#[derive(Clone, Debug, Drop, Serde)]
pub struct MsgChanOpenTry {
    pub port_id_on_b: PortId,
    pub connection_hops_on_b: Array<ConnectionId>,
    pub port_id_on_a: PortId,
    pub chan_id_on_a: ChannelId,
    pub version_on_a: ChannelVersion,
    pub proof_chan_end_on_a: StateProof,
    pub proof_height_on_a: Height,
    pub ordering: ChannelOrdering
}

pub impl MsgChanOpenTryValidateBasic of ValidateBasic<MsgChanOpenTry> {
    fn validate_basic(self: @MsgChanOpenTry) {
        assert(!self.port_id_on_b.is_zero(), ChannelErrors::MISSING_PORT_ID);
        assert(self.connection_hops_on_b.len() > 0, ChannelErrors::MISSING_CONNECTION_ID);
        assert(!self.port_id_on_a.is_zero(), ChannelErrors::MISSING_PORT_ID);
        assert(!self.chan_id_on_a.is_zero(), ChannelErrors::MISSING_CHANNEL_ID);
        assert(self.version_on_a.is_non_zero(), ChannelErrors::MISSING_CHANNEL_VERSION);
        assert(self.proof_chan_end_on_a.is_non_zero(), ChannelErrors::EMPTY_CHAN_END_PROOF);
        assert(self.proof_height_on_a.is_non_zero(), ChannelErrors::ZERO_PROOF_HEIGHT);
    }
}

#[derive(Clone, Debug, Drop, Serde)]
pub struct MsgChanOpenAck {
    pub port_id_on_a: PortId,
    pub chan_id_on_a: ChannelId,
    pub chan_id_on_b: ChannelId,
    pub version_on_b: ChannelVersion,
    pub proof_chan_end_on_b: StateProof,
    pub proof_height_on_b: Height
}

pub impl MsgChanOpenAckValidateBasic of ValidateBasic<MsgChanOpenAck> {
    fn validate_basic(self: @MsgChanOpenAck) {
        assert(!self.port_id_on_a.is_zero(), ChannelErrors::MISSING_PORT_ID);
        assert(!self.chan_id_on_a.is_zero(), ChannelErrors::MISSING_CHANNEL_ID);
        assert(!self.chan_id_on_b.is_zero(), ChannelErrors::MISSING_CHANNEL_ID);
        assert(self.version_on_b.is_non_zero(), ChannelErrors::MISSING_CHANNEL_VERSION);
        assert(self.proof_chan_end_on_b.is_non_zero(), ChannelErrors::EMPTY_CHAN_END_PROOF);
        assert(self.proof_height_on_b.is_non_zero(), ChannelErrors::ZERO_PROOF_HEIGHT);
    }
}

#[derive(Clone, Debug, Drop, Serde)]
pub struct MsgChanOpenConfirm {
    pub port_id_on_b: PortId,
    pub chan_id_on_b: ChannelId,
    pub proof_chan_end_on_a: StateProof,
    pub proof_height_on_a: Height
}

pub impl MsgChanOpenConfirmValidateBasic of ValidateBasic<MsgChanOpenConfirm> {
    fn validate_basic(self: @MsgChanOpenConfirm) {
        assert(!self.port_id_on_b.is_zero(), ChannelErrors::MISSING_PORT_ID);
        assert(!self.chan_id_on_b.is_zero(), ChannelErrors::MISSING_CHANNEL_ID);
        assert(self.proof_chan_end_on_a.is_non_zero(), ChannelErrors::EMPTY_CHAN_END_PROOF);
        assert(self.proof_height_on_a.is_non_zero(), ChannelErrors::ZERO_PROOF_HEIGHT);
    }
}

#[derive(Clone, Debug, Drop, Serde)]
pub struct MsgRecvPacket {
    pub packet: Packet,
    pub proof_commitment_on_a: StateProof,
    pub proof_height_on_a: Height,
}

impl MsgRecvPacketValidateBasic of ValidateBasic<MsgRecvPacket> {
    fn validate_basic(self: @MsgRecvPacket) {
        self.packet.validate_basic();
        assert(self.proof_commitment_on_a.is_non_zero(), ChannelErrors::EMPTY_COMMITMENT_PROOF);
    }
}

#[derive(Clone, Debug, Drop, Serde)]
pub struct MsgAckPacket {
    pub packet: Packet,
    pub acknowledgement: Acknowledgement,
    pub proof_ack_on_b: StateProof,
    pub proof_height_on_b: Height
}

impl MsgAcknowledgePacketValidateBasic of ValidateBasic<MsgAckPacket> {
    fn validate_basic(self: @MsgAckPacket) {
        self.packet.validate_basic();
        assert(self.acknowledgement.is_non_zero(), ChannelErrors::EMPTY_ACK);
        assert(self.proof_ack_on_b.is_non_zero(), ChannelErrors::EMPTY_ACK_PROOF);
    }
}

#[derive(Clone, Debug, Drop, Serde)]
pub struct MsgTimeoutPacket {
    pub packet: Packet,
    pub next_seq_recv_on_b: Sequence,
    pub proof_unreceived_on_b: StateProof,
    pub proof_height_on_b: Height,
}

impl MsgTimeoutPacketValidateBasic of ValidateBasic<MsgTimeoutPacket> {
    fn validate_basic(self: @MsgTimeoutPacket) {
        self.packet.validate_basic();
        assert(self.proof_unreceived_on_b.is_non_zero(), ChannelErrors::EMPTY_UNRECEIVED_PROOF);
    }
}
