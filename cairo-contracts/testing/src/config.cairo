use starknet::ContractAddress;
use starknet_ibc_app_transfer::TRANSFER_PORT_ID;
use starknet_ibc_app_transfer::types::PrefixedDenomTrait;
use starknet_ibc_app_transfer::types::{
    MsgTransfer, PacketData, PrefixedDenom, Denom, Memo, TracePrefixTrait
};
use starknet_ibc_core_channel::Packet;
use starknet_ibc_core_client::{Height, Timestamp};
use starknet_ibc_core_host::{PortId, ChannelId, Sequence};
use starknet_ibc_testing::constants::{PUBKEY, NAME, AMOUNT, SUPPLY};

#[derive(Clone, Debug, Drop, Serde)]
pub struct TestConfig {
    pub native_denom: PrefixedDenom,
    pub hosted_denom: PrefixedDenom,
    pub chan_id_on_a: ByteArray,
    pub chan_id_on_b: ByteArray,
    pub amount: u256,
}

pub trait TestConfigTrait {
    fn default() -> TestConfig;
    fn set_native_denom(ref self: TestConfig, native_token_address: ContractAddress);
    fn prefix_native_denom(ref self: TestConfig);
    fn prefix_hosted_denom(ref self: TestConfig);
    fn dummy_msg_transder(
        self: @TestConfig, denom: PrefixedDenom, sender: ContractAddress, receiver: ContractAddress
    ) -> MsgTransfer;
    fn dummy_recv_packet(
        self: @TestConfig, denom: PrefixedDenom, sender: ContractAddress, receiver: ContractAddress
    ) -> Packet;
    fn dummy_packet_data(
        self: @TestConfig, denom: PrefixedDenom, sender: ContractAddress, receiver: ContractAddress
    ) -> PacketData;
}

impl TestConfigImpl of TestConfigTrait {
    fn default() -> TestConfig {
        let native_denom = PrefixedDenom {
            trace_path: array![], base: Denom::Native(PUBKEY().into())
        };

        let hosted_denom = PrefixedDenom { trace_path: array![], base: Denom::Hosted(NAME()) };

        TestConfig {
            native_denom,
            hosted_denom,
            chan_id_on_a: "channel-0",
            chan_id_on_b: "channel-1",
            amount: AMOUNT,
        }
    }

    fn set_native_denom(ref self: TestConfig, native_token_address: ContractAddress) {
        self
            .native_denom =
                PrefixedDenom {
                    trace_path: self.native_denom.trace_path,
                    base: Denom::Native(native_token_address.into())
                };
    }


    fn prefix_native_denom(ref self: TestConfig) {
        let trace_prefix = TracePrefixTrait::new(
            PortId { port_id: TRANSFER_PORT_ID() },
            ChannelId { channel_id: self.chan_id_on_a.clone() }
        );
        self.native_denom.add_prefix(trace_prefix);
    }

    fn prefix_hosted_denom(ref self: TestConfig) {
        let trace_prefix = TracePrefixTrait::new(
            PortId { port_id: TRANSFER_PORT_ID() },
            ChannelId { channel_id: self.chan_id_on_b.clone() }
        );
        self.hosted_denom.add_prefix(trace_prefix);
    }

    fn dummy_msg_transder(
        self: @TestConfig, denom: PrefixedDenom, sender: ContractAddress, receiver: ContractAddress
    ) -> MsgTransfer {
        MsgTransfer {
            port_id_on_a: PortId { port_id: TRANSFER_PORT_ID() },
            chan_id_on_a: ChannelId { channel_id: self.chan_id_on_a.clone() },
            packet_data: self.dummy_packet_data(denom, sender, receiver),
            timeout_height_on_b: Height { revision_number: 0, revision_height: 1000 },
            timeout_timestamp_on_b: Timestamp { timestamp: 1000 }
        }
    }

    fn dummy_recv_packet(
        self: @TestConfig, denom: PrefixedDenom, sender: ContractAddress, receiver: ContractAddress
    ) -> Packet {
        let mut serialized_data = array![];
        Serde::serialize(@self.dummy_packet_data(denom, sender, receiver), ref serialized_data);

        Packet {
            seq_on_a: Sequence { sequence: 0 },
            port_id_on_a: PortId { port_id: TRANSFER_PORT_ID() },
            chan_id_on_a: ChannelId { channel_id: self.chan_id_on_a.clone() },
            port_id_on_b: PortId { port_id: TRANSFER_PORT_ID() },
            chan_id_on_b: ChannelId { channel_id: self.chan_id_on_b.clone() },
            data: serialized_data,
            timeout_height_on_b: Height { revision_number: 0, revision_height: 1000 },
            timeout_timestamp_on_b: Timestamp { timestamp: 1000 }
        }
    }

    fn dummy_packet_data(
        self: @TestConfig, denom: PrefixedDenom, sender: ContractAddress, receiver: ContractAddress
    ) -> PacketData {
        PacketData { denom, amount: *self.amount, sender, receiver, memo: Memo { memo: "" }, }
    }
}
