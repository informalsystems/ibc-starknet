use starknet::ContractAddress;
use starknet_ibc_core::channel::{
    Acknowledgement, AppVersion, ChannelOrdering, IAppCallbackDispatcher,
    IAppCallbackDispatcherTrait, Packet,
};
use starknet_ibc_core::host::{ChannelId, ConnectionId, PortId};

#[derive(Clone, Debug, Drop, Serde)]
pub struct AppContract {
    pub address: ContractAddress,
}

impl ContractAddressIntoAppAddr of Into<ContractAddress, AppContract> {
    fn into(self: ContractAddress) -> AppContract {
        AppContract { address: self }
    }
}

impl AppContractIntoFelt252 of Into<AppContract, felt252> {
    fn into(self: AppContract) -> felt252 {
        self.address.into()
    }
}

#[generate_trait]
pub impl AppContractImpl of AppContractTrait {
    fn on_chan_open_init(
        self: @AppContract,
        port_id_on_a: PortId,
        chan_id_on_a: ChannelId,
        conn_id_on_a: ConnectionId,
        port_id_on_b: PortId,
        version_proposal: AppVersion,
        ordering: ChannelOrdering,
    ) -> AppVersion {
        IAppCallbackDispatcher { contract_address: *self.address }
            .on_chan_open_init(
                port_id_on_a, chan_id_on_a, conn_id_on_a, port_id_on_b, version_proposal, ordering,
            )
    }

    fn on_chan_open_try(
        self: @AppContract,
        port_id_on_b: PortId,
        chan_id_on_b: ChannelId,
        conn_id_on_b: ConnectionId,
        port_id_on_a: PortId,
        version_on_a: AppVersion,
        ordering: ChannelOrdering,
    ) -> AppVersion {
        IAppCallbackDispatcher { contract_address: *self.address }
            .on_chan_open_try(
                port_id_on_b, chan_id_on_b, conn_id_on_b, port_id_on_a, version_on_a, ordering,
            )
    }

    fn on_chan_open_ack(
        self: @AppContract, port_id_on_a: PortId, chan_id_on_a: ChannelId, version_on_b: AppVersion,
    ) {
        IAppCallbackDispatcher { contract_address: *self.address }
            .on_chan_open_ack(port_id_on_a, chan_id_on_a, version_on_b)
    }

    fn on_chan_open_confirm(self: @AppContract, port_id_on_b: PortId, chan_id_on_b: ChannelId) {
        IAppCallbackDispatcher { contract_address: *self.address }
            .on_chan_open_confirm(port_id_on_b, chan_id_on_b)
    }

    fn on_recv_packet(self: @AppContract, packet: Packet) -> Acknowledgement {
        IAppCallbackDispatcher { contract_address: *self.address }.on_recv_packet(packet)
    }

    fn on_ack_packet(self: @AppContract, packet: Packet, ack: Acknowledgement) {
        IAppCallbackDispatcher { contract_address: *self.address }.on_ack_packet(packet, ack)
    }

    fn on_timeout_packet(self: @AppContract, packet: Packet) {
        IAppCallbackDispatcher { contract_address: *self.address }.on_timeout_packet(packet)
    }

    fn json_packet_data(self: @AppContract, raw_packet_data: Array<felt252>) -> ByteArray {
        IAppCallbackDispatcher { contract_address: *self.address }.json_packet_data(raw_packet_data)
    }
}
