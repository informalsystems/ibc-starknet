use core::array::ArrayTrait;
use core::byte_array::ByteArrayTrait;
use core::num::traits::zero::Zero;
use core::serde::Serde;
use core::to_byte_array::FormatAsByteArray;
use core::traits::TryInto;
use openzeppelin::utils::selectors;
use starknet::ContractAddress;
use starknet::Store;
use starknet::contract_address_const;
use starknet::syscalls::call_contract_syscall;
use starknet_ibc::core::types::{PortId, ChannelId, Sequence, Height, Timestamp};

/// Maximum memo length allowed for ICS-20 transfers. This bound corresponds to
/// the `MaximumMemoLength` in the `ibc-go`.
pub(crate) const MAXIMUM_MEMO_LENGTH: u32 = 32768;

#[derive(Clone, Debug, Drop, Serde, Store)]
pub struct MsgTransfer {
    pub port_id_on_a: PortId,
    pub chan_id_on_a: ChannelId,
    pub packet_data: PacketData,
}

#[derive(Clone, Debug, Drop, Serde, Store)]
pub struct Packet {
    pub seq_on_a: Sequence,
    pub port_id_on_a: PortId,
    pub chan_id_on_a: ChannelId,
    pub port_id_on_b: PortId,
    pub chan_id_on_b: ChannelId,
    pub data: PacketData,
    pub timeout_height_on_b: Height,
    pub timeout_timestamp_on_b: Timestamp,
}

#[derive(Clone, Debug, Drop, Serde, Store)]
pub struct PacketData {
    pub token: Token,
    pub sender: ContractAddress,
    pub receiver: ContractAddress,
    pub memo: Memo,
}

#[derive(Clone, Debug, Drop, Serde, Store)]
pub struct Token {
    pub denom: Denom,
    pub amount: u256,
}

#[derive(Clone, Debug, Drop, Serde, Store)]
pub enum Denom {
    Native: ContractAddress,
    IBC: ByteArray,
}

pub trait DenomTrait {
    fn is_zero(self: @Denom) -> bool;
    fn native(self: @Denom) -> Option<ContractAddress>;
    fn ibc(self: @Denom) -> Option<felt252>;
}

pub impl DenomImpl of DenomTrait {
    fn is_zero(self: @Denom) -> bool {
        match self {
            Denom::Native(contract_address) => contract_address.is_zero(),
            Denom::IBC(byte_array) => byte_array.len() == 0,
        }
    }

    fn native(self: @Denom) -> Option<ContractAddress> {
        match self {
            Denom::Native(contract_address) => Option::Some(*contract_address),
            Denom::IBC(_) => Option::None,
        }
    }

    fn ibc(self: @Denom) -> Option<felt252> {
        match self {
            Denom::Native(_) => Option::None,
            Denom::IBC(byte_array) => {
                // FIXME: This is not the correct way to convert a byte array to a felt252.
                let mut serialized_denom: Array<felt252> = array![];
                byte_array.serialize(ref serialized_denom);
                let mut denom_span = serialized_denom.span();
                Serde::<felt252>::deserialize(ref denom_span)
            }
        }
    }
}

pub impl ContractAddressIntoDenom of Into<ContractAddress, Denom> {
    fn into(self: ContractAddress) -> Denom {
        Denom::Native(self)
    }
}

#[derive(Clone, Debug, Drop, Serde, Store)]
pub struct Memo {
    pub memo: ByteArray,
}
