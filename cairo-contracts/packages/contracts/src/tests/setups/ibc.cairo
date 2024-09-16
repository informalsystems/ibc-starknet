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
pub struct IBCCoreHandle {
    pub contract_address: ContractAddress
}

#[generate_trait]
pub impl IBCCoreHandleImpl of IBCCoreHandleTrait {
    fn setup() -> IBCCoreHandle {
        let mut call_data = array![];

        let contract_address = declare_and_deploy("IBCCore", call_data);

        IBCCoreHandle { contract_address }
    }

    fn client_dispatcher(self: @IBCCoreHandle) -> IClientHandlerDispatcher {
        IClientHandlerDispatcher { contract_address: *self.contract_address }
    }

    fn channel_dispatcher(self: @IBCCoreHandle) -> IChannelHandlerDispatcher {
        IChannelHandlerDispatcher { contract_address: *self.contract_address }
    }

    fn register_dispatcher(self: @IBCCoreHandle) -> IRegisterClientDispatcher {
        IRegisterClientDispatcher { contract_address: *self.contract_address }
    }

    fn create_client(self: @IBCCoreHandle, msg: MsgCreateClient) -> CreateResponse {
        self.client_dispatcher().create_client(msg)
    }

    fn update_client(self: @IBCCoreHandle, msg: MsgUpdateClient) -> UpdateResponse {
        self.client_dispatcher().update_client(msg)
    }

    fn register_client(
        self: @IBCCoreHandle, client_type: felt252, client_address: ContractAddress
    ) {
        self.register_dispatcher().register_client(client_type, client_address)
    }

    fn recv_packet(self: @IBCCoreHandle, msg: MsgRecvPacket) {
        self.channel_dispatcher().recv_packet(msg)
    }
}
