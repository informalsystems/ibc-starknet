use openzeppelin_testing::events::{EventSpyExt, EventSpyExtImpl};
use openzeppelin_testing::{declare_class, declare_and_deploy};
use openzeppelin_utils::serde::SerializedAppend;
use snforge_std::{EventSpy, spy_events, ContractClass};
use starknet::ContractAddress;
use starknet_ibc_apps::tests::OWNER;
use starknet_ibc_apps::transfer::TokenTransferComponent::{
    Event, SendEvent, RecvEvent, CreateTokenEvent
};
use starknet_ibc_apps::transfer::types::{MsgTransfer, Participant, PrefixedDenom, Memo};
use starknet_ibc_apps::transfer::{
    ISendTransferDispatcher, ITokenAddressDispatcher, ISendTransferDispatcherTrait,
    ITokenAddressDispatcherTrait,
};
use starknet_ibc_core::channel::Packet;
use starknet_ibc_core::channel::{IAppCallback, IAppCallbackDispatcher, IAppCallbackDispatcherTrait};

#[derive(Drop, Serde)]
pub struct AppContract {
    pub address: ContractAddress,
}

#[generate_trait]
pub impl AppHandleImpl of AppHandle {
    fn deploy_transfer(owner: ContractAddress, erc20_class: ContractClass) -> AppContract {
        let mut call_data = array![];

        call_data.append_serde(owner);
        call_data.append_serde(erc20_class.class_hash);

        let address = declare_and_deploy("TransferApp", call_data);

        AppContract { address }
    }

    fn send_dispatcher(self: @AppContract) -> ISendTransferDispatcher {
        ISendTransferDispatcher { contract_address: *self.address }
    }

    fn callback_dispatcher(self: @AppContract) -> IAppCallbackDispatcher {
        IAppCallbackDispatcher { contract_address: *self.address }
    }

    fn ibc_token_address(self: @AppContract, token_key: felt252) -> Option<ContractAddress> {
        ITokenAddressDispatcher { contract_address: *self.address }.ibc_token_address(token_key)
    }

    fn send_transfer(self: @AppContract, msg: MsgTransfer) {
        self.send_dispatcher().send_transfer(msg);
    }

    fn on_recv_packet(self: @AppContract, packet: Packet) {
        self.callback_dispatcher().on_recv_packet(packet);
    }
}
