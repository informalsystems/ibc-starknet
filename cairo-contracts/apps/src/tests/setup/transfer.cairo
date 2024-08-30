use openzeppelin_testing::events::{EventSpyExt, EventSpyExtImpl};
use openzeppelin_testing::{declare_class, declare_and_deploy};
use openzeppelin_utils::serde::SerializedAppend;
use snforge_std::{EventSpy, spy_events, ContractClass};
use starknet::ContractAddress;
use starknet_ibc_apps::transfer::components::ICS20TransferComponent::{
    Event, SendEvent, RecvEvent, CreateTokenEvent
};
use starknet_ibc_apps::transfer::types::{MsgTransfer, Participant, PrefixedDenom, Memo};
use starknet_ibc_apps::transfer::interfaces::{
    ISendTransferDispatcher, IRecvPacketDispatcher, ITokenAddressDispatcher,
    ISendTransferDispatcherTrait, IRecvPacketDispatcherTrait, ITokenAddressDispatcherTrait
};
use starknet_ibc_core::channel::Packet;

#[derive(Drop, Serde)]
pub struct TransferAppHandle {
    pub contract_address: ContractAddress,
    pub spy: EventSpy,
}

#[generate_trait]
pub impl TransferAppHandleImpl of TransferAppHandleTrait {
    fn setup(erc20_class: ContractClass) -> TransferAppHandle {
        let mut call_data = array![];

        call_data.append_serde(erc20_class.class_hash);

        let contract_address = declare_and_deploy("TransferApp", call_data);

        let spy = spy_events();

        TransferAppHandle { contract_address, spy }
    }

    fn send_dispatcher(self: @TransferAppHandle) -> ISendTransferDispatcher {
        ISendTransferDispatcher { contract_address: *self.contract_address }
    }

    fn recv_dispatcher(self: @TransferAppHandle) -> IRecvPacketDispatcher {
        IRecvPacketDispatcher { contract_address: *self.contract_address }
    }

    fn ibc_token_address(self: @TransferAppHandle, token_key: felt252) -> Option<ContractAddress> {
        ITokenAddressDispatcher { contract_address: *self.contract_address }
            .ibc_token_address(token_key)
    }

    fn send_execute(self: @TransferAppHandle, msg: MsgTransfer) {
        self.send_dispatcher().send_execute(msg);
    }

    fn recv_execute(self: @TransferAppHandle, packet: Packet) {
        self.recv_dispatcher().recv_execute(packet);
    }

    fn assert_send_event(
        ref self: TransferAppHandle,
        sender: Participant,
        receiver: Participant,
        denom: PrefixedDenom,
        amount: u256
    ) {
        let expected = Event::SendEvent(
            SendEvent { sender, receiver, denom, amount, memo: Memo { memo: "" } }
        );
        self.spy.assert_emitted_single(self.contract_address, expected);
    }

    fn assert_recv_event(
        ref self: TransferAppHandle,
        sender: Participant,
        receiver: Participant,
        denom: PrefixedDenom,
        amount: u256,
        success: bool
    ) {
        let expected = Event::RecvEvent(
            RecvEvent { sender, receiver, denom, amount, memo: Memo { memo: "" }, success, }
        );
        self.spy.assert_emitted_single(self.contract_address, expected);
    }

    fn assert_create_token_event(
        ref self: TransferAppHandle,
        name: ByteArray,
        symbol: ByteArray,
        address: ContractAddress,
        initial_supply: u256
    ) {
        let expected = Event::CreateTokenEvent(
            CreateTokenEvent { name, symbol, address, initial_supply }
        );
        self.spy.assert_emitted_single(self.contract_address, expected);
    }

    fn drop_all_events(ref self: TransferAppHandle) {
        self.spy.drop_all_events();
    }
}
