#[starknet::contract]
pub(crate) mod Transfer {
    use openzeppelin::token::erc20::{ERC20Component, ERC20HooksEmptyImpl};
    use starknet::{ContractAddress, ClassHash};
    use starknet_ibc::apps::transfer::component::ICS20TransferComponent;

    component!(path: ERC20Component, storage: erc20, event: ERC20Event);
    component!(path: ICS20TransferComponent, storage: transfer, event: ICS20TransferEvent);

    #[abi(embed_v0)]
    impl ERC20MixinImpl = ERC20Component::ERC20MixinImpl<ContractState>;
    impl ERC20InternalImpl = ERC20Component::InternalImpl<ContractState>;

    impl TransferValidationImpl = ICS20TransferComponent::TransferValidationImpl<ContractState>;
    impl TransferExecutionImpl = ICS20TransferComponent::TransferExecutionImpl<ContractState>;
    impl TransferInternalImpl = ICS20TransferComponent::InternalImpl<ContractState>;

    #[storage]
    struct Storage {
        #[substorage(v0)]
        erc20: ERC20Component::Storage,
        #[substorage(v0)]
        transfer: ICS20TransferComponent::Storage,
    }

    #[event]
    #[derive(Drop, starknet::Event)]
    enum Event {
        #[flat]
        ERC20Event: ERC20Component::Event,
        #[flat]
        ICS20TransferEvent: ICS20TransferComponent::Event,
    }

    #[constructor]
    fn constructor(
        ref self: ContractState,
        name: ByteArray,
        symbol: ByteArray,
        fixed_supply: u256,
        recipient: ContractAddress,
        owner: ContractAddress
    ) {
        self.erc20.initializer(name, symbol);
    }
}

#[cfg(test)]
mod tests {
    use core::starknet::SyscallResultTrait;
    use starknet::ContractAddress;
    use starknet::contract_address_const;
    use starknet::syscalls::deploy_syscall;
    use starknet_ibc::Transfer;
    use starknet_ibc::apps::transfer::interface::{ITransferDispatcher, ITransferDispatcherTrait,};

    fn deploy() -> (ITransferDispatcher, ContractAddress) {
        let recipient: ContractAddress = contract_address_const::<'sender'>();

        let (contract_address, _) = deploy_syscall(
            Transfer::TEST_CLASS_HASH.try_into().unwrap(), recipient.into(), array![0].span(), false
        )
            .unwrap_syscall();

        (ITransferDispatcher { contract_address }, contract_address)
    }

    #[test]
    fn test_transfer() {
        deploy();
    }
}
