use starknet::ContractAddress;
use starknet_ibc_core::channel::{
    IAppCallbackDispatcher, IAppCallbackDispatcherTrait, Packet, Acknowledgement, ChannelOrdering,
    ChannelVersion
};
use starknet_ibc_core::host::{ConnectionId, ChannelId, PortId};

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
        version_on_a: ChannelVersion,
        port_id_on_b: PortId,
        ordering: ChannelOrdering
    ) {
        IAppCallbackDispatcher { contract_address: *self.address }
            .on_chan_open_init(
                port_id_on_a, chan_id_on_a, conn_id_on_a, version_on_a, port_id_on_b, ordering
            )
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
