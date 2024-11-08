use starknet::ContractAddress;
use starknet_ibc_apps::transfer::types::{
    MsgTransfer, PacketData, PrefixedDenom, Denom, TracePrefixTrait, Participant, PrefixedDenomTrait
};
use starknet_ibc_core::channel::{
    Packet, MsgRecvPacket, MsgAckPacket, MsgTimeoutPacket, Acknowledgement
};
use starknet_ibc_core::client::{Height, Timestamp};
use starknet_ibc_core::host::{ChannelId, Sequence};
use starknet_ibc_testkit::dummies::{
    NATIVE_DENOM, HOSTED_DENOM, AMOUNT, EMPTY_MEMO, PORT_ID, CHANNEL_ID, HEIGHT, TIMEOUT_HEIGHT,
    TIMEOUT_TIMESTAMP, STATE_PROOF
};

#[derive(Clone, Debug, Drop, Serde)]
pub struct TransferAppConfig {
    pub native_denom: PrefixedDenom,
    pub hosted_denom: PrefixedDenom,
    pub chan_id_on_a: ChannelId,
    pub chan_id_on_b: ChannelId,
    pub amount: u256,
    pub timeout_height: Height,
    pub timeout_timestamp: Timestamp,
}

#[generate_trait]
pub impl TransferAppConfigImpl of TransferAppConfigTrait {
    fn default() -> TransferAppConfig {
        TransferAppConfig {
            native_denom: NATIVE_DENOM(),
            hosted_denom: HOSTED_DENOM(),
            chan_id_on_a: CHANNEL_ID(1),
            chan_id_on_b: CHANNEL_ID(0),
            amount: AMOUNT,
            timeout_height: TIMEOUT_HEIGHT(1000),
            timeout_timestamp: TIMEOUT_TIMESTAMP(1000),
        }
    }

    fn set_native_denom(ref self: TransferAppConfig, native_token_address: ContractAddress) {
        self
            .native_denom =
                PrefixedDenom {
                    trace_path: self.native_denom.trace_path,
                    base: Denom::Native(native_token_address.into())
                };
    }

    fn prefix_native_denom(self: @TransferAppConfig) -> PrefixedDenom {
        let trace_prefix = TracePrefixTrait::new(PORT_ID(), self.chan_id_on_a.clone());
        let mut native_denom = self.native_denom.clone();

        native_denom.add_prefix(trace_prefix);

        native_denom
    }

    fn prefix_hosted_denom(self: @TransferAppConfig) -> PrefixedDenom {
        let trace_prefix = TracePrefixTrait::new(PORT_ID(), self.chan_id_on_b.clone());

        let mut hosted_denom = self.hosted_denom.clone();

        hosted_denom.add_prefix(trace_prefix);

        hosted_denom
    }

    fn set_timeout_height(ref self: TransferAppConfig, timeout_height: Height) {
        self.timeout_height = timeout_height;
    }

    fn set_timeout_timestamp(ref self: TransferAppConfig, timeout_timestamp: Timestamp) {
        self.timeout_timestamp = timeout_timestamp;
    }

    fn dummy_msg_transfer(
        self: @TransferAppConfig, denom: PrefixedDenom, sender: Participant, receiver: Participant
    ) -> MsgTransfer {
        MsgTransfer {
            port_id_on_a: PORT_ID(),
            chan_id_on_a: self.chan_id_on_a.clone(),
            packet_data: self.dummy_packet_data(denom, sender, receiver),
            timeout_height_on_b: self.timeout_height.clone(),
            timeout_timestamp_on_b: self.timeout_timestamp.clone(),
        }
    }

    fn dummy_msg_recv_packet(
        self: @TransferAppConfig, denom: PrefixedDenom, sender: Participant, receiver: Participant
    ) -> MsgRecvPacket {
        MsgRecvPacket {
            packet: self.dummy_packet(denom, sender, receiver),
            proof_commitment_on_a: STATE_PROOF(),
            proof_height_on_a: HEIGHT(10),
        }
    }

    fn dummy_msg_ack_packet(
        self: @TransferAppConfig,
        denom: PrefixedDenom,
        sender: Participant,
        receiver: Participant,
        acknowledgement: Acknowledgement
    ) -> MsgAckPacket {
        MsgAckPacket {
            packet: self.dummy_packet(denom, sender, receiver),
            acknowledgement,
            proof_ack_on_b: STATE_PROOF(),
            proof_height_on_b: HEIGHT(10),
        }
    }

    fn dummy_msg_timeout_packet(
        self: @TransferAppConfig,
        denom: PrefixedDenom,
        sender: Participant,
        receiver: Participant,
        proof_height: Height,
    ) -> MsgTimeoutPacket {
        MsgTimeoutPacket {
            packet: self.dummy_packet(denom, sender, receiver),
            next_seq_recv_on_b: Sequence { sequence: 1 },
            proof_unreceived_on_b: STATE_PROOF(),
            proof_height_on_b: proof_height,
        }
    }

    fn dummy_packet(
        self: @TransferAppConfig, denom: PrefixedDenom, sender: Participant, receiver: Participant
    ) -> Packet {
        let mut serialized_data = array![];
        Serde::serialize(@self.dummy_packet_data(denom, sender, receiver), ref serialized_data);

        Packet {
            seq_on_a: Sequence { sequence: 0 },
            port_id_on_a: PORT_ID(),
            chan_id_on_a: self.chan_id_on_a.clone(),
            port_id_on_b: PORT_ID(),
            chan_id_on_b: self.chan_id_on_b.clone(),
            data: serialized_data,
            timeout_height_on_b: self.timeout_height.clone(),
            timeout_timestamp_on_b: self.timeout_timestamp.clone(),
        }
    }

    fn dummy_packet_data(
        self: @TransferAppConfig, denom: PrefixedDenom, sender: Participant, receiver: Participant
    ) -> PacketData {
        PacketData { denom, amount: *self.amount, sender, receiver, memo: EMPTY_MEMO() }
    }
}
