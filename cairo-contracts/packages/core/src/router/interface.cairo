use starknet::ContractAddress;
use starknet_ibc_core::host::PortId;

#[starknet::interface]
pub trait IRouter<TContractState> {
    fn app_address(self: @TContractState, port_id: PortId) -> ContractAddress;

    fn bind_port_id(ref self: TContractState, port_id: PortId, app_address: ContractAddress);

    fn release_port_id(ref self: TContractState, port_id: PortId);
}
