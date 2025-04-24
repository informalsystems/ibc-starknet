#[starknet::contract]
mod RawStore {
    use starknet::storage_access::StorageAddress;
    use starknet::syscalls::{storage_write_syscall, storage_read_syscall};

    #[storage]
    struct Storage {}

    #[external(v0)]
    fn set(ref self: ContractState, entries: Array<(StorageAddress, felt252)>) {
        for (key, value) in entries {
            storage_write_syscall(0, key, value).unwrap();
        }
    }

    #[external(v0)]
    fn get(ref self: ContractState, address: StorageAddress) -> felt252 {
        storage_read_syscall(0, address).unwrap()
    }
}
