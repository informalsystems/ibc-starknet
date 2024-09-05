use openzeppelin_testing::events::{EventSpyExt, EventSpyExtImpl};
use openzeppelin_testing::{declare_class, declare_and_deploy};
use openzeppelin_utils::serde::SerializedAppend;
use snforge_std::{EventSpy, spy_events, ContractClass};
use starknet::ContractAddress;
use starknet_ibc_core::client::ClientEventEmitterComponent::{
    Event, CreateClientEvent, UpdateClientEvent
};
use starknet_ibc_core::client::{
    IClientHandlerDispatcher, IClientHandlerDispatcherTrait, IRegisterClientDispatcher,
    IRegisterClientDispatcherTrait, MsgCreateClient, MsgUpdateClient, CreateResponse,
    UpdateResponse, Height
};
use starknet_ibc_core::host::{ClientId};

#[derive(Drop, Serde)]
pub struct IBCCoreHandle {
    pub contract_address: ContractAddress,
    pub spy: EventSpy,
}

#[generate_trait]
pub impl IBCCoreHandleImpl of IBCCoreHandleTrait {
    fn setup() -> IBCCoreHandle {
        let mut call_data = array![];

        let contract_address = declare_and_deploy("IBC", call_data);

        let spy = spy_events();

        IBCCoreHandle { contract_address, spy }
    }

    fn handler_dispatcher(self: @IBCCoreHandle) -> IClientHandlerDispatcher {
        IClientHandlerDispatcher { contract_address: *self.contract_address }
    }

    fn register_dispatcher(self: @IBCCoreHandle) -> IRegisterClientDispatcher {
        IRegisterClientDispatcher { contract_address: *self.contract_address }
    }

    fn create_client(self: @IBCCoreHandle, msg: MsgCreateClient) -> CreateResponse {
        self.handler_dispatcher().create_client(msg)
    }

    fn update_client(self: @IBCCoreHandle, msg: MsgUpdateClient) -> UpdateResponse {
        self.handler_dispatcher().update_client(msg)
    }

    fn register_client(
        self: @IBCCoreHandle, client_type: felt252, client_address: ContractAddress
    ) {
        self.register_dispatcher().register_client(client_type, client_address)
    }

    fn assert_create_event(
        ref self: IBCCoreHandle, client_id: ClientId, consensus_height: Height,
    ) {
        let expected = Event::CreateClientEvent(CreateClientEvent { client_id, consensus_height });
        self.spy.assert_emitted_single(self.contract_address, expected);
    }

    fn assert_update_event(
        ref self: IBCCoreHandle,
        client_id: ClientId,
        consensus_heights: Array<Height>,
        header: Array<felt252>,
    ) {
        let expected = Event::UpdateClientEvent(
            UpdateClientEvent { client_id, consensus_heights, header }
        );
        self.spy.assert_emitted_single(self.contract_address, expected);
    }

    fn drop_all_events(ref self: IBCCoreHandle) {
        self.spy.drop_all_events();
    }
}
