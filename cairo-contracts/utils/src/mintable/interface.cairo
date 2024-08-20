use starknet::ContractAddress;

#[starknet::interface]
pub trait IERC20Mintable<TContractState> {
    fn permissioned_mint(ref self: TContractState, recipient: ContractAddress, amount: u256);
    fn permissioned_burn(ref self: TContractState, account: ContractAddress, amount: u256);
}
