#[starknet::component]
pub mod TokenTransferComponent {
    use core::array::ArrayTrait;
    use core::clone::Clone;
    use core::num::traits::Zero;
    use core::option::OptionTrait;
    use core::traits::TryInto;
    use openzeppelin_access::ownable::OwnableComponent;
    use openzeppelin_access::ownable::interface::IOwnable;
    use starknet::ClassHash;
    use starknet::ContractAddress;
    use starknet::storage::{
        Map, StorageMapReadAccess, StorageMapWriteAccess, StoragePointerReadAccess,
        StoragePointerWriteAccess,
    };
    use starknet::{get_caller_address, get_contract_address};
    use starknet_ibc_apps::transfer::types::{
        Denom, DenomTrait, Memo, MsgTransfer, PacketData, Participant, ParticipantTrait,
        PrefixedDenom, PrefixedDenomTrait, TracePrefixTrait,
    };
    use starknet_ibc_apps::transfer::{
        ERC20Contract, ERC20ContractTrait, ICreateIbcToken, ISendTransfer, ITransferQuery,
        ITransferrable, SUCCESS_ACK, TRANSFER_PORT_ID, TransferErrors, VERSION,
    };
    use starknet_ibc_core::channel::{
        AckStatus, AckStatusImpl, Acknowledgement, AppVersion, ChannelContract,
        ChannelContractTrait, ChannelEndTrait, ChannelErrors, ChannelOrdering, IAppCallback, Packet,
    };

    use starknet_ibc_core::host::{ChannelId, ConnectionId, PortId};
    use starknet_ibc_utils::{ComputeKey, ValidateBasic};

    #[storage]
    pub struct Storage {
        erc20_class_hash: ClassHash,
        salt: felt252,
        ibc_token_key_to_address: Map<felt252, ContractAddress>,
        ibc_token_address_to_key: Map<ContractAddress, felt252>,
    }

    #[event]
    #[derive(Debug, Drop, starknet::Event)]
    pub enum Event {
        SendEvent: SendEvent,
        RecvEvent: RecvEvent,
        AckEvent: AckEvent,
        AckStatusEvent: AckStatusEvent,
        TimeoutEvent: TimeoutEvent,
        CreateTokenEvent: CreateTokenEvent,
    }

    #[derive(Debug, Drop, Serde, starknet::Event)]
    pub struct SendEvent {
        #[key]
        pub sender: ContractAddress,
        #[key]
        pub receiver: ByteArray,
        #[key]
        pub denom: PrefixedDenom,
        pub amount: u256,
        pub memo: Memo,
    }

    #[derive(Debug, Drop, Serde, starknet::Event)]
    pub struct RecvEvent {
        #[key]
        pub sender: ByteArray,
        #[key]
        pub receiver: ContractAddress,
        #[key]
        pub denom: PrefixedDenom,
        pub amount: u256,
        pub memo: Memo,
        pub success: bool,
    }

    #[derive(Debug, Drop, Serde, starknet::Event)]
    pub struct AckEvent {
        #[key]
        pub sender: ContractAddress,
        #[key]
        pub receiver: ByteArray,
        #[key]
        pub denom: PrefixedDenom,
        pub amount: u256,
        pub memo: Memo,
        pub ack: Acknowledgement,
    }

    #[derive(Debug, Drop, Serde, starknet::Event)]
    pub struct AckStatusEvent {
        #[key]
        pub ack_status: AckStatus,
    }

    #[derive(Debug, Drop, Serde, starknet::Event)]
    pub struct TimeoutEvent {
        #[key]
        pub receiver: ByteArray,
        #[key]
        pub denom: PrefixedDenom,
        pub amount: u256,
        pub memo: Memo,
    }

    #[derive(Debug, Drop, Serde, starknet::Event)]
    pub struct CreateTokenEvent {
        #[key]
        pub name: ByteArray,
        #[key]
        pub symbol: ByteArray,
        #[key]
        pub decimals: u8,
        #[key]
        pub address: ContractAddress,
    }

    // -----------------------------------------------------------
    // Transfer Initializer
    // -----------------------------------------------------------

    #[generate_trait]
    pub impl TransferInitializerImpl<
        TContractState, +HasComponent<TContractState>, +Drop<TContractState>,
    > of TransferInitializerTrait<TContractState> {
        fn initializer(ref self: ComponentState<TContractState>, erc20_class_hash: ClassHash) {
            assert(erc20_class_hash.is_non_zero(), TransferErrors::ZERO_ERC20_CLASS_HASH);

            self.write_erc20_class_hash(erc20_class_hash);

            self.write_salt(1);
        }
    }

    // -----------------------------------------------------------
    // ISendTransfer
    // -----------------------------------------------------------

    #[embeddable_as(SendTransfer)]
    impl SendTransferImpl<
        TContractState,
        +HasComponent<TContractState>,
        +ITransferrable<TContractState>,
        +Drop<TContractState>,
        impl Ownable: OwnableComponent::HasComponent<TContractState>,
    > of ISendTransfer<ComponentState<TContractState>> {
        // NOTE: We first ensure the validity of the incoming message at the
        // application level. Then, send it through the IBC core contract for
        // validations related to the TAO layer. If everything checks out, the
        // packet is first executed in the core contract, followed by execution
        // at the application level.
        fn send_transfer(ref self: ComponentState<TContractState>, msg: MsgTransfer) {
            let sender = get_caller_address();

            self.send_validate(msg.clone(), sender);

            let channel: ChannelContract = self.owner().into();

            let packet = self.construct_send_packet(@channel, msg.clone());

            channel.send_packet(packet);

            self.send_execute(msg, sender);
        }
    }

    // -----------------------------------------------------------
    // IAppCallback
    // -----------------------------------------------------------

    #[embeddable_as(TransferAppCallback)]
    impl TransferAppCallbackImpl<
        TContractState,
        +HasComponent<TContractState>,
        +ITransferrable<TContractState>,
        +Drop<TContractState>,
        impl Ownable: OwnableComponent::HasComponent<TContractState>,
    > of IAppCallback<ComponentState<TContractState>> {
        fn on_chan_open_init(
            ref self: ComponentState<TContractState>,
            port_id_on_a: PortId,
            chan_id_on_a: ChannelId,
            conn_id_on_a: ConnectionId,
            port_id_on_b: PortId,
            version_proposal: AppVersion,
            ordering: ChannelOrdering,
        ) -> AppVersion {
            self.assert_owner();

            assert(port_id_on_a == TRANSFER_PORT_ID(), TransferErrors::INVALID_PORT_ID);

            if version_proposal.is_non_zero() {
                assert(version_proposal == VERSION(), TransferErrors::INVALID_APP_VERSION);
            }

            assert(ordering == ChannelOrdering::Unordered, TransferErrors::UNSUPPORTED_ORDERING);

            VERSION()
        }

        fn on_chan_open_try(
            ref self: ComponentState<TContractState>,
            port_id_on_b: PortId,
            chan_id_on_b: ChannelId,
            conn_id_on_b: ConnectionId,
            port_id_on_a: PortId,
            version_on_a: AppVersion,
            ordering: ChannelOrdering,
        ) -> AppVersion {
            self.assert_owner();

            assert(version_on_a == VERSION(), TransferErrors::INVALID_APP_VERSION);

            VERSION()
        }

        fn on_chan_open_ack(
            ref self: ComponentState<TContractState>,
            port_id_on_a: PortId,
            chan_id_on_a: ChannelId,
            version_on_b: AppVersion,
        ) {
            self.assert_owner();

            assert(version_on_b == VERSION(), TransferErrors::INVALID_APP_VERSION);
        }

        fn on_chan_open_confirm(
            ref self: ComponentState<TContractState>, port_id_on_b: PortId, chan_id_on_b: ChannelId,
        ) {
            self.assert_owner();
        }

        fn on_recv_packet(
            ref self: ComponentState<TContractState>, packet: Packet,
        ) -> Acknowledgement {
            self.assert_owner();

            let (mut packet_data, receiver) = self.recv_deserialize(packet.clone());

            self.recv_validate(packet.clone(), packet_data.clone(), receiver);

            self.recv_execute(packet, ref packet_data, receiver);

            SUCCESS_ACK()
        }

        fn on_ack_packet(
            ref self: ComponentState<TContractState>, packet: Packet, ack: Acknowledgement,
        ) {
            self.assert_owner();

            let (packet_data, ack_status) = self.ack_deserialize(packet.clone(), ack);

            self.ack_validate(@packet, @packet_data, @ack_status);

            self.ack_execute(packet, packet_data, ack_status);
        }

        fn on_timeout_packet(ref self: ComponentState<TContractState>, packet: Packet) {
            self.assert_owner();

            let packet_data = packet.data.clone().into();

            self.timeout_validate(@packet, @packet_data);

            self.timeout_execute(packet, packet_data);
        }

        fn json_packet_data(
            self: @ComponentState<TContractState>, raw_packet_data: Array<felt252>,
        ) -> ByteArray {
            let packet_data: PacketData = raw_packet_data.into();

            serde_json::to_byte_array(packet_data)
        }
    }

    // -----------------------------------------------------------
    // ICreateToken
    // -----------------------------------------------------------

    #[embeddable_as(CreateIbcToken)]
    impl CreateTokenImpl<
        TContractState,
        +HasComponent<TContractState>,
        +Drop<TContractState>,
        impl Ownable: OwnableComponent::HasComponent<TContractState>,
    > of ICreateIbcToken<ComponentState<TContractState>> {
        fn create_ibc_token(
            ref self: ComponentState<TContractState>, denom: PrefixedDenom,
        ) -> ContractAddress {
            let mut token = self.get_token(denom.key());
            assert(token.is_zero(), TransferErrors::TOKEN_ALREADY_EXISTS);

            let channel: ChannelContract = self.owner().into();

            let maybe_prefix = denom.first_prefix();
            assert(maybe_prefix.is_some(), TransferErrors::MISSING_TRACE_PREFIX);
            let prefix = maybe_prefix.unwrap();

            assert(prefix.port_id == @TRANSFER_PORT_ID(), TransferErrors::INVALID_PORT_ID);

            let chan_end_on_b = channel
                .channel_end(prefix.port_id.clone(), prefix.channel_id.clone());
            assert(chan_end_on_b.is_open(), ChannelErrors::INVALID_CHANNEL_STATE);

            let maybe_base = denom.base.hosted();
            assert(maybe_base.is_some(), TransferErrors::INVALID_DENOM);
            let base = maybe_base.unwrap();

            let token = self.create_token(base, 0);
            self.record_ibc_token(denom, token.address);
            token.address
        }
    }

    // -----------------------------------------------------------
    // ITokenAddress
    // -----------------------------------------------------------

    #[embeddable_as(TokenTransferQuery)]
    impl ITransferQueryImpl<
        TContractState, +HasComponent<TContractState>, +Drop<TContractState>,
    > of ITransferQuery<ComponentState<TContractState>> {
        fn ibc_token_address(
            self: @ComponentState<TContractState>, token_key: felt252,
        ) -> ContractAddress {
            let address = self.read_ibc_token_address(token_key);

            assert(address.is_non_zero(), TransferErrors::ZERO_TOKEN_ADDRESS);

            address
        }
    }

    // -----------------------------------------------------------
    // Transfer Handlers
    // -----------------------------------------------------------

    #[generate_trait]
    pub impl SendTransferInternalImpl<
        TContractState,
        +HasComponent<TContractState>,
        +ITransferrable<TContractState>,
        +Drop<TContractState>,
    > of SendTransferInternalTrait<TContractState> {
        fn send_validate(
            self: @ComponentState<TContractState>, msg: MsgTransfer, sender: ContractAddress,
        ) {
            self.get_contract().can_send();

            msg.validate_basic();

            assert(sender.is_non_zero(), TransferErrors::ZERO_SENDER);

            match @msg.denom.base {
                Denom::Native(erc20_token) => {
                    self
                        .escrow_validate(
                            sender,
                            msg.port_id_on_a.clone(),
                            msg.chan_id_on_a.clone(),
                            erc20_token.clone(),
                            msg.amount,
                            msg.memo.clone(),
                        );
                },
                Denom::Hosted(_) => {
                    self.burn_validate(sender, msg.denom.clone(), msg.amount, msg.memo.clone());
                },
            }
        }

        fn send_execute(
            ref self: ComponentState<TContractState>, msg: MsgTransfer, sender: ContractAddress,
        ) {
            match @msg.denom.base {
                Denom::Native(erc20_token) => {
                    self.escrow_execute(sender, erc20_token.clone(), msg.amount, msg.memo.clone());
                },
                Denom::Hosted(_) => {
                    self.burn_execute(sender, msg.denom.clone(), msg.amount, msg.memo.clone());
                },
            }

            self
                .emit_send_event(
                    sender, msg.receiver.clone(), msg.denom.clone(), msg.amount, msg.memo.clone(),
                )
        }

        fn construct_send_packet(
            self: @ComponentState<TContractState>, channel: @ChannelContract, msg: MsgTransfer,
        ) -> Packet {
            let chan_end_on_a = channel
                .channel_end(msg.port_id_on_a.clone(), msg.chan_id_on_a.clone());

            let port_id_on_b = chan_end_on_a.counterparty_port_id().clone();

            let chan_id_on_b = chan_end_on_a.counterparty_channel_id().clone();

            let seq_on_a = channel
                .next_sequence_send(msg.port_id_on_a.clone(), msg.chan_id_on_a.clone());

            let mut data: Array<felt252> = ArrayTrait::new();

            let packet_data = PacketData {
                sender: Participant::Native(get_caller_address()),
                receiver: Participant::External(msg.receiver.clone()),
                denom: msg.denom.clone(),
                amount: msg.amount,
                memo: msg.memo.clone(),
            };

            packet_data.serialize(ref data);

            Packet {
                seq_on_a,
                port_id_on_a: msg.port_id_on_a,
                chan_id_on_a: msg.chan_id_on_a,
                port_id_on_b,
                chan_id_on_b,
                data,
                timeout_height_on_b: msg.timeout_height_on_b,
                timeout_timestamp_on_b: msg.timeout_timestamp_on_b,
            }
        }
    }

    #[generate_trait]
    pub(crate) impl RecvPacketInternalImpl<
        TContractState,
        +HasComponent<TContractState>,
        +ITransferrable<TContractState>,
        +Drop<TContractState>,
    > of RecvPacketInternalTrait<TContractState> {
        fn recv_deserialize(
            self: @ComponentState<TContractState>, packet: Packet,
        ) -> (PacketData, ContractAddress) {
            let packet_data: PacketData = packet.data.into();

            let receiver: Option<ContractAddress> = packet_data.receiver.clone().try_into();

            assert(receiver.is_some(), TransferErrors::INVALID_RECEIVER);

            (packet_data, receiver.unwrap())
        }

        fn recv_validate(
            self: @ComponentState<TContractState>,
            packet: Packet,
            packet_data: PacketData,
            receiver: ContractAddress,
        ) {
            self.get_contract().can_receive();

            packet.validate_basic();

            packet_data.validate_basic();

            assert(packet_data.sender.external().is_some(), TransferErrors::INVALID_SENDER);
            assert(packet_data.receiver.native().is_some(), TransferErrors::INVALID_RECEIVER);

            match @packet_data.denom.base {
                Denom::Native(erc20_token) => {
                    self
                        .unescrow_validate(
                            receiver,
                            packet.port_id_on_a.clone(),
                            packet.chan_id_on_a.clone(),
                            erc20_token.clone(),
                            packet_data.amount,
                        );
                },
                Denom::Hosted(_) => {
                    self.mint_validate(receiver, packet_data.denom.clone(), packet_data.amount);
                },
            }
        }

        fn recv_execute(
            ref self: ComponentState<TContractState>,
            packet: Packet,
            ref packet_data: PacketData,
            receiver: ContractAddress,
        ) {
            let trace_prefix = TracePrefixTrait::new(
                packet.port_id_on_b.clone(), packet.chan_id_on_b.clone(),
            );

            match @packet_data.denom.base {
                Denom::Native(erc20_token) => {
                    self
                        .unescrow_execute(
                            receiver,
                            packet.port_id_on_a.clone(),
                            packet.chan_id_on_a.clone(),
                            erc20_token.clone(),
                            packet_data.amount,
                        )
                },
                Denom::Hosted(_) => {
                    packet_data.denom.add_prefix(trace_prefix);

                    self.mint_execute(receiver, packet_data.denom.clone(), packet_data.amount)
                },
            };

            self.emit_recv_event(packet_data.clone(), true);
        }
    }

    #[generate_trait]
    pub(crate) impl AckPacketInternalImpl<
        TContractState,
        +HasComponent<TContractState>,
        +ITransferrable<TContractState>,
        +Drop<TContractState>,
    > of AckPacketInternalTrait<TContractState> {
        fn ack_deserialize(
            self: @ComponentState<TContractState>, packet: Packet, ack: Acknowledgement,
        ) -> (PacketData, AckStatus) {
            let packet_data = packet.data.into();

            let ack_status = AckStatusImpl::new(ack, @SUCCESS_ACK());

            (packet_data, ack_status)
        }

        fn ack_validate(
            self: @ComponentState<TContractState>,
            packet: @Packet,
            packet_data: @PacketData,
            ack_status: @AckStatus,
        ) {
            packet.validate_basic();

            packet_data.validate_basic();

            assert(packet_data.sender.native().is_some(), TransferErrors::INVALID_SENDER);
            assert(packet_data.receiver.external().is_some(), TransferErrors::INVALID_RECEIVER);
            assert(ack_status.is_non_empty(), TransferErrors::EMPTY_ACK_STATUS);

            if ack_status.is_error() {
                self.refund_validate(packet.clone(), packet_data.clone());
            }
        }

        fn ack_execute(
            ref self: ComponentState<TContractState>,
            packet: Packet,
            packet_data: PacketData,
            ack_status: AckStatus,
        ) {
            if ack_status.is_error() {
                self.refund_execute(packet, packet_data.clone());
            }

            self.emit_ack_event(packet_data, ack_status.ack().clone());

            self.emit_ack_status_event(ack_status);
        }
    }

    #[generate_trait]
    pub(crate) impl TimeoutPacketInternalImpl<
        TContractState,
        +HasComponent<TContractState>,
        +ITransferrable<TContractState>,
        +Drop<TContractState>,
    > of TimeoutPacketInternalTrait<TContractState> {
        fn timeout_validate(
            self: @ComponentState<TContractState>, packet: @Packet, packet_data: @PacketData,
        ) {
            packet.validate_basic();

            packet_data.validate_basic();

            assert(packet_data.receiver.external().is_some(), TransferErrors::INVALID_RECEIVER);

            self.refund_validate(packet.clone(), packet_data.clone());
        }

        fn timeout_execute(
            ref self: ComponentState<TContractState>, packet: Packet, packet_data: PacketData,
        ) {
            self.refund_execute(packet, packet_data.clone());

            self.emit_timeout_event(packet_data);
        }
    }

    // -----------------------------------------------------------
    // Transfer Validation/Execution
    // -----------------------------------------------------------

    #[generate_trait]
    pub impl TransferValidationImpl<
        TContractState, +HasComponent<TContractState>, +Drop<TContractState>,
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
            let token = self.get_token(denom.key());

            assert(token.is_non_zero(), TransferErrors::ZERO_TOKEN_ADDRESS);

            let balance = token.balance_of(account);

            assert(balance >= amount, TransferErrors::INSUFFICIENT_BALANCE);
        }

        fn refund_validate(
            self: @ComponentState<TContractState>, packet: Packet, packet_data: PacketData,
        ) {
            let sender: Option<ContractAddress> = packet_data.sender.try_into();

            assert(sender.is_some(), TransferErrors::INVALID_SENDER);

            match @packet_data.denom.base {
                Denom::Native(erc20_token) => {
                    self
                        .unescrow_validate(
                            sender.unwrap(),
                            packet.port_id_on_a,
                            packet.chan_id_on_a,
                            erc20_token.clone(),
                            packet_data.amount,
                        )
                },
                Denom::Hosted(_) => {
                    self.mint_validate(sender.unwrap(), packet_data.denom, packet_data.amount)
                },
            };
        }
    }

    #[generate_trait]
    pub impl TransferExecutionImpl<
        TContractState, +HasComponent<TContractState>, +Drop<TContractState>,
    > of TransferExecutionTrait<TContractState> {
        fn escrow_execute(
            ref self: ComponentState<TContractState>,
            from_account: ContractAddress,
            token: ERC20Contract,
            amount: u256,
            memo: Memo,
        ) {
            token.transfer_from(from_account, get_contract_address(), amount);
        }

        fn unescrow_execute(
            ref self: ComponentState<TContractState>,
            to_account: ContractAddress,
            port_id: PortId,
            channel_id: ChannelId,
            token: ERC20Contract,
            amount: u256,
        ) {
            token.transfer(to_account, amount);
        }

        fn mint_execute(
            ref self: ComponentState<TContractState>,
            account: ContractAddress,
            denom: PrefixedDenom,
            amount: u256,
        ) {
            let mut token = self.get_token(denom.key());
            if token.is_zero() {
                let name = denom.base.hosted().unwrap();

                token = self.create_token(name, amount);

                self.record_ibc_token(denom, token.address);
            }
            token.mint(get_contract_address(), amount);
            token.transfer(account, amount);
        }

        fn burn_execute(
            ref self: ComponentState<TContractState>,
            account: ContractAddress,
            denom: PrefixedDenom,
            amount: u256,
            memo: Memo,
        ) {
            let token = self.get_token(denom.key());

            token.transfer_from(account, get_contract_address(), amount);

            token.burn(get_contract_address(), amount);
        }

        fn refund_execute(
            ref self: ComponentState<TContractState>, packet: Packet, packet_data: PacketData,
        ) {
            let sender: Option<ContractAddress> = packet_data.sender.try_into();

            match @packet_data.denom.base {
                Denom::Native(erc20_token) => {
                    self
                        .unescrow_execute(
                            sender.unwrap(),
                            packet.port_id_on_a,
                            packet.chan_id_on_a,
                            erc20_token.clone(),
                            packet_data.amount,
                        )
                },
                Denom::Hosted(_) => {
                    self.mint_execute(sender.unwrap(), packet_data.denom, packet_data.amount)
                },
            };
        }
    }

    // -----------------------------------------------------------
    // Transfer Owner
    // -----------------------------------------------------------

    #[generate_trait]
    pub(crate) impl TransferOwnerImpl<
        TContractState,
        +HasComponent<TContractState>,
        +Drop<TContractState>,
        impl Ownable: OwnableComponent::HasComponent<TContractState>,
    > of TransferOwnerTrait<TContractState> {
        fn owner(self: @ComponentState<TContractState>) -> ContractAddress {
            get_dep_component!(self, Ownable).owner()
        }

        fn assert_owner(self: @ComponentState<TContractState>) {
            assert(self.owner() == get_caller_address(), TransferErrors::INVALID_OWNER);
        }
    }

    // -----------------------------------------------------------
    // Transfer Internal
    // -----------------------------------------------------------

    #[generate_trait]
    pub(crate) impl TransferInternalImpl<
        TContractState, +HasComponent<TContractState>, +Drop<TContractState>,
    > of TransferInternalTrait<TContractState> {
        fn get_token(self: @ComponentState<TContractState>, token_key: felt252) -> ERC20Contract {
            self.read_ibc_token_address(token_key).into()
        }

        fn create_token(
            ref self: ComponentState<TContractState>, name: ByteArray, amount: u256,
        ) -> ERC20Contract {
            let salt = self.read_salt();

            // TODO: Determine what the symbol should be.
            let mut symbol: ByteArray = "IBC/";
            symbol.append(@name);

            // TODO: Determine what the decimals should be.
            let decimals = 0;

            let erc20_token = ERC20ContractTrait::create(
                self.read_erc20_class_hash(),
                salt,
                name.clone(),
                symbol.clone(),
                decimals,
                get_contract_address(),
            );

            self.write_salt(salt + 1);

            self.emit_create_token_event(name, symbol, decimals, erc20_token.address);

            erc20_token
        }

        fn record_ibc_token(
            ref self: ComponentState<TContractState>,
            denom: PrefixedDenom,
            token_address: ContractAddress,
        ) {
            let denom_key = denom.key();

            self.write_ibc_token_key_to_address(denom_key, token_address);

            self.write_ibc_token_address_to_key(token_address, denom_key);
        }

        fn assert_non_ibc_token(
            self: @ComponentState<TContractState>,
            denom: ERC20Contract,
            port_id: PortId,
            channel_id: ChannelId,
        ) {
            let token_key = self.read_ibc_token_key(denom.address);

            if token_key.is_non_zero() {
                let trace_prefix = TracePrefixTrait::new(port_id, channel_id);

                let denom = PrefixedDenom {
                    trace_path: array![trace_prefix], base: Denom::Native(denom),
                };

                // Checks if the token is an IBC-created token. If so, it cannot
                // be transferred back to the source by escrowing. A prefixed
                // denom should be passed to burn instead.
                assert(token_key == denom.key(), TransferErrors::INVALID_DENOM);
            }
        }
    }

    // -----------------------------------------------------------
    // Transfer Reader/Writer
    // -----------------------------------------------------------

    #[generate_trait]
    pub(crate) impl TransferReaderImpl<
        TContractState, +HasComponent<TContractState>, +Drop<TContractState>,
    > of TransferReaderTrait<TContractState> {
        fn read_erc20_class_hash(self: @ComponentState<TContractState>) -> ClassHash {
            let class_hash = self.erc20_class_hash.read();

            assert(class_hash.is_non_zero(), TransferErrors::ZERO_ERC20_CLASS_HASH);

            class_hash
        }

        fn read_salt(self: @ComponentState<TContractState>) -> felt252 {
            let salt = self.salt.read();

            assert(salt.is_non_zero(), TransferErrors::ZERO_SALT);

            salt
        }

        // NOTE: The `read_ibc_token_address` and `read_ibc_token_key` methods
        // do not reject cases where the value might be zero (non-existent). As
        // these methods are also called internally where there is logic for
        // handling non-existent cases.

        fn read_ibc_token_address(
            self: @ComponentState<TContractState>, token_key: felt252,
        ) -> ContractAddress {
            self.ibc_token_key_to_address.read(token_key)
        }

        fn read_ibc_token_key(
            self: @ComponentState<TContractState>, token_address: ContractAddress,
        ) -> felt252 {
            self.ibc_token_address_to_key.read(token_address)
        }
    }

    #[generate_trait]
    pub(crate) impl TransferWriterImpl<
        TContractState, +HasComponent<TContractState>, +Drop<TContractState>,
    > of TransferWriterTrait<TContractState> {
        fn write_erc20_class_hash(
            ref self: ComponentState<TContractState>, erc20_class_hash: ClassHash,
        ) {
            self.erc20_class_hash.write(erc20_class_hash);
        }

        fn write_salt(ref self: ComponentState<TContractState>, salt: felt252) {
            self.salt.write(salt);
        }

        fn write_ibc_token_key_to_address(
            ref self: ComponentState<TContractState>,
            token_key: felt252,
            token_address: ContractAddress,
        ) {
            self.ibc_token_key_to_address.write(token_key, token_address);
        }

        fn write_ibc_token_address_to_key(
            ref self: ComponentState<TContractState>,
            token_address: ContractAddress,
            token_key: felt252,
        ) {
            self.ibc_token_address_to_key.write(token_address, token_key);
        }
    }

    // -----------------------------------------------------------
    // Transfer Event Emitter
    // -----------------------------------------------------------

    #[generate_trait]
    pub(crate) impl TransferEventImpl<
        TContractState, +HasComponent<TContractState>, +Drop<TContractState>,
    > of TransferEventTrait<TContractState> {
        fn emit_send_event(
            ref self: ComponentState<TContractState>,
            sender: ContractAddress,
            receiver: ByteArray,
            denom: PrefixedDenom,
            amount: u256,
            memo: Memo,
        ) {
            self.emit(SendEvent { sender, receiver, denom, amount, memo });
        }

        fn emit_recv_event(
            ref self: ComponentState<TContractState>, packet_data: PacketData, success: bool,
        ) {
            self
                .emit(
                    RecvEvent {
                        sender: packet_data.sender.external().unwrap(),
                        receiver: packet_data.receiver.native().unwrap(),
                        denom: packet_data.denom,
                        amount: packet_data.amount,
                        memo: packet_data.memo,
                        success,
                    },
                );
        }

        fn emit_ack_event(
            ref self: ComponentState<TContractState>, packet_data: PacketData, ack: Acknowledgement,
        ) {
            self
                .emit(
                    AckEvent {
                        sender: packet_data
                            .sender
                            .native()
                            .unwrap(), // NOTE: Safe to unwrap, since it's checked in the validation.
                        receiver: packet_data
                            .receiver
                            .external()
                            .unwrap(), // NOTE: Safe to unwrap, since it's checked in the validation.
                        denom: packet_data.denom,
                        amount: packet_data.amount,
                        memo: packet_data.memo,
                        ack,
                    },
                );
        }

        fn emit_ack_status_event(ref self: ComponentState<TContractState>, ack_status: AckStatus) {
            self.emit(AckStatusEvent { ack_status });
        }

        fn emit_timeout_event(ref self: ComponentState<TContractState>, packet_data: PacketData) {
            self
                .emit(
                    TimeoutEvent {
                        receiver: packet_data
                            .receiver
                            .external()
                            .unwrap(), // NOTE: Safe to unwrap, since it's checked in the validation.
                        denom: packet_data.denom,
                        amount: packet_data.amount,
                        memo: packet_data.memo,
                    },
                );
        }

        fn emit_create_token_event(
            ref self: ComponentState<TContractState>,
            name: ByteArray,
            symbol: ByteArray,
            decimals: u8,
            address: ContractAddress,
        ) {
            let event = CreateTokenEvent { name, symbol, decimals, address };
            self.emit(event);
        }
    }
}

