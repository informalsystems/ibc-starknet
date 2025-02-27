use core::array::ArrayTrait;
use core::fmt::{Display, Error, Formatter};
use core::num::traits::Zero;
use serde_json::{Serialize, SerializerTrait};
use starknet::ContractAddress;
use starknet_ibc_apps::transfer::{ERC20Contract, TRANSFER_PORT_ID_HASH, TransferErrors};
use starknet_ibc_core::client::{Height, Timestamp};
use starknet_ibc_core::host::{ChannelId, ChannelIdTrait, PortId, PortIdTrait};
use starknet_ibc_utils::{ComputeKey, LocalKeyBuilderImpl, ValidateBasic};

/// Maximum memo length allowed for ICS-20 transfers. This bound corresponds to
/// the `MaximumMemoLength` in the `ibc-go`.
pub(crate) const MAXIMUM_MEMO_LENGTH: u32 = 32768;

/// Message used to build an ICS20 token transfer packet.
#[derive(Clone, Debug, Drop, Serde)]
pub struct MsgTransfer {
    pub port_id_on_a: PortId,
    pub chan_id_on_a: ChannelId,
    pub denom: PrefixedDenom,
    pub amount: u256,
    pub receiver: ByteArray,
    pub memo: Memo,
    pub timeout_height_on_b: Height,
    pub timeout_timestamp_on_b: Timestamp,
}

impl MsgTransferValidateBasic of ValidateBasic<MsgTransfer> {
    fn validate_basic(self: @MsgTransfer) {
        self.port_id_on_a.validate(TRANSFER_PORT_ID_HASH);
        self.chan_id_on_a.validate();
        assert(self.denom.base.is_non_zero(), TransferErrors::INVALID_DENOM);
        assert(self.receiver.len() > 0, TransferErrors::INVALID_RECEIVER);
        assert(self.amount.is_non_zero(), TransferErrors::ZERO_AMOUNT);
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

pub impl PacketDataJsonSerialize of Serialize<PacketData> {
    fn serialize<S, +Drop<S>, +SerializerTrait<S>>(self: PacketData, ref serializer: S) {
        serializer.serialize_field("denom", format!("{}", self.denom));
        serializer.serialize_field("amount", format!("{}", self.amount));
        serializer.serialize_field("sender", format!("{}", self.sender));
        serializer.serialize_field("receiver", format!("{}", self.receiver));
        serializer.serialize_field("memo", format!("{}", self.memo));
        serializer.end();
    }
}

pub impl ArrayFelt252IntoPacketData of Into<Array<felt252>, PacketData> {
    fn into(self: Array<felt252>) -> PacketData {
        let mut packet_data_span = self.span();

        let maybe_packet_data: Option<PacketData> = Serde::deserialize(ref packet_data_span);

        match maybe_packet_data {
            Option::Some(packet_data) => packet_data,
            Option::None => panic!("{}", TransferErrors::INVALID_PACKET_DATA),
        }
    }
}

impl PacketDataValidateBasic of ValidateBasic<PacketData> {
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

#[generate_trait]
pub impl PrefixedDenomImpl of PrefixedDenomTrait {
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

    /// Returns the outermost (first) prefix relevant to this chain (Starknet).
    fn first_prefix(self: @PrefixedDenom) -> Option<@TracePrefix> {
        let len = self.trace_path.len();
        if len.is_zero() {
            Option::None
        } else {
            Option::Some(self.trace_path.at(len - 1))
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
        denom_prefix.append(@format!("{}", self.base));
        denom_prefix
    }
}

impl PrefixedDenomDisplay of Display<PrefixedDenom> {
    fn fmt(self: @PrefixedDenom, ref f: Formatter) -> Result<(), Error> {
        Display::fmt(@self.as_byte_array(), ref f)
    }
}

impl PrefixedDenomKeyImpl of ComputeKey<PrefixedDenom> {
    fn key(self: @PrefixedDenom) -> felt252 {
        let mut key_builder = LocalKeyBuilderImpl::init();
        // Note: why we didn't use `key_builder.append_serde(self)` here?
        //
        // We wanted to get the same hash for a standalone `base`
        // and a `PrefixedDenom` the same `base` with empty `TracePath`.
        //
        // So, we pass serialization of each `TracePrefix` in `TracePath`
        // separately and then the serialization of `base`.
        //
        // `key_builder.append_serde(self)` would have included the serialization
        // of empty array which contains the length (zero).
        // So, the hash would  have been different.
        let mut trace_path_span = self.trace_path.span();
        while let Option::Some(path) = trace_path_span.pop_front() {
            key_builder.append_serde(path);
        };
        key_builder.append_serde(self.base);
        key_builder.key()
    }
}

#[derive(Clone, Debug, Drop, PartialEq, Serde)]
pub struct TracePrefix {
    pub port_id: PortId,
    pub channel_id: ChannelId,
}

#[generate_trait]
pub impl TracePrefixImpl of TracePrefixTrait {
    fn new(port_id: PortId, channel_id: ChannelId) -> TracePrefix {
        TracePrefix { port_id: port_id, channel_id: channel_id }
    }
}

#[derive(Clone, Debug, Drop, Serde)]
pub enum Denom {
    Native: ERC20Contract,
    Hosted: ByteArray,
}

#[generate_trait]
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
            Denom::Hosted(base) => Option::Some(base.clone()),
        }
    }
}

pub impl ContractAddressIntoDenom of Into<ContractAddress, Denom> {
    fn into(self: ContractAddress) -> Denom {
        Denom::Native(ERC20Contract { address: self })
    }
}

pub impl DenomDisplay of Display<Denom> {
    fn fmt(self: @Denom, ref f: Formatter) -> Result<(), Error> {
        match self {
            Denom::Native(contract) => {
                Display::fmt(@format!("0x{:x}", *contract.address), ref f)
            },
            Denom::Hosted(byte_array) => Display::fmt(@byte_array, ref f),
        }
    }
}

/// Represents a participant either sending or receiving a packet.
#[derive(Clone, Debug, Drop, Serde)]
pub enum Participant {
    Native: ContractAddress,
    External: ByteArray,
}

#[generate_trait]
pub impl ParticipantImpl of ParticipantTrait {
    fn native(self: @Participant) -> Option<ContractAddress> {
        if let Participant::Native(contract_address) = self {
            Option::Some(*contract_address)
        } else {
            Option::None
        }
    }

    fn external(self: @Participant) -> Option<ByteArray> {
        if let Participant::External(byte_array) = self {
            Option::Some(byte_array.clone())
        } else {
            Option::None
        }
    }

    fn is_non_zero(self: @Participant) -> bool {
        match self {
            Participant::Native(contract_address) => contract_address.is_non_zero(),
            Participant::External(byte_array) => byte_array.len() > 0,
        }
    }

    fn as_byte_array(self: @Participant) -> ByteArray {
        match self {
            Participant::Native(contract_address) => { format!("0x{:x}", *contract_address) },
            Participant::External(byte_array) => byte_array.clone(),
        }
    }
}

pub impl ParticipantDisplay of Display<Participant> {
    fn fmt(self: @Participant, ref f: Formatter) -> Result<(), Error> {
        Display::fmt(@self.as_byte_array(), ref f)
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

impl ByteArrayIntoParticipant of Into<ByteArray, Participant> {
    fn into(self: ByteArray) -> Participant {
        Participant::External(self)
    }
}

#[derive(Clone, Debug, Drop, Serde)]
pub struct Memo {
    pub memo: ByteArray,
}

pub impl MemoDisplay of Display<Memo> {
    fn fmt(self: @Memo, ref f: Formatter) -> Result<(), Error> {
        Display::fmt(self.memo, ref f)
    }
}

impl MemoValidateBasic of ValidateBasic<Memo> {
    fn validate_basic(self: @Memo) {
        assert(self.memo.len() <= MAXIMUM_MEMO_LENGTH, TransferErrors::MAXIMUM_MEMO_LENGTH);
    }
}

#[cfg(test)]
pub mod tests {
    use serde_json::to_byte_array;
    use starknet_ibc_testkit::dummies::{ERC20, PACKET_DATA_FROM_SN};
    use starknet_ibc_utils::{ComputeKey, LocalKeyBuilderImpl, LocalKeyBuilderTrait};

    // Snapshot test to ensure serialization stays consistent.
    #[test]
    fn test_json_serialized_packet_data() {
        let json = to_byte_array(PACKET_DATA_FROM_SN(ERC20()));
        let expected: ByteArray =
            "{\"denom\":\"0x49d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7\",\"amount\":\"100\",\"sender\":\"0x55534552\",\"receiver\":\"cosmos1wxeyh7zgn4tctjzs0vtqpc6p5cxq5t2muzl7ng\",\"memo\":\"\"}";
        assert_eq!(json, expected);
    }

    #[test]
    fn test_prefixed_denom_key() {
        let prefixed_denom = PACKET_DATA_FROM_SN(ERC20()).denom;
        let prefixed_key = prefixed_denom.key();
        let base_key = {
            let mut hasher = LocalKeyBuilderImpl::init();
            hasher.append_serde(@prefixed_denom.base);
            hasher.key()
        };
        let expected: felt252 = 0x2e74acb5f5dfbbc9cddcd69d5ee307713735fe038880606804515cf078fc1ee;
        assert_eq!(prefixed_key, expected);
        assert_eq!(base_key, expected);
    }
}
