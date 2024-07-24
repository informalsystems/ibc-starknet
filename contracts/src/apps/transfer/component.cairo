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
        ITransfer, ITransferrable, ITransferValidationContext, ITransferExecutionContext
    };
    use starknet_ibc::apps::transfer::types::{MsgTransfer, PrefixedCoin, Memo, MAXIMUM_MEMO_LENGTH};
    use starknet_ibc::core::types::{PortId, ChannelId};

    #[storage]
    struct Storage {
        salt: felt252,
        governor: ContractAddress,
        send_capability: bool,
        receive_capability: bool,
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
        TContractState,
        +HasComponent<TContractState>,
        +ERC20ABI<TContractState>,
        +Drop<TContractState>
    > of ITransfer<ComponentState<TContractState>> {
        fn send_transfer(ref self: ComponentState<TContractState>, msg: MsgTransfer) {
            self.can_send();

            let is_sender_chain_source = self.is_sender_chain_source(msg.packet_data.token.denom);

            let is_receiver_chain_source = self
                .is_receiver_chain_source(msg.packet_data.token.denom);

            assert(
                !is_sender_chain_source && !is_receiver_chain_source,
                ICS20Errors::INVALID_TOKEN_NAME
            );

            if is_sender_chain_source {
                self
                    .escrow_validate(
                        msg.packet_data.sender.clone(),
                        msg.port_id_on_a.clone(),
                        msg.chan_id_on_a.clone(),
                        msg.packet_data.token.clone(),
                        msg.packet_data.memo.clone(),
                    );

                self
                    .escrow_execute(
                        msg.packet_data.sender.clone(),
                        msg.port_id_on_a.clone(),
                        msg.chan_id_on_a.clone(),
                        msg.packet_data.token.clone(),
                        msg.packet_data.memo.clone(),
                    );
            }

            if is_receiver_chain_source {
                self
                    .burn_validate(
                        msg.packet_data.sender.clone(),
                        msg.packet_data.token.clone(),
                        msg.packet_data.memo.clone(),
                    );

                self
                    .burn_execute(
                        msg.packet_data.sender.clone(),
                        msg.packet_data.token.clone(),
                        msg.packet_data.memo.clone(),
                    );
            }
        }

        fn register_token(
            ref self: ComponentState<TContractState>,
            token_name: felt252,
            token_address: ContractAddress
        ) {
            let governor = self.governor.read();

            assert(governor == get_caller_address(), ICS20Errors::UNAUTHORIZED_REGISTAR);

            assert(token_name.is_non_zero(), ICS20Errors::ZERO_TOKEN_NAME);

            let registered_token_address: ContractAddress = self.registered_tokens.read(token_name);

            assert(registered_token_address.is_zero(), ICS20Errors::ALREADY_LISTED_TOKEN);

            assert(token_address.is_non_zero(), ICS20Errors::ZERO_TOKEN_ADDRESS);

            self.registered_tokens.write(token_name, token_address);
        }
    }

    #[embeddable_as(TransferrableImpl)]
    pub impl Transferrable<
        TContractState, +HasComponent<TContractState>, +Drop<TContractState>
    > of ITransferrable<ComponentState<TContractState>> {
        fn can_send(self: @ComponentState<TContractState>) {
            let send_capability = self.send_capability.read();
            assert(send_capability, ICS20Errors::NO_SEND_CAPABILITY);
        }
        fn can_receive(self: @ComponentState<TContractState>) {
            let receive_capability = self.receive_capability.read();
            assert(receive_capability, ICS20Errors::NO_RECEIVE_CAPABILITY);
        }
    }


    #[embeddable_as(TransferValidationImpl)]
    pub impl TransferValidationContext<
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
            port_id: PortId,
            channel_id: ChannelId,
            coin: PrefixedCoin,
            memo: Memo,
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
    pub(crate) impl TransferInternalImpl<
        TContractState,
        +HasComponent<TContractState>,
        +ERC20ABI<TContractState>,
        +Drop<TContractState>,
    > of TransferInternalTrait<TContractState> {
        fn initializer(ref self: ComponentState<TContractState>) {
            self.governor.write(get_caller_address());
            self.send_capability.write(true);
            self.receive_capability.write(true);
        }

        fn is_sender_chain_source(self: @ComponentState<TContractState>, denom: felt252) -> bool {
            self.registered_tokens.read(denom).is_zero()
        }

        fn is_receiver_chain_source(self: @ComponentState<TContractState>, denom: felt252) -> bool {
            self.minted_tokens.read(denom).is_zero()
        }

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

