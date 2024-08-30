use core::array::ArrayTrait;
use core::hash::{HashStateTrait, HashStateExTrait};
use core::num::traits::Zero;
use core::poseidon::PoseidonTrait;
use core::poseidon::poseidon_hash_span;
use starknet::ContractAddress;
use starknet::contract_address_const;
use starknet_ibc_apps::transfer::{
    ERC20Contract, ERC20ContractTrait, TransferErrors, TRANSFER_PORT_ID_HASH
};
use starknet_ibc_core::client::{Height, Timestamp};
use starknet_ibc_core::host::{PortId, PortIdTrait, ChannelId, ChannelIdTrait};
use starknet_ibc_utils::{ValidateBasicTrait, ComputeKeyTrait};

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
    pub sender: Participant,
    pub receiver: Participant,
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
    fn as_byte_array(self: @PrefixedDenom) -> ByteArray;
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

    fn as_byte_array(self: @PrefixedDenom) -> ByteArray {
        let mut denom_prefix: ByteArray = "";
        let mut trace_path_span = self.trace_path.span();
        while let Option::Some(path) = trace_path_span.pop_front() {
            denom_prefix.append(path.port_id.port_id);
            denom_prefix.append(@"/");
            denom_prefix.append(path.channel_id.channel_id);
            denom_prefix.append(@"/");
        };
        denom_prefix.append(@self.base.hosted().unwrap());
        denom_prefix
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
    Native: ERC20Contract,
    Hosted: ByteArray,
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
        Denom::Native(ERC20Contract { address: self })
    }
}

/// Represents a participant either sending or receiving a packet.
#[derive(Clone, Debug, Drop, Serde)]
pub enum Participant {
    Native: ContractAddress,
    External: Array<felt252>,
}

pub trait ParticipantTrait {
    fn is_non_zero(self: @Participant) -> bool;
}

impl ParticipantImpl of ParticipantTrait {
    fn is_non_zero(self: @Participant) -> bool {
        match self {
            Participant::Native(contract_address) => contract_address.is_non_zero(),
            Participant::External(array) => {
                match array.len() {
                    0 => false,
                    1 => array[0].is_non_zero(),
                    _ => true,
                }
            }
        }
    }
}

impl ParticipantTryIntoContractAddress of TryInto<Participant, ContractAddress> {
    fn try_into(self: Participant) -> Option<ContractAddress> {
        match self {
            Participant::Native(contract_address) => Option::Some(contract_address),
            _ => Option::None,
        }
    }
}

impl ContractAddressIntoParticipant of Into<ContractAddress, Participant> {
    fn into(self: ContractAddress) -> Participant {
        Participant::Native(self)
    }
}

impl ArrayFelt252IntoParticipant of Into<Array<felt252>, Participant> {
    fn into(self: Array<felt252>) -> Participant {
        Participant::External(self)
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

