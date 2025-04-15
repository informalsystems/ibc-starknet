use openzeppelin_testing::declare_and_deploy;
use starknet::ContractAddress;
use starknet_ibc_core::channel::{
    ChannelEnd, IChannelHandlerDispatcher, IChannelHandlerDispatcherTrait, IChannelQueryDispatcher,
    IChannelQueryDispatcherTrait, MsgAckPacket, MsgChanOpenAck, MsgChanOpenConfirm, MsgChanOpenInit,
    MsgChanOpenTry, MsgRecvPacket, MsgTimeoutPacket, Packet,
};
use starknet_ibc_core::client::{
    CreateResponse, IClientHandlerDispatcher, IClientHandlerDispatcherTrait,
    IRegisterClientDispatcher, IRegisterClientDispatcherTrait, IRegisterRelayerDispatcher,
    IRegisterRelayerDispatcherTrait, MsgCreateClient, MsgRecoverClient, MsgUpdateClient,
    UpdateResponse,
};
use starknet_ibc_core::commitment::Commitment;
use starknet_ibc_core::connection::{
    ConnectionEnd, IConnectionHandlerDispatcher, IConnectionHandlerDispatcherTrait,
    IConnectionQueryDispatcher, IConnectionQueryDispatcherTrait, MsgConnOpenAck, MsgConnOpenConfirm,
    MsgConnOpenInit, MsgConnOpenTry,
};
use starknet_ibc_core::host::{ChannelId, ConnectionId, PortId, Sequence};
use starknet_ibc_core::router::{IRouterDispatcher, IRouterDispatcherTrait};

#[derive(Copy, Drop, Serde)]
pub struct CoreContract {
    pub address: ContractAddress,
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

    fn connection_handler_dispatcher(self: @CoreContract) -> IConnectionHandlerDispatcher {
        IConnectionHandlerDispatcher { contract_address: *self.address }
    }

    fn connection_query_dispatcher(self: @CoreContract) -> IConnectionQueryDispatcher {
        IConnectionQueryDispatcher { contract_address: *self.address }
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

    fn register_relayer_dispatcher(self: @CoreContract) -> IRegisterRelayerDispatcher {
        IRegisterRelayerDispatcher { contract_address: *self.address }
    }

    fn create_client(self: @CoreContract, msg: MsgCreateClient) -> CreateResponse {
        self.client_handler_dispatcher().create_client(msg)
    }

    fn update_client(self: @CoreContract, msg: MsgUpdateClient) -> UpdateResponse {
        self.client_handler_dispatcher().update_client(msg)
    }

    fn recover_client(self: @CoreContract, msg: MsgRecoverClient) {
        self.client_handler_dispatcher().recover_client(msg)
    }

    fn register_relayer(self: @CoreContract, relayer_address: ContractAddress) {
        self.register_relayer_dispatcher().register_relayer(relayer_address)
    }

    fn register_client(self: @CoreContract, client_type: felt252, client_address: ContractAddress) {
        self.register_client_dispatcher().register_client(client_type, client_address)
    }

    fn register_app(self: @CoreContract, port_id: PortId, app_address: ContractAddress) {
        self.router_dispatcher().bind_port_id(port_id, app_address)
    }

    fn conn_open_init(self: @CoreContract, msg: MsgConnOpenInit) -> ConnectionId {
        self.connection_handler_dispatcher().conn_open_init(msg)
    }

    fn conn_open_try(self: @CoreContract, msg: MsgConnOpenTry) -> ConnectionId {
        self.connection_handler_dispatcher().conn_open_try(msg)
    }

    fn conn_open_ack(self: @CoreContract, msg: MsgConnOpenAck) {
        self.connection_handler_dispatcher().conn_open_ack(msg)
    }

    fn conn_open_confirm(self: @CoreContract, msg: MsgConnOpenConfirm) {
        self.connection_handler_dispatcher().conn_open_confirm(msg)
    }

    fn connection_end(self: @CoreContract, connection_id: ConnectionId) -> ConnectionEnd {
        self.connection_query_dispatcher().connection_end(connection_id)
    }

    fn chan_open_init(self: @CoreContract, msg: MsgChanOpenInit) -> ChannelId {
        self.channel_handler_dispatcher().chan_open_init(msg)
    }

    fn chan_open_try(self: @CoreContract, msg: MsgChanOpenTry) -> ChannelId {
        self.channel_handler_dispatcher().chan_open_try(msg)
    }

    fn chan_open_ack(self: @CoreContract, msg: MsgChanOpenAck) {
        self.channel_handler_dispatcher().chan_open_ack(msg)
    }

    fn chan_open_confirm(self: @CoreContract, msg: MsgChanOpenConfirm) {
        self.channel_handler_dispatcher().chan_open_confirm(msg)
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
        self: @CoreContract, port_id: PortId, channel_id: ChannelId, sequence: Sequence,
    ) -> Option<Commitment> {
        self.channel_query_dispatcher().packet_commitment(port_id, channel_id, sequence)
    }

    fn packet_receipt(
        self: @CoreContract, port_id: PortId, channel_id: ChannelId, sequence: Sequence,
    ) -> bool {
        self.channel_query_dispatcher().packet_receipt(port_id, channel_id, sequence)
    }

    fn packet_acknowledgement(
        self: @CoreContract, port_id: PortId, channel_id: ChannelId, sequence: Sequence,
    ) -> Commitment {
        self.channel_query_dispatcher().packet_acknowledgement(port_id, channel_id, sequence)
    }

    fn next_sequence_send(self: @CoreContract, port_id: PortId, channel_id: ChannelId) -> Sequence {
        self.channel_query_dispatcher().next_sequence_send(port_id, channel_id)
    }
}
