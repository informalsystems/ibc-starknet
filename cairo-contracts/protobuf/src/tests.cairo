use protobuf::types::message::{ProtoMessage, ProtoCodecImpl};
use protobuf::types::tag::WireType;
use protobuf::primitives::array::{ByteArrayAsProtoMessage, ArrayAsProtoMessage};
use protobuf::primitives::numeric::{BoolAsProtoMessage, NumberAsProtoMessage, I64AsProtoMessage};
use protobuf::hex::decode as hex_decode;

#[derive(Default, Debug, Clone, Drop, PartialEq, Serde)]
struct Proposer {
    address: ByteArray,
    pub_key: ByteArray,
}

impl ProposerAsProtoMessage of ProtoMessage<Proposer> {
    fn encode_raw(self: @Proposer, ref output: ByteArray) {
        ProtoCodecImpl::encode_length_delimited_raw(1, self.address, ref output);
        ProtoCodecImpl::encode_length_delimited_raw(2, self.pub_key, ref output);
    }

    fn decode_raw(ref value: Proposer, serialized: @ByteArray, ref index: usize, length: usize) {
        let bound = index + length;

        ProtoCodecImpl::decode_length_delimited_raw(
            1, ref value.address, serialized, ref index, bound
        );
        ProtoCodecImpl::decode_length_delimited_raw(
            2, ref value.pub_key, serialized, ref index, bound
        );

        assert(index == bound, 'invalid length for Proposer');
    }

    fn wire_type() -> WireType {
        WireType::LengthDelimited
    }

    fn type_url() -> ByteArray {
        "test/dummy.Proposer"
    }
}


#[derive(Default, Debug, Copy, Drop, PartialEq, Serde)]
enum ValidatorType {
    #[default]
    Full,
    Light,
}

impl ValidatorTypeIntoU64 of Into<ValidatorType, u64> {
    fn into(self: ValidatorType) -> u64 {
        match self {
            ValidatorType::Full => 0,
            ValidatorType::Light => 1,
        }
    }
}

impl U64IntoValidatorType of Into<u64, ValidatorType> {
    fn into(self: u64) -> ValidatorType {
        match self {
            0 => ValidatorType::Full,
            1 => ValidatorType::Light,
            _ => panic!("invalid ValidatorType"),
        }
    }
}


#[derive(Default, Debug, Clone, Drop, PartialEq, Serde)]
struct TmHeader {
    height: i64,
    active: bool,
    chain_id: ByteArray,
    time: u64,
    hash: Array<u8>,
    indexes: Array<u64>,
    proposer: Proposer,
    validator_type: ValidatorType,
}

impl TmHeaderAsProtoMessage of ProtoMessage<TmHeader> {
    fn encode_raw(self: @TmHeader, ref output: ByteArray) {
        ProtoCodecImpl::encode_length_delimited_raw(1, self.height, ref output);
        ProtoCodecImpl::encode_length_delimited_raw(2, self.active, ref output);
        ProtoCodecImpl::encode_length_delimited_raw(3, self.chain_id, ref output);
        ProtoCodecImpl::encode_length_delimited_raw(4, self.time, ref output);
        ProtoCodecImpl::encode_length_delimited_raw(5, self.hash, ref output);
        ProtoCodecImpl::encode_length_delimited_raw(6, self.indexes, ref output);
        ProtoCodecImpl::encode_length_delimited_raw(7, self.proposer, ref output);
        ProtoCodecImpl::encode_length_delimited_raw(8, self.validator_type, ref output);
    }

    fn decode_raw(ref value: TmHeader, serialized: @ByteArray, ref index: usize, length: usize) {
        let bound = index + length;

        ProtoCodecImpl::decode_length_delimited_raw(
            1, ref value.height, serialized, ref index, bound
        );
        ProtoCodecImpl::decode_length_delimited_raw(
            2, ref value.active, serialized, ref index, bound
        );
        ProtoCodecImpl::decode_length_delimited_raw(
            3, ref value.chain_id, serialized, ref index, bound
        );
        ProtoCodecImpl::decode_length_delimited_raw(
            4, ref value.time, serialized, ref index, bound
        );
        ProtoCodecImpl::decode_length_delimited_raw(
            5, ref value.hash, serialized, ref index, bound
        );
        ProtoCodecImpl::decode_length_delimited_raw(
            6, ref value.indexes, serialized, ref index, bound
        );
        ProtoCodecImpl::decode_length_delimited_raw(
            7, ref value.proposer, serialized, ref index, bound
        );
        ProtoCodecImpl::decode_length_delimited_raw(
            8, ref value.validator_type, serialized, ref index, bound
        );

        assert(index == bound, 'invalid length for TmHeader');
    }

    fn wire_type() -> WireType {
        WireType::LengthDelimited
    }

    fn type_url() -> ByteArray {
        "test/dummy.TmHeader"
    }
}

#[test]
fn test_proto_u64() {
    let hex = "d295fcd8ceb1aaaaab01";
    let bytes = hex_decode(@hex);
    let num = ProtoCodecImpl::decode::<u64>(@bytes);
    assert_eq!(num, 0xab54a98ceb1f0ad2, "number decode failed");
    let bytes2 = ProtoCodecImpl::encode(@num);
    assert_eq!(bytes, bytes2, "num encode failed");
}

#[test]
fn test_proto_byte_array() {
    let hex = "48656C6C6F2C20576F726C6421";
    let bytes = hex_decode(@hex);
    let byte_array = ProtoCodecImpl::decode::<ByteArray>(@bytes);
    assert_eq!(byte_array, "Hello, World!", "byte array decode failed");
    let bytes2 = ProtoCodecImpl::encode(@byte_array);
    assert_eq!(bytes, bytes2, "byte array encode failed");
}

#[test]
fn test_proto_to_cairo_struct() {
    let hex =
        "08f6ffffffffffffffff0110011a0b636f736d6f736875622d342080ccb9ff052a0412345678320af8acd19101f0bdf3d5093a260a12636f736d6f733168616670746d347a7879361210636f736d6f7376616c707562313233344001";
    let bytes = hex_decode(@hex);
    let header = ProtoCodecImpl::decode::<TmHeader>(@bytes);
    let header2 = TmHeader {
        height: -10,
        active: true,
        chain_id: "cosmoshub-4",
        time: 1609459200,
        hash: array![0x12, 0x34, 0x56, 0x78],
        indexes: array![0x12345678, 0x9abcdef0],
        proposer: Proposer { address: "cosmos1hafptm4zxy6", pub_key: "cosmosvalpub1234", },
        validator_type: ValidatorType::Light,
    };
    assert_eq!(header2, header, "tm header decode failed");
    let bytes2 = ProtoCodecImpl::encode(@header);
    assert_eq!(bytes, bytes2, "tm header encode failed");
}

#[test]
fn test_proto_to_cairo_struct_absent_field() {
    let hex = "08f6ffffffffffffffff012080ccb9ff05";
    let bytes = hex_decode(@hex);
    let header = ProtoCodecImpl::decode::<TmHeader>(@bytes);
    let header2 = TmHeader {
        height: -10,
        active: false,
        chain_id: "",
        time: 1609459200,
        hash: array![],
        indexes: array![],
        proposer: Proposer { address: "", pub_key: "", },
        validator_type: ValidatorType::Full,
    };
    assert_eq!(header2, header, "tmh decode wo field failed");
    let bytes2 = ProtoCodecImpl::encode(@header);
    assert_eq!(bytes, bytes2, "tmh encode wo field failed");
}

#[test]
#[should_panic]
fn test_proto_to_cairo_struct_non_canonical_order() {
    let hex = "2080ccb9ff0508f6ffffffffffffffff01";
    let bytes = hex_decode(@hex);
    ProtoCodecImpl::decode::<TmHeader>(@bytes);
}

#[test]
fn test_proto_to_any() {
    let header = TmHeader {
        height: -10,
        active: false,
        chain_id: "",
        time: 1609459200,
        hash: array![],
        indexes: array![],
        proposer: Proposer { address: "", pub_key: "", },
        validator_type: ValidatorType::Full,
    };
    let any = ProtoCodecImpl::to_any(@header);
    let header2 = ProtoCodecImpl::from_any::<TmHeader>(@any);
    assert_eq!(header2, header, "any conversion failed");
}
