#[starknet::component]
pub mod ICS20TransferComponent {
    use core::array::ArrayTrait;
    use core::num::traits::zero::Zero;
    use core::option::OptionTrait;
    use core::starknet::SyscallResultTrait;
    use core::traits::TryInto;
    use openzeppelin::token::erc20::ERC20Component::{InternalImpl, InternalTrait};
    use openzeppelin::token::erc20::ERC20Component;
    use openzeppelin::token::erc20::interface::ERC20ABI;
    use openzeppelin::token::erc20::{ERC20ABIDispatcher, ERC20ABIDispatcherTrait};
    use starknet::ClassHash;
    use starknet::ContractAddress;
    use starknet::get_caller_address;
    use starknet::get_contract_address;
    use starknet::syscalls::deploy_syscall;
    use starknet_ibc::apps::transfer::errors::ICS20Errors;
    use starknet_ibc::apps::transfer::interface::{
        ITransfer, ITransferValidationContext, ITransferExecutionContext
    };
    use starknet_ibc::apps::transfer::types::{MsgTransfer, PrefixedCoin, Memo, MAXIMUM_MEMO_LENGTH};
    use starknet_ibc::core::types::{PortId, ChannelId};

    #[storage]
    struct Storage {
        salt: felt252,
        governor: ContractAddress,
        registered_tokens: LegacyMap::<felt252, ContractAddress>,
        minted_tokens: LegacyMap::<felt252, ContractAddress>,
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
        denom: ByteArray,
        memo: Memo,
    }

    #[embeddable_as(Transfer)]
    impl TransferImpl<
        TContractState, +HasComponent<TContractState>, +Drop<TContractState>
    > of ITransfer<ComponentState<TContractState>> {
        fn send_transfer(ref self: ComponentState<TContractState>, msg: MsgTransfer) {}
        fn register_token(
            ref self: ComponentState<TContractState>,
            token_name: felt252,
            token_address: ContractAddress
        ) {
            let governor = self.governor.read();
            let maybe_governor = get_caller_address();
            assert(maybe_governor == governor, ICS20Errors::UNAUTHORIZED_REGISTAR);

            let registered_token_address: ContractAddress = self.registered_tokens.read(token_name);
            assert(registered_token_address.is_non_zero(), ICS20Errors::ALREADY_LISTED_TOKEN);
            assert(registered_token_address == token_address, ICS20Errors::ALREADY_LISTED_TOKEN);

            self.registered_tokens.write(token_name, token_address);
        }
    }

    #[embeddable_as(TransferValidationImpl)]
    impl TransferValidationContext<
        TContractState,
        +HasComponent<TContractState>,
        +ERC20ABI<TContractState>,
        +Drop<TContractState>,
    > of ITransferValidationContext<ComponentState<TContractState>> {
        fn escrow_validate(
            self: @ComponentState<TContractState>,
            from_address: ContractAddress,
            port_id: PortId,
            channel_id: ChannelId,
            coin: PrefixedCoin,
            memo: Memo,
        ) {
            assert(memo.memo.len() > MAXIMUM_MEMO_LENGTH, ICS20Errors::MAXIMUM_MEMO_LENGTH);

            let token_address = self.registered_tokens.read(coin.denom);
            let balance = ERC20ABIDispatcher { contract_address: token_address }
                .balance_of(from_address);
            assert(balance > coin.amount, ICS20Errors::INSUFFICIENT_BALANCE);
        }

        fn unescrow_validate(
            self: @ComponentState<TContractState>,
            to_address: ContractAddress,
            port_id: PortId,
            channel_id: ChannelId,
            coin: PrefixedCoin,
        ) {}

        fn mint_validate(
            self: @ComponentState<TContractState>, address: ContractAddress, coin: PrefixedCoin,
        ) {}

        fn burn_validate(
            self: @ComponentState<TContractState>,
            address: ContractAddress,
            coin: PrefixedCoin,
            memo: Memo,
        ) {}
    }

    #[embeddable_as(TransferExecutionImpl)]
    pub impl TransferExecutionContext<
        TContractState,
        +HasComponent<TContractState>,
        +ERC20ABI<TContractState>,
        +Drop<TContractState>,
    > of ITransferExecutionContext<ComponentState<TContractState>> {
        fn escrow_execute(
            ref self: ComponentState<TContractState>,
            from_address: ContractAddress,
            port_id: felt252,
            channel_id: felt252,
            coin: PrefixedCoin,
            memo: ByteArray,
        ) {
            let to_address = get_contract_address();
            let mut contract = self.get_contract_mut();
            contract.transfer_from(from_address, to_address, coin.amount);
        }

        fn unescrow_execute(
            ref self: ComponentState<TContractState>,
            to_address: ContractAddress,
            port_id: PortId,
            channel_id: ChannelId,
            coin: PrefixedCoin,
        ) {}

        fn mint_execute(
            ref self: ComponentState<TContractState>, address: ContractAddress, coin: PrefixedCoin,
        ) {}

        fn burn_execute(
            ref self: ComponentState<TContractState>,
            address: ContractAddress,
            coin: PrefixedCoin,
            memo: Memo,
        ) {}
    }

    #[generate_trait]
    pub impl TransferInternalImpl<
        TContractState,
        +HasComponent<TContractState>,
        +ERC20ABI<TContractState>,
        +Drop<TContractState>,
    > of TransferInternalTrait<TContractState> {
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

