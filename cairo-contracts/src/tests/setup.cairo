use core::option::OptionTrait;
use openzeppelin_testing::{deploy, pop_log};
use openzeppelin::token::erc20::{ERC20ABIDispatcher, ERC20ABIDispatcherTrait};
use openzeppelin::utils::serde::SerializedAppend;
use starknet::ContractAddress;
use starknet::testing;
use starknet_ibc::apps::transfer::component::ICS20TransferComponent::{
    Event as TransferEvent, SendEvent, RecvEvent
};
use starknet_ibc::apps::transfer::interface::{
    ISendTransferDispatcher, IRecvPacketDispatcher, ITokenAddressDispatcher,
};
use starknet_ibc::apps::transfer::interface::{
    ISendTransferDispatcherTrait, IRecvPacketDispatcherTrait, ITokenAddressDispatcherTrait
};
use starknet_ibc::apps::transfer::types::MsgTransfer;
use starknet_ibc::core::channel::types::Packet;
use starknet_ibc::presets::{Transfer, ERC20Mintable};
use starknet_ibc::tests::constants::{NAME, SYMBOL, SUPPLY, OWNER};

#[derive(Clone, Debug, Drop, PartialEq)]
pub struct ICS20TransferContract {
    pub contract_address: ContractAddress,
}

pub trait ICS20TransferContractTrait {
    fn setup() -> ICS20TransferContract;
    fn send_dispatcher(self: @ICS20TransferContract) -> ISendTransferDispatcher;
    fn recv_dispatcher(self: @ICS20TransferContract) -> IRecvPacketDispatcher;
    fn ibc_token_address(
        self: @ICS20TransferContract, token_key: felt252
    ) -> Option<ContractAddress>;
    fn send_execute(self: @ICS20TransferContract, msg: MsgTransfer);
    fn recv_execute(self: @ICS20TransferContract, packet: Packet);
    fn pop_event(self: @ICS20TransferContract) -> Option<TransferEvent>;
    fn assert_send_event(self: @ICS20TransferContract) -> SendEvent;
    fn assert_recv_event(self: @ICS20TransferContract) -> RecvEvent;
}

pub impl ICS20TransferContractImpl of ICS20TransferContractTrait {
    fn setup() -> ICS20TransferContract {
        let mut call_data = array![];
        call_data.append_serde(ERC20Mintable::TEST_CLASS_HASH);

        let contract_address = deploy(Transfer::TEST_CLASS_HASH, call_data);

        ICS20TransferContract { contract_address }
    }

    fn send_dispatcher(self: @ICS20TransferContract) -> ISendTransferDispatcher {
        ISendTransferDispatcher { contract_address: *self.contract_address }
    }

    fn recv_dispatcher(self: @ICS20TransferContract) -> IRecvPacketDispatcher {
        IRecvPacketDispatcher { contract_address: *self.contract_address }
    }

    fn ibc_token_address(
        self: @ICS20TransferContract, token_key: felt252
    ) -> Option<ContractAddress> {
        ITokenAddressDispatcher { contract_address: *self.contract_address }
            .ibc_token_address(token_key)
    }

    fn send_execute(self: @ICS20TransferContract, msg: MsgTransfer) {
        self.send_dispatcher().send_execute(msg);
    }

    fn recv_execute(self: @ICS20TransferContract, packet: Packet) {
        self.recv_dispatcher().recv_execute(packet);
    }

    fn pop_event(self: @ICS20TransferContract) -> Option<TransferEvent> {
        pop_log(*self.contract_address)
    }

    fn assert_send_event(self: @ICS20TransferContract) -> SendEvent {
        match self.pop_event().expect('no event') {
            TransferEvent::SendEvent(e) => e,
            _ => panic!("unexpected event"),
        }
    }

    fn assert_recv_event(self: @ICS20TransferContract) -> RecvEvent {
        match self.pop_event().expect('no event') {
            TransferEvent::RecvEvent(e) => e,
            _ => panic!("unexpected event"),
        }
    }
}

#[derive(Clone, Debug, Drop, PartialEq)]
pub struct ERC20Contract {
    pub contract_address: ContractAddress,
}

pub trait ERC20ContractTrait {
    fn setup() -> ERC20Contract;
    fn setup_with_addr(contract_address: ContractAddress) -> ERC20Contract;
    fn dispatcher(self: @ERC20Contract) -> ERC20ABIDispatcher;
    fn approve(
        self: @ERC20Contract, owner: ContractAddress, spender: ContractAddress, amount: u256
    );
    fn assert_balance(self: @ERC20Contract, account: ContractAddress, expected: u256);
    fn assert_total_supply(self: @ERC20Contract, expected: u256);
}

pub impl ERC20ContractImpl of ERC20ContractTrait {
    fn setup() -> ERC20Contract {
        let contract_address = deploy(ERC20Mintable::TEST_CLASS_HASH, dummy_erc20_call_data());

        ERC20Contract { contract_address }
    }

    fn setup_with_addr(contract_address: ContractAddress) -> ERC20Contract {
        ERC20Contract { contract_address }
    }

    fn dispatcher(self: @ERC20Contract) -> ERC20ABIDispatcher {
        ERC20ABIDispatcher { contract_address: *self.contract_address }
    }

    fn approve(
        self: @ERC20Contract, owner: ContractAddress, spender: ContractAddress, amount: u256
    ) {
        starknet::testing::set_contract_address(owner);
        self.dispatcher().approve(spender, amount);
    }

    fn assert_balance(self: @ERC20Contract, account: ContractAddress, expected: u256) {
        let balance = self.dispatcher().balance_of(account);
        assert_eq!(balance, expected);
    }

    fn assert_total_supply(self: @ERC20Contract, expected: u256) {
        let total_supply = self.dispatcher().total_supply();
        assert_eq!(total_supply, expected);
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
