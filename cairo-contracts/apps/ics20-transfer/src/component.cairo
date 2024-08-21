#[starknet::component]
pub mod ICS20TransferComponent {
    use core::array::ArrayTrait;
    use core::clone::Clone;
    use core::num::traits::Zero;
    use core::option::OptionTrait;
    use core::starknet::SyscallResultTrait;
    use starknet::ClassHash;
    use starknet::ContractAddress;
    use starknet::get_contract_address;
    use starknet_ibc_app_transfer::transferrable::ITransferrable;
    use starknet_ibc_app_transfer::types::{
        MsgTransfer, PrefixedDenom, Denom, DenomTrait, PacketData, TracePrefix, Memo,
        TracePrefixTrait, PrefixedDenomTrait
    };
    use starknet_ibc_app_transfer::{
        ERC20Contract, ERC20ContractTrait, ISendTransfer, IRecvPacket, ITokenAddress, TransferErrors
    };
    use starknet_ibc_core_channel::Packet;
    use starknet_ibc_core_host::{PortId, ChannelId, ChannelIdTrait};
    use starknet_ibc_utils::{ComputeKeyTrait, ValidateBasicTrait};

    #[storage]
    struct Storage {
        erc20_class_hash: ClassHash,
        salt: felt252,
        ibc_token_name_to_address: LegacyMap<felt252, ContractAddress>,
        ibc_token_address_to_name: LegacyMap<ContractAddress, felt252>,
    }

    #[event]
    #[derive(Debug, Drop, starknet::Event)]
    pub enum Event {
        SendEvent: SendEvent,
        RecvEvent: RecvEvent,
    }

    #[derive(Debug, Drop, Serde, starknet::Event)]
    pub struct SendEvent {
        #[key]
        pub sender: ContractAddress,
        #[key]
        pub receiver: ContractAddress,
        #[key]
        pub denom: PrefixedDenom,
        pub amount: u256,
        pub memo: Memo,
    }

    #[derive(Debug, Drop, Serde, starknet::Event)]
    pub struct RecvEvent {
        #[key]
        pub sender: ContractAddress,
        #[key]
        pub receiver: ContractAddress,
        #[key]
        pub denom: PrefixedDenom,
        pub amount: u256,
        pub memo: Memo,
        pub success: bool,
    }

    #[embeddable_as(SendTransfer)]
    impl SendTransferImpl<
        TContractState,
        +HasComponent<TContractState>,
        +ITransferrable<TContractState>,
        +Drop<TContractState>
    > of ISendTransfer<ComponentState<TContractState>> {
        fn send_validate(self: @ComponentState<TContractState>, msg: MsgTransfer) {
            self.get_contract().can_send();

            msg.validate_basic();

            match @msg.packet_data.denom.base {
                Denom::Native(erc20_token) => {
                    self
                        .escrow_validate(
                            msg.packet_data.sender.clone(),
                            msg.port_id_on_a.clone(),
                            msg.chan_id_on_a.clone(),
                            erc20_token.clone(),
                            msg.packet_data.amount,
                            msg.packet_data.memo.clone(),
                        );
                },
                Denom::Hosted(_) => {
                    self
                        .burn_validate(
                            msg.packet_data.sender.clone(),
                            msg.packet_data.denom.clone(),
                            msg.packet_data.amount,
                            msg.packet_data.memo.clone(),
                        );
                }
            }
        }

        fn send_execute(ref self: ComponentState<TContractState>, msg: MsgTransfer) {
            self.send_validate(msg.clone());

            match @msg.packet_data.denom.base {
                Denom::Native(erc20_token) => {
                    self
                        .escrow_execute(
                            msg.packet_data.sender.clone(),
                            erc20_token.clone(),
                            msg.packet_data.amount,
                            msg.packet_data.memo.clone(),
                        );
                },
                Denom::Hosted(_) => {
                    self
                        .burn_execute(
                            msg.packet_data.sender.clone(),
                            msg.packet_data.denom.clone(),
                            msg.packet_data.amount,
                            msg.packet_data.memo.clone(),
                        );
                }
            }

            self.emit_send_event(msg.packet_data);
        }
    }

    #[embeddable_as(RecvPacket)]
    impl RecvPacketImpl<
        TContractState,
        +HasComponent<TContractState>,
        +ITransferrable<TContractState>,
        +Drop<TContractState>
    > of IRecvPacket<ComponentState<TContractState>> {
        fn recv_validate(self: @ComponentState<TContractState>, packet: Packet) {
            self._recv_validate(packet);
        }

        fn recv_execute(ref self: ComponentState<TContractState>, packet: Packet) {
            self._recv_execute(packet);
        }
    }

    #[generate_trait]
    impl RecvPacketInternalImpl<
        TContractState,
        +HasComponent<TContractState>,
        +ITransferrable<TContractState>,
        +Drop<TContractState>
    > of RecvPacketInternalTrait<TContractState> {
        fn _recv_validate(self: @ComponentState<TContractState>, packet: Packet) -> PacketData {
            self.get_contract().can_receive();

            let mut pakcet_data_span = packet.data.span();

            let maybe_packet_data: Option<PacketData> = Serde::deserialize(ref pakcet_data_span);

            assert(maybe_packet_data.is_some(), TransferErrors::INVALID_PACKET_DATA);

            let packet_date = maybe_packet_data.unwrap();

            packet_date.validate_basic();

            match @packet_date.denom.base {
                Denom::Native(erc20_token) => {
                    self
                        .unescrow_validate(
                            packet_date.receiver.clone(),
                            packet.port_id_on_a.clone(),
                            packet.chan_id_on_a.clone(),
                            erc20_token.clone(),
                            packet_date.amount,
                        );
                },
                Denom::Hosted(_) => {
                    self
                        .mint_validate(
                            packet_date.receiver.clone(),
                            packet_date.denom.clone(),
                            packet_date.amount
                        );
                }
            }

            packet_date
        }

        fn _recv_execute(ref self: ComponentState<TContractState>, packet: Packet) -> PacketData {
            let mut packet_data = self._recv_validate(packet.clone());

            let trace_prefix = TracePrefixTrait::new(
                packet.port_id_on_b.clone(), packet.chan_id_on_b.clone()
            );

            match @packet_data.denom.base {
                Denom::Native(erc20_token) => {
                    packet_data.denom.remove_prefix(@trace_prefix);

                    self
                        .unescrow_execute(
                            packet_data.receiver.clone(),
                            packet.port_id_on_a.clone(),
                            packet.chan_id_on_a.clone(),
                            erc20_token.clone(),
                            packet_data.amount,
                        )
                },
                Denom::Hosted(_) => {
                    packet_data.denom.add_prefix(trace_prefix);

                    self
                        .mint_execute(
                            packet_data.receiver.clone(),
                            packet_data.denom.clone(),
                            packet_data.amount
                        )
                }
            };

            self.emit_recv_event(packet_data.clone(), true);

            packet_data
        }
    }

    #[embeddable_as(IBCTokenAddress)]
    impl ITokenAddressImpl<
        TContractState, +HasComponent<TContractState>, +Drop<TContractState>
    > of ITokenAddress<ComponentState<TContractState>> {
        fn ibc_token_address(
            self: @ComponentState<TContractState>, token_key: felt252
        ) -> Option<ContractAddress> {
            let token_address = self.ibc_token_name_to_address.read(token_key);

            if token_address.is_non_zero() {
                Option::Some(token_address)
            } else {
                Option::None
            }
        }
    }

    #[generate_trait]
    pub impl TransferValidationImpl<
        TContractState, +HasComponent<TContractState>, +Drop<TContractState>
    > of TransferValidationTrait<TContractState> {
        fn escrow_validate(
            self: @ComponentState<TContractState>,
            from_account: ContractAddress,
            port_id: PortId,
            channel_id: ChannelId,
            denom: ERC20Contract,
            amount: u256,
            memo: Memo,
        ) {
            let balance = denom.balance_of(from_account);

            assert(balance >= amount, TransferErrors::INSUFFICIENT_BALANCE);

            self.assert_non_ibc_token(denom, port_id, channel_id);
        }

        fn unescrow_validate(
            self: @ComponentState<TContractState>,
            to_account: ContractAddress,
            port_id: PortId,
            channel_id: ChannelId,
            denom: ERC20Contract,
            amount: u256,
        ) {
            let balance = denom.balance_of(get_contract_address());

            assert(balance >= amount, TransferErrors::INSUFFICIENT_BALANCE);

            self.assert_non_ibc_token(denom, port_id, channel_id);
        }

        fn mint_validate(
            self: @ComponentState<TContractState>,
            account: ContractAddress,
            denom: PrefixedDenom,
            amount: u256,
        ) { // NOTE: Normally, the minting process does not require any checks.
        // However, an implementer might choose to incorporate custom
        // checks, such as blacklisting.
        }

        fn burn_validate(
            self: @ComponentState<TContractState>,
            account: ContractAddress,
            denom: PrefixedDenom,
            amount: u256,
            memo: Memo,
        ) {
            let token_address: ERC20Contract = self
                .ibc_token_name_to_address
                .read(denom.compute_key())
                .into();

            let balance = token_address.balance_of(account);

            assert(balance >= amount, TransferErrors::INSUFFICIENT_BALANCE);
        }
    }

    #[generate_trait]
    pub impl TransferExecutionImpl<
        TContractState, +HasComponent<TContractState>, +Drop<TContractState>
    > of TransferExecutionTrait<TContractState> {
        fn escrow_execute(
            ref self: ComponentState<TContractState>,
            from_account: ContractAddress,
            denom: ERC20Contract,
            amount: u256,
            memo: Memo,
        ) {
            denom.transfer_from(from_account, get_contract_address(), amount);
        }

        fn unescrow_execute(
            ref self: ComponentState<TContractState>,
            to_account: ContractAddress,
            port_id: PortId,
            channel_id: ChannelId,
            denom: ERC20Contract,
            amount: u256,
        ) {
            denom.transfer(to_account, amount);
        }

        fn mint_execute(
            ref self: ComponentState<TContractState>,
            account: ContractAddress,
            denom: PrefixedDenom,
            amount: u256,
        ) {
            let token_address: ERC20Contract = self
                .ibc_token_name_to_address
                .read(denom.compute_key())
                .into();

            if token_address.is_non_zero() {
                token_address.mint(account, amount);
            } else {
                let name = denom.base.hosted().unwrap();

                let token_address = self.create_token(account, name, amount);

                self.record_ibc_token(denom, token_address);
            }
        }

        fn burn_execute(
            ref self: ComponentState<TContractState>,
            account: ContractAddress,
            denom: PrefixedDenom,
            amount: u256,
            memo: Memo,
        ) {
            let token_address: ERC20Contract = self
                .ibc_token_name_to_address
                .read(denom.compute_key())
                .into();

            token_address.burn(account, amount);
        }
    }

    #[generate_trait]
    pub impl TransferInternalImpl<
        TContractState, +HasComponent<TContractState>, +Drop<TContractState>
    > of TransferInternalTrait<TContractState> {
        fn initializer(ref self: ComponentState<TContractState>, erc20_class_hash: ClassHash) {
            self.erc20_class_hash.write(erc20_class_hash);
            self.salt.write(0);
        }

        fn create_token(
            ref self: ComponentState<TContractState>,
            account: ContractAddress,
            name: ByteArray,
            amount: u256,
        ) -> ContractAddress {
            let salt = self.salt.read();

            let mut symbol: ByteArray = "IBC/";

            symbol.append(@name);

            let erc20_token = ERC20ContractTrait::create(
                self.erc20_class_hash.read(),
                salt,
                name,
                symbol, // TODO: Determine what the symbol should be.
                amount,
                account,
                get_contract_address()
            );

            self.salt.write(salt + 1);

            erc20_token.address
        }

        fn record_ibc_token(
            ref self: ComponentState<TContractState>,
            denom: PrefixedDenom,
            token_address: ContractAddress,
        ) {
            let denom_key = denom.compute_key();

            self.ibc_token_name_to_address.write(denom_key, token_address);

            self.ibc_token_address_to_name.write(token_address, denom_key);
        }

        fn assert_non_ibc_token(
            self: @ComponentState<TContractState>,
            denom: ERC20Contract,
            port_id: PortId,
            channel_id: ChannelId,
        ) {
            let token_key = self.ibc_token_address_to_name.read(denom.address);

            if token_key.is_non_zero() {
                let trace_prefix = TracePrefixTrait::new(port_id, channel_id);

                let denom = PrefixedDenom {
                    trace_path: array![trace_prefix], base: Denom::Native(denom),
                };

                // Checks if the token is an IBC-created token. If so, it cannot
                // be transferred back to the source by escrowing. A prefixed
                // denom should be passed to burn instead.
                assert(token_key == denom.compute_key(), TransferErrors::INVALID_DENOM);
            }
        }
    }

    #[generate_trait]
    pub(crate) impl TransferEventImpl<
        TContractState, +HasComponent<TContractState>, +Drop<TContractState>
    > of TransferEventTrait<TContractState> {
        fn emit_send_event(ref self: ComponentState<TContractState>, packet_date: PacketData) {
            self
                .emit(
                    SendEvent {
                        sender: packet_date.sender,
                        receiver: packet_date.receiver,
                        denom: packet_date.denom,
                        amount: packet_date.amount,
                        memo: packet_date.memo,
                    }
                );
        }

        fn emit_recv_event(
            ref self: ComponentState<TContractState>, packet_date: PacketData, success: bool,
        ) {
            self
                .emit(
                    RecvEvent {
                        sender: packet_date.sender,
                        receiver: packet_date.receiver,
                        denom: packet_date.denom,
                        amount: packet_date.amount,
                        memo: packet_date.memo,
                        success,
                    }
                );
        }
    }
}

