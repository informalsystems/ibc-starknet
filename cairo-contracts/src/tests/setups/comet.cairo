use openzeppelin_testing::events::{EventSpyExt, EventSpyExtImpl};
use openzeppelin_testing::{declare_class, declare_and_deploy};
use openzeppelin_utils::serde::SerializedAppend;
use snforge_std::{EventSpy, spy_events, ContractClass};
use starknet::ContractAddress;
use starknet_ibc_core::client::ClientEventEmitterComponent::{
    Event, CreateClientEvent, UpdateClientEvent
};
use starknet_ibc_core::client::{
    IClientStateDispatcher, IClientStateDispatcherTrait, MsgCreateClient, MsgUpdateClient,
    CreateResponse, UpdateResponse, Height, Status
};
use starknet_ibc_core::host::{ClientId};

#[derive(Drop, Serde)]
pub struct CometClientHandle {
    pub contract_address: ContractAddress,
}

#[generate_trait]
pub impl CometClientHandleImpl of CometClientHandleTrait {
    fn setup() -> CometClientHandle {
        let mut call_data = array![];

        let contract_address = declare_and_deploy("CometClient", call_data);

        CometClientHandle { contract_address }
    }

    fn dispatcher(self: @CometClientHandle) -> IClientStateDispatcher {
        IClientStateDispatcher { contract_address: *self.contract_address }
    }

    fn client_type(self: @CometClientHandle) -> felt252 {
        self.dispatcher().client_type()
    }

    fn latest_height(self: @CometClientHandle, client_sequence: u64) -> Height {
        self.dispatcher().latest_height(client_sequence)
    }

    fn status(self: @CometClientHandle, client_sequence: u64) -> Status {
        self.dispatcher().status(client_sequence)
    }
}
