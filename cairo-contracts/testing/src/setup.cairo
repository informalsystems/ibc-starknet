use core::option::OptionTrait;
use openzeppelin_testing::events::{EventSpyExt, EventSpyExtImpl};
use openzeppelin_testing::{declare_class, deploy, declare_and_deploy};
use openzeppelin_token::erc20::{ERC20ABIDispatcher, ERC20ABIDispatcherTrait};
use openzeppelin_utils::serde::SerializedAppend;
use snforge_std::{EventSpy, spy_events, ContractClass, start_cheat_caller_address};
use starknet::{ClassHash, ContractAddress, testing};
use starknet_ibc_app_transfer::ICS20TransferComponent::{
    Event as TransferEvent, SendEvent, RecvEvent, CreateTokenEvent
};
use starknet_ibc_app_transfer::types::{MsgTransfer, Participant, PrefixedDenom, Memo};
use starknet_ibc_app_transfer::{
    ISendTransferDispatcher, IRecvPacketDispatcher, ITokenAddressDispatcher, ERC20Contract,
    ISendTransferDispatcherTrait, IRecvPacketDispatcherTrait, ITokenAddressDispatcherTrait
};
use starknet_ibc_core_channel::Packet;
use starknet_ibc_testing::constants::{NAME, SYMBOL, SUPPLY, OWNER};

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
        let expected = TransferEvent::SendEvent(
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
        let expected = TransferEvent::RecvEvent(
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
        let expected = TransferEvent::CreateTokenEvent(
            CreateTokenEvent { name, symbol, address, initial_supply }
        );
        self.spy.assert_emitted_single(self.contract_address, expected);
    }

    fn drop_all_events(ref self: TransferAppHandle) {
        self.spy.drop_all_events();
    }
}

#[generate_trait]
pub impl ERC20ContractImpl of ERC20ContractTrait {
    fn setup(contract_class: ContractClass) -> ERC20Contract {
        deploy(contract_class, dummy_erc20_call_data()).into()
    }

    fn dispatcher(self: @ERC20Contract) -> ERC20ABIDispatcher {
        ERC20ABIDispatcher { contract_address: *self.address }
    }

    fn approve(
        ref self: ERC20Contract, owner: ContractAddress, spender: ContractAddress, amount: u256
    ) {
        start_cheat_caller_address(self.address, owner);
        self.dispatcher().approve(spender, amount);
        start_cheat_caller_address(self.address, spender);
    }

    fn assert_balance(self: @ERC20Contract, account: ContractAddress, expected: u256) {
        let balance = self.dispatcher().balance_of(account);
        assert(balance == expected, 'balance mismatch');
    }

    fn assert_total_supply(self: @ERC20Contract, expected: u256) {
        let total_supply = self.dispatcher().total_supply();
        assert(total_supply == expected, 'total supply mismatch');
    }
}

pub(crate) fn dummy_erc20_call_data() -> Array<felt252> {
    let mut call_data: Array<felt252> = array![];
    Serde::serialize(@NAME(), ref call_data);
    Serde::serialize(@SYMBOL(), ref call_data);
    Serde::serialize(@SUPPLY, ref call_data);
    Serde::serialize(@OWNER(), ref call_data);
    Serde::serialize(@OWNER(), ref call_data);
    call_data
}
