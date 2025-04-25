use core::num::traits::OverflowingAdd;
use starknet::SyscallResult;
use starknet::storage_access::{
    StorageAddress, StorageBaseAddress, Store, StorePacking, storage_address_from_base,
    storage_address_from_base_and_offset, storage_base_address_from_felt252,
};

pub impl StorePackingViaSerde<T, +Serde<T>, +Drop<T>> of StorePacking<T, Array<felt252>> {
    fn pack(value: T) -> Array<felt252> {
        let mut serialized: Array<felt252> = Default::default();
        Serde::<T>::serialize(@value, ref serialized);
        serialized
    }

    fn unpack(value: Array<felt252>) -> T {
        let mut serialized_span = value.span();
        Serde::<T>::deserialize(ref serialized_span).unwrap()
    }
}

/// Store for a `Array<felt252>` in storage.
///
/// Following the `ByteArray`'s `Store` implementation.
pub impl ArrayFelt252Store of Store<Array<felt252>> {
    #[inline]
    fn read(address_domain: u32, base: StorageBaseAddress) -> SyscallResult<Array<felt252>> {
        inner_read_byte_array(address_domain, storage_address_from_base(base))
    }

    #[inline]
    fn write(
        address_domain: u32, base: StorageBaseAddress, value: Array<felt252>,
    ) -> SyscallResult<()> {
        inner_write_byte_array(address_domain, storage_address_from_base(base), value)
    }

    #[inline]
    fn read_at_offset(
        address_domain: u32, base: StorageBaseAddress, offset: u8,
    ) -> SyscallResult<Array<felt252>> {
        inner_read_byte_array(address_domain, storage_address_from_base_and_offset(base, offset))
    }

    #[inline]
    fn write_at_offset(
        address_domain: u32, base: StorageBaseAddress, offset: u8, value: Array<felt252>,
    ) -> SyscallResult<()> {
        inner_write_byte_array(
            address_domain, storage_address_from_base_and_offset(base, offset), value,
        )
    }

    #[inline]
    fn size() -> u8 {
        1
    }
}

fn inner_byte_array_pointer(address: StorageAddress, chunk: felt252) -> StorageBaseAddress {
    let (r, _, _) = core::poseidon::hades_permutation(
        address.into(), chunk, 'Array<felt252>'_felt252,
    );
    storage_base_address_from_felt252(r)
}

fn inner_read_byte_array(
    address_domain: u32, address: StorageAddress,
) -> SyscallResult<Array<felt252>> {
    let mut len: usize =
        match starknet::syscalls::storage_read_syscall(address_domain, address)?.try_into() {
        Some(x) => x,
        None => { return SyscallResult::Err(array!['Invalid ByteArray length']); },
    };

    let mut chunk = 0;
    let mut chunk_base = inner_byte_array_pointer(address, chunk);
    let mut index_in_chunk = 0_u8;
    let mut result: Array<felt252> = Default::default();
    while len != 0 {
        let value =
            match starknet::syscalls::storage_read_syscall(
                address_domain, storage_address_from_base_and_offset(chunk_base, index_in_chunk),
            ) {
            Ok(value) => value,
            Err(err) => { return Err(err); },
        };

        result.append(value);
        len -= 1;

        let (mut next_index_in_chunk, is_overflowed) = index_in_chunk.overflowing_add(1);

        if is_overflowed {
            chunk += 1;
            chunk_base = inner_byte_array_pointer(address, chunk);
            next_index_in_chunk = 0;
        }

        index_in_chunk = next_index_in_chunk;
    }
    Ok(result)
}

fn inner_write_byte_array(
    address_domain: u32, address: StorageAddress, value: Array<felt252>,
) -> SyscallResult<()> {
    let len = value.len();
    starknet::syscalls::storage_write_syscall(address_domain, address, len.into())?;

    let mut full_words = value.span();

    let mut chunk = 0;
    let mut chunk_base = inner_byte_array_pointer(address, chunk);
    let mut index_in_chunk = 0_u8;
    while let Some(curr_value) = full_words.pop_front() {
        starknet::syscalls::storage_write_syscall(
            address_domain,
            storage_address_from_base_and_offset(chunk_base, index_in_chunk),
            (*curr_value).into(),
        )?;

        let (mut next_index_in_chunk, is_overflowed) = index_in_chunk.overflowing_add(1);

        if is_overflowed {
            chunk += 1;
            chunk_base = inner_byte_array_pointer(address, chunk);
            next_index_in_chunk = 0;
        }

        index_in_chunk = next_index_in_chunk;
    }
    Ok(())
}
