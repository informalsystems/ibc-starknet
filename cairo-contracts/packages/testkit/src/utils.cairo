use starknet::syscalls::call_contract_syscall;
use starknet::{ContractAddress, SyscallResultTrait};

pub fn call_contract<T, +Drop<T>, +Serde<T>>(
    address: ContractAddress, entry_point_selector: felt252, data: @T,
) {
    let mut calldata = array![];
    data.serialize(ref calldata);
    call_contract_syscall(address, entry_point_selector, calldata.span()).unwrap_syscall();
}
