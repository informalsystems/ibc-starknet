use starknet::ContractAddress;
use starknet_ibc_core::channel::{
    ChannelEnd, IChannelHandlerDispatcher, IChannelHandlerDispatcherTrait, IChannelQueryDispatcher,
    IChannelQueryDispatcherTrait, Packet,
};
use starknet_ibc_core::host::{ChannelId, PortId, Sequence};

#[derive(Clone, Debug, Drop, Serde)]
pub struct ChannelContract {
    pub address: ContractAddress,
}

impl ContractAddressIntoChannelAddr of Into<ContractAddress, ChannelContract> {
    fn into(self: ContractAddress) -> ChannelContract {
        ChannelContract { address: self }
    }
}

impl ChannelContractIntoFelt252 of Into<ChannelContract, felt252> {
    fn into(self: ChannelContract) -> felt252 {
        self.address.into()
    }
}

#[generate_trait]
pub impl ChannelContractImpl of ChannelContractTrait {
    fn send_packet(self: @ChannelContract, packet: Packet) {
        IChannelHandlerDispatcher { contract_address: *self.address }.send_packet(packet)
    }

    fn channel_end(self: @ChannelContract, port_id: PortId, channel_id: ChannelId) -> ChannelEnd {
        IChannelQueryDispatcher { contract_address: *self.address }.channel_end(port_id, channel_id)
    }

    fn next_sequence_send(
        self: @ChannelContract, port_id: PortId, channel_id: ChannelId,
    ) -> Sequence {
        IChannelQueryDispatcher { contract_address: *self.address }
            .next_sequence_send(port_id, channel_id)
    }
}
