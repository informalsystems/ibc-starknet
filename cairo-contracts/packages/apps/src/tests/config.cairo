use starknet::ContractAddress;
use starknet_ibc_apps::tests::{PUBKEY, NAME, AMOUNT, EMPTY_MEMO};
use starknet_ibc_apps::transfer::types::PrefixedDenomTrait;
use starknet_ibc_apps::transfer::types::{
    MsgTransfer, PacketData, PrefixedDenom, Denom, TracePrefixTrait, Participant
};
use starknet_ibc_core::channel::{Packet, MsgRecvPacket};
use starknet_ibc_core::client::Timestamp;
use starknet_ibc_core::host::{ChannelId, Sequence};
use starknet_ibc_core::tests::{PORT_ID, CHANNEL_ID, HEIGHT};

#[derive(Clone, Debug, Drop, Serde)]
pub struct TransferAppConfig {
    pub native_denom: PrefixedDenom,
    pub hosted_denom: PrefixedDenom,
    pub chan_id_on_a: ChannelId,
    pub chan_id_on_b: ChannelId,
    pub amount: u256,
}

#[generate_trait]
pub impl TransferAppConfigImpl of TransferAppConfigTrait {
    fn default() -> TransferAppConfig {
        let native_denom = PrefixedDenom {
            trace_path: array![], base: Denom::Native(PUBKEY().into())
        };

        let hosted_denom = PrefixedDenom { trace_path: array![], base: Denom::Hosted(NAME()) };

        TransferAppConfig {
            native_denom,
            hosted_denom,
            chan_id_on_a: CHANNEL_ID(0),
            chan_id_on_b: CHANNEL_ID(1),
            amount: AMOUNT,
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

    fn dummy_msg_transder(
        self: @TransferAppConfig, denom: PrefixedDenom, sender: Participant, receiver: Participant
    ) -> MsgTransfer {
        MsgTransfer {
            port_id_on_a: PORT_ID(),
            chan_id_on_a: self.chan_id_on_a.clone(),
            packet_data: self.dummy_packet_data(denom, sender, receiver),
            timeout_height_on_b: HEIGHT(1000),
            timeout_timestamp_on_b: Timestamp { timestamp: 1000 }
        }
    }

    fn dummy_msg_recv_packet(
        self: @TransferAppConfig, denom: PrefixedDenom, sender: Participant, receiver: Participant
    ) -> MsgRecvPacket {
        MsgRecvPacket {
            packet: self.dummy_recv_packet(denom, sender, receiver),
            proof_commitment_on_a: array![],
            proof_height_on_a: HEIGHT(10),
        }
    }

    fn dummy_recv_packet(
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
            timeout_height_on_b: HEIGHT(1000),
            timeout_timestamp_on_b: Timestamp { timestamp: 1000 }
        }
    }

    fn dummy_packet_data(
        self: @TransferAppConfig, denom: PrefixedDenom, sender: Participant, receiver: Participant
    ) -> PacketData {
        PacketData { denom, amount: *self.amount, sender, receiver, memo: EMPTY_MEMO() }
    }
}
