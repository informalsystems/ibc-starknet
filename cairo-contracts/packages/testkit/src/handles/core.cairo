use openzeppelin_testing::declare_and_deploy;
use starknet::ContractAddress;
use starknet_ibc_core::channel::{
    IChannelHandlerDispatcher, IChannelHandlerDispatcherTrait, MsgChanOpenInit, MsgChanOpenTry,
    MsgRecvPacket, MsgAckPacket, MsgTimeoutPacket, IChannelQueryDispatcher,
    IChannelQueryDispatcherTrait, ChannelEnd, Packet,
};
use starknet_ibc_core::client::{
    IClientHandlerDispatcher, IClientHandlerDispatcherTrait, IRegisterClientDispatcher,
    IRegisterClientDispatcherTrait, MsgCreateClient, MsgUpdateClient, CreateResponse,
    UpdateResponse,
};
use starknet_ibc_core::commitment::Commitment;
use starknet_ibc_core::host::{ChannelId, PortId, Sequence};
use starknet_ibc_core::router::{IRouterDispatcher, IRouterDispatcherTrait};

#[derive(Copy, Drop, Serde)]
pub struct CoreContract {
    pub address: ContractAddress
}

#[generate_trait]
pub impl CoreHandleImpl of CoreHandle {
    fn deploy(contract_name: ByteArray) -> CoreContract {
        let mut call_data = array![];

        let address = declare_and_deploy(contract_name, call_data);

        CoreContract { address }
    }

    fn client_handler_dispatcher(self: @CoreContract) -> IClientHandlerDispatcher {
        IClientHandlerDispatcher { contract_address: *self.address }
    }

    fn channel_handler_dispatcher(self: @CoreContract) -> IChannelHandlerDispatcher {
        IChannelHandlerDispatcher { contract_address: *self.address }
    }

    fn channel_query_dispatcher(self: @CoreContract) -> IChannelQueryDispatcher {
        IChannelQueryDispatcher { contract_address: *self.address }
    }

    fn router_dispatcher(self: @CoreContract) -> IRouterDispatcher {
        IRouterDispatcher { contract_address: *self.address }
    }

    fn register_client_dispatcher(self: @CoreContract) -> IRegisterClientDispatcher {
        IRegisterClientDispatcher { contract_address: *self.address }
    }

    fn create_client(self: @CoreContract, msg: MsgCreateClient) -> CreateResponse {
        self.client_handler_dispatcher().create_client(msg)
    }

    fn update_client(self: @CoreContract, msg: MsgUpdateClient) -> UpdateResponse {
        self.client_handler_dispatcher().update_client(msg)
    }

    fn register_client(self: @CoreContract, client_type: felt252, client_address: ContractAddress) {
        self.register_client_dispatcher().register_client(client_type, client_address)
    }

    fn register_app(self: @CoreContract, port_id: ByteArray, app_address: ContractAddress) {
        self.router_dispatcher().bind_port_id(port_id, app_address)
    }

    fn chan_open_init(self: @CoreContract, msg: MsgChanOpenInit) {
        self.channel_handler_dispatcher().chan_open_init(msg)
    }

    fn chan_open_try(self: @CoreContract, msg: MsgChanOpenTry) {
        self.channel_handler_dispatcher().chan_open_try(msg)
    }

    fn send_packet(self: @CoreContract, packet: Packet) {
        self.channel_handler_dispatcher().send_packet(packet)
    }

    fn recv_packet(self: @CoreContract, msg: MsgRecvPacket) {
        self.channel_handler_dispatcher().recv_packet(msg)
    }

    fn ack_packet(self: @CoreContract, msg: MsgAckPacket) {
        self.channel_handler_dispatcher().ack_packet(msg)
    }

    fn timeout_packet(self: @CoreContract, msg: MsgTimeoutPacket) {
        self.channel_handler_dispatcher().timeout_packet(msg)
    }

    fn channel_end(self: @CoreContract, port_id: PortId, channel_id: ChannelId) -> ChannelEnd {
        self.channel_query_dispatcher().channel_end(port_id, channel_id)
    }

    fn packet_commitment(
        self: @CoreContract, port_id: PortId, channel_id: ChannelId, sequence: Sequence
    ) -> Commitment {
        self.channel_query_dispatcher().packet_commitment(port_id, channel_id, sequence)
    }

    fn packet_receipt(
        self: @CoreContract, port_id: PortId, channel_id: ChannelId, sequence: Sequence
    ) -> bool {
        self.channel_query_dispatcher().packet_receipt(port_id, channel_id, sequence)
    }

    fn packet_acknowledgement(
        self: @CoreContract, port_id: PortId, channel_id: ChannelId, sequence: Sequence
    ) -> Commitment {
        self.channel_query_dispatcher().packet_acknowledgement(port_id, channel_id, sequence)
    }

    fn next_sequence_send(self: @CoreContract, port_id: PortId, channel_id: ChannelId) -> Sequence {
        self.channel_query_dispatcher().next_sequence_send(port_id, channel_id)
    }
}
