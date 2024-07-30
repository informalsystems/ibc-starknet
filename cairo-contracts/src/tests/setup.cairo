use core::option::OptionTrait;
use openzeppelin::tests::utils::{deploy, pop_log};
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
use starknet_ibc::presets::{Transfer, ERC20};
use starknet_ibc::tests::utils::dummy_erc20_call_data;

#[derive(Clone, Debug, Drop, PartialEq, Eq)]
pub struct ICS20TransferContract {
    pub contract_address: ContractAddress,
}

pub trait ICS20TransferContractTrait {
    fn setup() -> ICS20TransferContract;
    fn send_dispatcher(self: @ICS20TransferContract) -> ISendTransferDispatcher;
    fn recv_dispatcher(self: @ICS20TransferContract) -> IRecvPacketDispatcher;
    fn addr_dispatcher(self: @ICS20TransferContract) -> ITokenAddressDispatcher;
    fn pop_event(self: @ICS20TransferContract) -> Option<TransferEvent>;
    fn assert_send_event(self: @ICS20TransferContract) -> SendEvent;
    fn assert_recv_event(self: @ICS20TransferContract) -> RecvEvent;
}

pub impl ICS20TransferContractImpl of ICS20TransferContractTrait {
    fn setup() -> ICS20TransferContract {
        let mut call_data = array![];
        call_data.append_serde(ERC20::TEST_CLASS_HASH);

        let contract_address = deploy(Transfer::TEST_CLASS_HASH, call_data);

        ICS20TransferContract { contract_address }
    }

    fn send_dispatcher(self: @ICS20TransferContract) -> ISendTransferDispatcher {
        ISendTransferDispatcher { contract_address: *self.contract_address }
    }

    fn recv_dispatcher(self: @ICS20TransferContract) -> IRecvPacketDispatcher {
        IRecvPacketDispatcher { contract_address: *self.contract_address }
    }

    fn addr_dispatcher(self: @ICS20TransferContract) -> ITokenAddressDispatcher {
        ITokenAddressDispatcher { contract_address: *self.contract_address }
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

#[derive(Clone, Debug, Drop, PartialEq, Eq)]
pub struct ERC20Contract {
    pub contract_address: ContractAddress,
}

pub trait ERC20ContractTrait {
    fn setup() -> ERC20Contract;
    fn setup_with_addr(contract_address: ContractAddress) -> ERC20Contract;
    fn dispatcher(self: @ERC20Contract) -> ERC20ABIDispatcher;
}

pub impl ERC20ContractImpl of ERC20ContractTrait {
    fn setup() -> ERC20Contract {
        let contract_address = deploy(ERC20::TEST_CLASS_HASH, dummy_erc20_call_data());

        ERC20Contract { contract_address }
    }

    fn setup_with_addr(contract_address: ContractAddress) -> ERC20Contract {
        ERC20Contract { contract_address }
    }

    fn dispatcher(self: @ERC20Contract) -> ERC20ABIDispatcher {
        ERC20ABIDispatcher { contract_address: *self.contract_address }
    }
}
