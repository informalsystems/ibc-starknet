#[starknet::component]
pub mod ICS20TransferComponent {
    use core::array::ArrayTrait;
    use core::option::OptionTrait;
    use core::starknet::SyscallResultTrait;
    use core::traits::TryInto;
    use openzeppelin::token::erc20::ERC20Component;
    use starknet::ClassHash;
    use starknet::ContractAddress;
    use starknet::get_caller_address;
    use starknet::get_contract_address;
    use starknet::syscalls::deploy_syscall;
    use starknet_ibc::apps::transfer::types::{PrefixedCoin, Memo};
    use starknet_ibc::core::types::{PortId, ChannelId};

    #[storage]
    struct Storage {
        salt: felt252,
        tokens: LegacyMap::<felt252, ContractAddress>,
    }

    #[event]
    #[derive(Drop, starknet::Event)]
    pub enum Event {
        TransferEvent: TransferEvent,
    }

    #[derive(Drop, Serde, starknet::Event)]
    pub struct TransferEvent {
        sender: ContractAddress,
        receiver: ContractAddress,
        amount: u64,
        denom: felt252,
        memo: Memo,
    }

    #[generate_trait]
    pub impl TransferValidationImpl<
        TContractState,
        +HasComponent<TContractState>,
        impl ERC20MixinImpl: ERC20Component::HasComponent<TContractState>
    > of TransferValidationTrait<TContractState> {
        fn escrow_validate(
            self: @ComponentState<TContractState>,
            from_account: ContractAddress,
            port_id: PortId,
            channel_id: ChannelId,
            coin: PrefixedCoin,
            memo: Memo,
        ) {}
        fn unescrow_validate(
            self: @ComponentState<TContractState>,
            to_account: ContractAddress,
            port_id: PortId,
            channel_id: ChannelId,
            coin: PrefixedCoin,
        ) {}
        fn mint_validate(
            self: @ComponentState<TContractState>, account: ContractAddress, coin: PrefixedCoin,
        ) {}
        fn burn_validate(
            self: @ComponentState<TContractState>,
            account: ContractAddress,
            coin: PrefixedCoin,
            memo: Memo,
        ) {}
    }

    #[generate_trait]
    pub impl TransferExecutionImpl<
        TContractState,
        +HasComponent<TContractState>,
        impl ERC20MixinImpl: ERC20Component::HasComponent<TContractState>
    > of TransferExecutionTrait<TContractState> {
        fn escrow_execute(
            ref self: ComponentState<TContractState>,
            from_account: ContractAddress,
            port_id: felt252,
            channel_id: felt252,
            coin: PrefixedCoin,
            memo: ByteArray,
        ) {}
        fn unescrow_execute(
            ref self: ComponentState<TContractState>,
            to_account: ContractAddress,
            port_id: PortId,
            channel_id: ChannelId,
            coin: PrefixedCoin,
        ) {}
        fn mint_execute(
            ref self: ComponentState<TContractState>, account: ContractAddress, coin: PrefixedCoin,
        ) {}
        fn burn_execute(
            ref self: ComponentState<TContractState>,
            account: ContractAddress,
            coin: PrefixedCoin,
            memo: Memo,
        ) {}
    }

    #[generate_trait]
    pub impl InternalImpl<
        TContractState,
        +HasComponent<TContractState>,
        impl ERC20MixinImpl: ERC20Component::HasComponent<TContractState>
    > of InternalTrait<TContractState> {
        fn create_token(ref self: ComponentState<TContractState>) -> ContractAddress {
            // unimplemented! > Dummy value to pass the type check
            0.try_into().unwrap()
        }
        fn get_escrow_address(
            self: @ComponentState<TContractState>, port_id: felt252, channel_id: felt252
        ) -> ContractAddress {
            // unimplemented! > Dummy value to pass the type check
            0.try_into().unwrap()
        }
    }
}

