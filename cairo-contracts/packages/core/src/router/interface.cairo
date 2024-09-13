use starknet::ContractAddress;

#[starknet::interface]
pub trait IRouter<TContractState> {
    fn get_app_address(self: @TContractState, port_id: felt252) -> Option<ContractAddress>;

    fn bind_port_id(ref self: TContractState, port_id: felt252, app_address: ContractAddress);

    fn release_port_id(ref self: TContractState, port_id: felt252);
}
