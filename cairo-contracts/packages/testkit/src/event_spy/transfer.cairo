use openzeppelin_testing::events::{EventSpyExt, EventSpyExtImpl};
use snforge_std::EventSpy;
use starknet::ContractAddress;
use starknet_ibc_apps::transfer::TokenTransferComponent::{
    Event, SendEvent, RecvEvent, CreateTokenEvent
};
use starknet_ibc_apps::transfer::types::{Participant, PrefixedDenom};
use starknet_ibc_testkit::dummies::EMPTY_MEMO;

#[generate_trait]
pub impl TransferEventSpyExtImpl of TransferEventSpyExt {
    fn assert_send_event(
        ref self: EventSpy,
        contract_address: ContractAddress,
        sender: Participant,
        receiver: Participant,
        denom: PrefixedDenom,
        amount: u256
    ) {
        let expected = Event::SendEvent(
            SendEvent { sender, receiver, denom, amount, memo: EMPTY_MEMO() }
        );
        self.assert_emitted_single(contract_address, expected);
    }

    fn assert_recv_event(
        ref self: EventSpy,
        contract_address: ContractAddress,
        sender: Participant,
        receiver: Participant,
        denom: PrefixedDenom,
        amount: u256,
        success: bool
    ) {
        let expected = Event::RecvEvent(
            RecvEvent { sender, receiver, denom, amount, memo: EMPTY_MEMO(), success }
        );
        self.assert_emitted_single(contract_address, expected);
    }

    fn assert_create_token_event(
        ref self: EventSpy,
        contract_address: ContractAddress,
        name: ByteArray,
        symbol: ByteArray,
        address: ContractAddress,
        initial_supply: u256
    ) {
        let expected = Event::CreateTokenEvent(
            CreateTokenEvent { name, symbol, address, initial_supply }
        );
        self.assert_emitted_single(contract_address, expected);
    }
}
