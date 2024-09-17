use openzeppelin_testing::{declare_class, declare_and_deploy};
use openzeppelin_utils::serde::SerializedAppend;
use snforge_std::ContractClass;
use starknet::ContractAddress;
use starknet_ibc_core::channel::{
    IChannelHandlerDispatcher, IChannelHandlerDispatcherTrait, MsgRecvPacket
};
use starknet_ibc_core::client::ClientEventEmitterComponent::{
    Event, CreateClientEvent, UpdateClientEvent
};
use starknet_ibc_core::client::{
    IClientHandlerDispatcher, IClientHandlerDispatcherTrait, IRegisterClientDispatcher,
    IRegisterClientDispatcherTrait, MsgCreateClient, MsgUpdateClient, CreateResponse,
    UpdateResponse, Height
};

#[derive(Drop, Serde)]
pub struct CoreContract {
    pub address: ContractAddress
}

#[generate_trait]
pub impl CoreHandleImpl of CoreHandle {
    fn deploy() -> CoreContract {
        let mut call_data = array![];

        let address = declare_and_deploy("IBCCore", call_data);

        CoreContract { address }
    }

    fn client_dispatcher(self: @CoreContract) -> IClientHandlerDispatcher {
        IClientHandlerDispatcher { contract_address: *self.address }
    }

    fn channel_dispatcher(self: @CoreContract) -> IChannelHandlerDispatcher {
        IChannelHandlerDispatcher { contract_address: *self.address }
    }

    fn register_dispatcher(self: @CoreContract) -> IRegisterClientDispatcher {
        IRegisterClientDispatcher { contract_address: *self.address }
    }

    fn create_client(self: @CoreContract, msg: MsgCreateClient) -> CreateResponse {
        self.client_dispatcher().create_client(msg)
    }

    fn update_client(self: @CoreContract, msg: MsgUpdateClient) -> UpdateResponse {
        self.client_dispatcher().update_client(msg)
    }

    fn register_client(self: @CoreContract, client_type: felt252, client_address: ContractAddress) {
        self.register_dispatcher().register_client(client_type, client_address)
    }

    fn recv_packet(self: @CoreContract, msg: MsgRecvPacket) {
        self.channel_dispatcher().recv_packet(msg)
    }
}
