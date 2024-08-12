use core::array::ArrayTrait;
use core::hash::{HashStateTrait, HashStateExTrait};
use core::num::traits::Zero;
use core::poseidon::PoseidonTrait;
use core::poseidon::poseidon_hash_span;
use core::serde::Serde;
use core::starknet::SyscallResultTrait;
use openzeppelin::token::erc20::{ERC20ABIDispatcher, ERC20ABIDispatcherTrait};
use openzeppelin::utils::serde::SerializedAppend;
use starknet::ClassHash;
use starknet::ContractAddress;
use starknet::syscalls::deploy_syscall;
use starknet_ibc::apps::mintable::interface::{
    IERC20MintableDispatcher, IERC20MintableDispatcherTrait
};
use starknet_ibc::apps::transfer::TRANSFER_PORT_ID_HASH;
use starknet_ibc::apps::transfer::errors::TransferErrors;
use starknet_ibc::core::client::types::{Height, Timestamp};
use starknet_ibc::core::host::types::{PortId, PortIdTrait, ChannelId, ChannelIdTrait};
use starknet_ibc::utils::{ValidateBasicTrait, ComputeKeyTrait};

/// Maximum memo length allowed for ICS-20 transfers. This bound corresponds to
/// the `MaximumMemoLength` in the `ibc-go`.
pub(crate) const MAXIMUM_MEMO_LENGTH: u32 = 32768;

/// Message used to build an ICS20 token transfer packet.
#[derive(Clone, Debug, Drop, Serde)]
pub struct MsgTransfer {
    pub port_id_on_a: PortId,
    pub chan_id_on_a: ChannelId,
    pub packet_data: PacketData,
    pub timeout_height_on_b: Height,
    pub timeout_timestamp_on_b: Timestamp,
}

impl MsgTransferValidateBasicImpl of ValidateBasicTrait<MsgTransfer> {
    fn validate_basic(self: @MsgTransfer) {
        self.port_id_on_a.validate(TRANSFER_PORT_ID_HASH);
        self.chan_id_on_a.validate();
        self.packet_data.validate_basic();
    }
}

#[derive(Clone, Debug, Drop, Serde)]
pub struct PacketData {
    pub denom: PrefixedDenom,
    pub amount: u256,
    pub sender: ContractAddress,
    pub receiver: ContractAddress,
    pub memo: Memo,
}

impl PacketDataValidateBasicImpl of ValidateBasicTrait<PacketData> {
    fn validate_basic(self: @PacketData) {
        assert(self.sender.is_non_zero(), TransferErrors::INVALID_SENDER);
        assert(self.receiver.is_non_zero(), TransferErrors::INVALID_RECEIVER);
        assert(self.denom.base.is_non_zero(), TransferErrors::INVALID_DENOM);
        assert(self.amount.is_non_zero(), TransferErrors::ZERO_AMOUNT);
        self.memo.validate_basic();
    }
}

#[derive(Clone, Debug, Drop, Serde)]
pub struct PrefixedDenom {
    pub trace_path: Array<TracePrefix>,
    pub base: Denom,
}

pub trait PrefixedDenomTrait {
    fn starts_with(self: @PrefixedDenom, prefix: @TracePrefix) -> bool;
    fn add_prefix(ref self: PrefixedDenom, prefix: TracePrefix);
    fn remove_prefix(ref self: PrefixedDenom, prefix: @TracePrefix);
}

impl PrefixedDenomImpl of PrefixedDenomTrait {
    fn starts_with(self: @PrefixedDenom, prefix: @TracePrefix) -> bool {
        self.trace_path.at(0) == prefix
    }

    fn add_prefix(ref self: PrefixedDenom, prefix: TracePrefix) {
        self.trace_path.append(prefix);
    }

    fn remove_prefix(ref self: PrefixedDenom, prefix: @TracePrefix) {
        if self.starts_with(prefix) {
            self.trace_path.pop_front().unwrap();
        }
    }
}

impl PrefixedDenomKeyImpl of ComputeKeyTrait<PrefixedDenom> {
    fn compute_key(self: @PrefixedDenom) -> felt252 {
        let mut serialized_prefixed_denom: Array<felt252> = ArrayTrait::new();
        let mut trace_path_span = self.trace_path.span();
        while let Option::Some(path) = trace_path_span.pop_front() {
            Serde::serialize(path, ref serialized_prefixed_denom);
        };
        Serde::serialize(self.base, ref serialized_prefixed_denom);
        PoseidonTrait::new().update(poseidon_hash_span(serialized_prefixed_denom.span())).finalize()
    }
}

#[derive(Clone, Debug, Drop, PartialEq, Serde)]
pub struct TracePrefix {
    pub port_id: PortId,
    pub channel_id: ChannelId,
}

pub trait TracePrefixTrait {
    fn new(port_id: PortId, channel_id: ChannelId) -> TracePrefix;
}

impl TracePrefixImpl of TracePrefixTrait {
    fn new(port_id: PortId, channel_id: ChannelId) -> TracePrefix {
        TracePrefix { port_id: port_id, channel_id: channel_id, }
    }
}

#[derive(Clone, Debug, Drop, Serde)]
pub enum Denom {
    Native: ERC20Token,
    Hosted: ByteArray,
}

#[derive(Clone, Debug, Drop, Serde)]
pub struct ERC20Token {
    pub address: ContractAddress,
}

impl ContractAddressIntoTokenAddr of Into<ContractAddress, ERC20Token> {
    fn into(self: ContractAddress) -> ERC20Token {
        ERC20Token { address: self }
    }
}

impl ERC20TokenIntoFelt252 of Into<ERC20Token, felt252> {
    fn into(self: ERC20Token) -> felt252 {
        self.address.into()
    }
}

pub trait ERC20TokenTrait {
    fn is_non_zero(self: @ERC20Token) -> bool;
    fn create(
        class_hash: ClassHash,
        salt: felt252,
        name: ByteArray,
        symbol: ByteArray,
        amount: u256,
        recipient: ContractAddress,
        owner: ContractAddress
    ) -> ERC20Token;
    fn transfer(self: @ERC20Token, recipient: ContractAddress, amount: u256) -> bool;
    fn transfer_from(
        self: @ERC20Token, sender: ContractAddress, recipient: ContractAddress, amount: u256
    ) -> bool;
    fn mint(self: @ERC20Token, recipient: ContractAddress, amount: u256);
    fn burn(self: @ERC20Token, account: ContractAddress, amount: u256);
    fn balance_of(self: @ERC20Token, from_account: ContractAddress) -> u256;
}

impl ERC20TokenImpl of ERC20TokenTrait {
    fn is_non_zero(self: @ERC20Token) -> bool {
        self.address.is_non_zero()
    }

    fn create(
        class_hash: ClassHash,
        salt: felt252,
        name: ByteArray,
        symbol: ByteArray,
        amount: u256,
        recipient: ContractAddress,
        owner: ContractAddress
    ) -> ERC20Token {
        let mut call_data = array![];

        call_data.append_serde(name);
        call_data.append_serde(symbol);
        call_data.append_serde(amount);
        call_data.append_serde(recipient);
        call_data.append_serde(owner);

        let (address, _) = deploy_syscall(class_hash, salt, call_data.span(), false,)
            .unwrap_syscall();

        ERC20Token { address }
    }

    fn transfer(self: @ERC20Token, recipient: ContractAddress, amount: u256) -> bool {
        ERC20ABIDispatcher { contract_address: *self.address }.transfer(recipient, amount)
    }

    fn transfer_from(
        self: @ERC20Token, sender: ContractAddress, recipient: ContractAddress, amount: u256
    ) -> bool {
        ERC20ABIDispatcher { contract_address: *self.address }
            .transfer_from(sender, recipient, amount)
    }

    fn mint(self: @ERC20Token, recipient: ContractAddress, amount: u256) {
        IERC20MintableDispatcher { contract_address: *self.address }
            .permissioned_mint(recipient, amount)
    }

    fn burn(self: @ERC20Token, account: ContractAddress, amount: u256) {
        IERC20MintableDispatcher { contract_address: *self.address }
            .permissioned_burn(account, amount)
    }

    fn balance_of(self: @ERC20Token, from_account: ContractAddress) -> u256 {
        ERC20ABIDispatcher { contract_address: *self.address }.balance_of(from_account)
    }
}

pub trait DenomTrait {
    fn is_non_zero(self: @Denom) -> bool;
    fn native(self: @Denom) -> Option<ContractAddress>;
    fn hosted(self: @Denom) -> Option<ByteArray>;
}

pub impl DenomImpl of DenomTrait {
    fn is_non_zero(self: @Denom) -> bool {
        match self {
            Denom::Native(token_addr) => token_addr.is_non_zero(),
            Denom::Hosted(byte_array) => byte_array.len() != 0,
        }
    }

    fn native(self: @Denom) -> Option<ContractAddress> {
        match self {
            Denom::Native(contract_address) => Option::Some(*contract_address.address),
            Denom::Hosted(_) => Option::None,
        }
    }

    fn hosted(self: @Denom) -> Option<ByteArray> {
        match self {
            Denom::Native(_) => Option::None,
            Denom::Hosted(base) => Option::Some(base.clone())
        }
    }
}

pub impl ContractAddressIntoDenom of Into<ContractAddress, Denom> {
    fn into(self: ContractAddress) -> Denom {
        Denom::Native(ERC20Token { address: self })
    }
}

#[derive(Clone, Debug, Drop, Serde)]
pub struct Memo {
    pub memo: ByteArray,
}

impl MemoValidateBasicImpl of ValidateBasicTrait<Memo> {
    fn validate_basic(self: @Memo) {
        assert(self.memo.len() <= MAXIMUM_MEMO_LENGTH, TransferErrors::MAXIMUM_MEMO_LENGTH);
    }
}

