use core::array::ArrayTrait;
use core::serde::Serde;
use core::to_byte_array::FormatAsByteArray;
use core::traits::TryInto;
use starknet::ContractAddress;
use starknet_ibc::apps::transfer::errors::ICS20Errors;

#[starknet::component]
pub mod ICS20TransferComponent {
    use core::array::ArrayTrait;
    use core::clone::Clone;
    use core::num::traits::Zero;
    use core::option::OptionTrait;
    use core::starknet::SyscallResultTrait;
    use core::traits::TryInto;
    use openzeppelin::token::erc20::{ERC20ABIDispatcher, ERC20ABIDispatcherTrait};
    use openzeppelin::utils::serde::SerializedAppend;
    use starknet::ClassHash;
    use starknet::ContractAddress;
    use starknet::get_caller_address;
    use starknet::get_contract_address;
    use starknet::syscalls::deploy_syscall;
    use starknet_ibc::apps::transfer::errors::ICS20Errors;
    use starknet_ibc::apps::transfer::interface::{ISendTransfer, IRecvPacket, ITransferrable};
    use starknet_ibc::apps::transfer::types::{
        MsgTransfer, Packet, Token, Denom, DenomTrait, Memo, MAXIMUM_MEMO_LENGTH
    };
    use starknet_ibc::core::types::{PortId, ChannelId};

    #[storage]
    struct Storage {
        erc20_class_hash: ClassHash,
        salt: felt252,
        governor: ContractAddress,
        send_capability: bool,
        receive_capability: bool,
        minted_token_name_to_address: LegacyMap<felt252, ContractAddress>,
        minted_token_address_to_name: LegacyMap<ContractAddress, felt252>,
    }

    #[event]
    #[derive(Debug, Drop, starknet::Event)]
    pub enum Event {
        TransferEvent: TransferEvent,
        RecvEvent: RecvEvent,
    }

    #[derive(Debug, Drop, Serde, starknet::Event)]
    pub struct TransferEvent {
        sender: ContractAddress,
        receiver: ContractAddress,
        amount: u256,
        denom: Denom,
        memo: Memo,
    }

    #[derive(Debug, Drop, Serde, starknet::Event)]
    pub struct RecvEvent {
        sender: ContractAddress,
        receiver: ContractAddress,
        denom: Denom,
        amount: u256,
        memo: Memo,
        success: bool,
    }

    #[embeddable_as(SendTransfer)]
    impl SendTransferImpl<
        TContractState, +HasComponent<TContractState>, +Drop<TContractState>
    > of ISendTransfer<ComponentState<TContractState>> {
        fn send_validate(self: @ComponentState<TContractState>, msg: MsgTransfer) {
            self._send_validate(msg);
        }

        fn send_execute(ref self: ComponentState<TContractState>, msg: MsgTransfer) {
            self._send_execute(msg);
        }
    }

    #[generate_trait]
    impl SendTransferInternalImpl<
        TContractState, +HasComponent<TContractState>, +Drop<TContractState>
    > of SendTransferInternal<TContractState> {
        fn _send_validate(self: @ComponentState<TContractState>, msg: MsgTransfer) -> Denom {
            self.can_send();

            assert(msg.packet_data.sender.is_non_zero(), ICS20Errors::INVALID_SENDER);
            assert(!msg.packet_data.token.denom.is_zero(), ICS20Errors::INVALID_DENOM);
            assert(msg.packet_data.token.amount.is_non_zero(), ICS20Errors::ZERO_AMOUNT);
            assert(
                msg.packet_data.memo.memo.len() < MAXIMUM_MEMO_LENGTH,
                ICS20Errors::MAXIMUM_MEMO_LENGTH
            );

            match @msg.packet_data.token.denom {
                Denom::Native(_) => {
                    self
                        .escrow_validate(
                            msg.packet_data.sender.clone(),
                            msg.port_id_on_a.clone(),
                            msg.chan_id_on_a.clone(),
                            msg.packet_data.token.clone(),
                            msg.packet_data.memo.clone(),
                        );
                },
                Denom::IBC(_) => {
                    self
                        .burn_validate(
                            msg.packet_data.sender.clone(),
                            msg.packet_data.token.clone(),
                            msg.packet_data.memo.clone(),
                        );
                }
            }

            msg.packet_data.token.denom
        }

        fn _send_execute(ref self: ComponentState<TContractState>, msg: MsgTransfer) -> Denom {
            let denom = self._send_validate(msg.clone());

            match @denom {
                Denom::Native(_) => {
                    self
                        .escrow_execute(
                            msg.packet_data.sender.clone(),
                            msg.port_id_on_a.clone(),
                            msg.chan_id_on_a.clone(),
                            msg.packet_data.token.clone(),
                            msg.packet_data.memo.clone(),
                        );
                },
                Denom::IBC(_) => {
                    self
                        .burn_execute(
                            msg.packet_data.sender.clone(),
                            msg.packet_data.token.clone(),
                            msg.packet_data.memo.clone(),
                        );
                }
            }

            self
                .emit(
                    TransferEvent {
                        sender: msg.packet_data.sender.clone(),
                        receiver: msg.packet_data.receiver.clone(),
                        amount: msg.packet_data.token.amount,
                        denom: denom.clone(),
                        memo: msg.packet_data.memo.clone(),
                    }
                );

            denom
        }
    }

    #[embeddable_as(RecvPacket)]
    impl RecvPacketImpl<
        TContractState, +HasComponent<TContractState>, +Drop<TContractState>
    > of IRecvPacket<ComponentState<TContractState>> {
        fn recv_validate(self: @ComponentState<TContractState>, packet: Packet) {
            self._recv_validate(packet);
        }

        fn recv_execute(ref self: ComponentState<TContractState>, packet: Packet) {
            self._recv_execute(packet);
        }
    }

    #[generate_trait]
    impl RecvPacketInernalImpl<
        TContractState, +HasComponent<TContractState>, +Drop<TContractState>
    > of RecvPacketInternal<TContractState> {
        fn _recv_validate(self: @ComponentState<TContractState>, packet: Packet) -> Denom {
            self.can_receive();

            assert(packet.data.receiver.is_non_zero(), ICS20Errors::INVALID_RECEIVEER);
            assert(!packet.data.token.denom.is_zero(), ICS20Errors::INVALID_DENOM);
            assert(packet.data.token.amount.is_non_zero(), ICS20Errors::ZERO_AMOUNT);

            match @packet.data.token.denom {
                Denom::Native(_) => {
                    self
                        .unescrow_validate(
                            packet.data.receiver.clone(),
                            packet.port_id_on_a.clone(),
                            packet.chan_id_on_a.clone(),
                            packet.data.token.clone(),
                        );
                },
                Denom::IBC(_) => {
                    self.mint_validate(packet.data.receiver.clone(), packet.data.token.clone(),);
                }
            }

            packet.data.token.denom
        }

        fn _recv_execute(ref self: ComponentState<TContractState>, packet: Packet) -> Denom {
            let denom = self._recv_validate(packet.clone());

            match @denom {
                Denom::Native(_) => {
                    self
                        .unescrow_execute(
                            packet.data.receiver.clone(),
                            packet.port_id_on_a.clone(),
                            packet.chan_id_on_a.clone(),
                            packet.data.token.clone(),
                        );
                },
                Denom::IBC(_) => {
                    self.mint_execute(packet.data.receiver.clone(), packet.data.token.clone(),);
                }
            }

            self
                .emit(
                    RecvEvent {
                        sender: packet.data.sender.clone(),
                        receiver: packet.data.receiver.clone(),
                        denom: denom.clone(),
                        amount: packet.data.token.amount,
                        memo: packet.data.memo.clone(),
                        success: true,
                    }
                );

            denom
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

    #[generate_trait]
    pub(crate) impl TransferValidationImpl<
        TContractState, +HasComponent<TContractState>,
    > of TransferValidationTrait<TContractState> {
        fn escrow_validate(
            self: @ComponentState<TContractState>,
            from_account: ContractAddress,
            port_id: PortId,
            channel_id: ChannelId,
            token: Token,
            memo: Memo,
        ) {
            let contract_address = token.denom.native().unwrap();
            let balance = ERC20ABIDispatcher { contract_address }.balance_of(from_account);
            assert(balance > token.amount, ICS20Errors::INSUFFICIENT_BALANCE);
        }

        fn unescrow_validate(
            self: @ComponentState<TContractState>,
            to_account: ContractAddress,
            port_id: PortId,
            channel_id: ChannelId,
            token: Token,
        ) {
            let contract_address = token.denom.native().unwrap();
            let balance = ERC20ABIDispatcher { contract_address }
                .balance_of(get_contract_address());
            assert(token.amount > balance, ICS20Errors::INSUFFICIENT_BALANCE);
        }

        fn mint_validate(
            self: @ComponentState<TContractState>, account: ContractAddress, token: Token,
        ) {}

        fn burn_validate(
            self: @ComponentState<TContractState>,
            account: ContractAddress,
            token: Token,
            memo: Memo,
        ) {
            let contract_address = self
                .minted_token_name_to_address
                .read(token.denom.ibc().unwrap());
            let balance = ERC20ABIDispatcher { contract_address }.balance_of(account);
            assert(token.amount > balance, ICS20Errors::INSUFFICIENT_BALANCE);
        }
    }

    #[generate_trait]
    pub(crate) impl TransferExecutionImpl<
        TContractState, +HasComponent<TContractState>, +Drop<TContractState>
    > of TransferExecutionTrait<TContractState> {
        fn escrow_execute(
            ref self: ComponentState<TContractState>,
            from_account: ContractAddress,
            port_id: PortId,
            channel_id: ChannelId,
            token: Token,
            memo: Memo,
        ) {
            let contract_address = token.denom.native().unwrap();
            ERC20ABIDispatcher { contract_address }
                .transfer_from(from_account, get_contract_address(), token.amount);
        }

        fn unescrow_execute(
            ref self: ComponentState<TContractState>,
            to_account: ContractAddress,
            port_id: PortId,
            channel_id: ChannelId,
            token: Token,
        ) {}

        fn mint_execute(
            ref self: ComponentState<TContractState>, account: ContractAddress, token: Token,
        ) {
            let ibc_denom = token.denom.ibc().unwrap();

            let contract_address = self.minted_token_name_to_address.read(ibc_denom);

            let contract_address = if contract_address.is_non_zero() {
                contract_address
            } else {
                self.create_token(account, token)
            };

            ERC20ABIDispatcher { contract_address }.transfer(account, token.amount);
        }

        fn burn_execute(
            ref self: ComponentState<TContractState>,
            account: ContractAddress,
            token: Token,
            memo: Memo,
        ) {}
    }

    #[generate_trait]
    pub(crate) impl TransferInternalImpl<
        TContractState, +HasComponent<TContractState>, +Drop<TContractState>
    > of TransferInternalTrait<TContractState> {
        fn initializer(ref self: ComponentState<TContractState>, erc20_class_hash: ClassHash) {
            self.governor.write(get_caller_address());
            self.erc20_class_hash.write(erc20_class_hash);
            self.salt.write(0);
            self.send_capability.write(true);
            self.receive_capability.write(true);
        }

        fn create_token(
            ref self: ComponentState<TContractState>, account: ContractAddress, token: Token
        ) -> ContractAddress {
            let salt = self.salt.read();

            let mut call_data = array![];
            call_data.append_serde(token.denom.clone());
            call_data
                .append_serde(token.denom); // TODO: determine what should be set as symbol here.
            call_data.append_serde(token.amount);
            call_data.append_serde(account);
            call_data.append_serde(get_contract_address());

            let (address, _) = deploy_syscall(
                self.erc20_class_hash.read(), salt, call_data.span(), false,
            )
                .unwrap_syscall();

            self.salt.write(salt + 1);

            address
        }
    }
}

