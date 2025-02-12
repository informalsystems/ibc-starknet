use openzeppelin_testing::events::{EventSpyExt, EventSpyExtImpl};
use openzeppelin_token::erc20::erc20::ERC20Component::{Event, Transfer};
use snforge_std::EventSpy;
use starknet::ContractAddress;

#[generate_trait]
pub impl ERC20EventSpyExtImpl of ERC20EventSpyExt {
    fn assert_transfer_event(
        ref self: EventSpy,
        contract_address: ContractAddress,
        from: ContractAddress,
        to: ContractAddress,
        value: u256
    ) {
        let expected = Event::Transfer(Transfer { from, to, value });
        self.assert_emitted_single(contract_address, expected);
    }
}
