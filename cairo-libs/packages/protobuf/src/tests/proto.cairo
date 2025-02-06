use protobuf::types::message::{
    ProtoMessage, ProtoName, ProtoCodecImpl, EncodeContext, DecodeContext, EncodeContextImpl,
    DecodeContextImpl,
};
use protobuf::types::wkt::Any;
use protobuf::types::tag::WireType;
use protobuf::primitives::array::{ByteArrayAsProtoMessage, ArrayAsProtoMessage};
use protobuf::primitives::numeric::{BoolAsProtoMessage, I64AsProtoMessage};
use protobuf::hex::decode as hex_decode;
use protobuf::base64::decode as base64_decode;

#[derive(Default, Debug, Clone, Drop, PartialEq, Serde)]
struct Proposer {
    address: ByteArray,
    pub_key: ByteArray,
}

impl ProposerAsProtoMessage of ProtoMessage<Proposer> {
    fn encode_raw(self: @Proposer, ref context: EncodeContext) {
        context.encode_field(1, self.address);
        context.encode_field(2, self.pub_key);
    }

    fn decode_raw(ref self: Proposer, ref context: DecodeContext) {
        context.decode_field(1, ref self.address);
        context.decode_field(2, ref self.pub_key);
    }

    fn wire_type() -> WireType {
        WireType::LengthDelimited
    }
}

impl ProposerAsProtoName of ProtoName<Proposer> {
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
    proposers: Array<Proposer>,
}

impl TmHeaderAsProtoMessage of ProtoMessage<TmHeader> {
    fn encode_raw(self: @TmHeader, ref context: EncodeContext) {
        context.encode_field(1, self.height);
        context.encode_field(2, self.active);
        context.encode_field(3, self.chain_id);
        context.encode_field(4, self.time);
        context.encode_field(5, self.hash);
        context.encode_field(6, self.indexes);
        context.encode_field(7, self.proposer);
        context.encode_field(8, self.validator_type);
        context.encode_repeated_field(9, self.proposers);
    }

    fn decode_raw(ref self: TmHeader, ref context: DecodeContext) {
        context.decode_field(1, ref self.height);
        context.decode_field(2, ref self.active);
        context.decode_field(3, ref self.chain_id);
        context.decode_field(4, ref self.time);
        context.decode_field(5, ref self.hash);
        context.decode_field(6, ref self.indexes);
        context.decode_field(7, ref self.proposer);
        context.decode_field(8, ref self.validator_type);
        context.decode_repeated_field(9, ref self.proposers);
    }

    fn wire_type() -> WireType {
        WireType::LengthDelimited
    }
}

pub impl TmHeaderAsProtoName of ProtoName<TmHeader> {
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
    let base64 =
        "CPb//////////wEQARoLY29zbW9zaHViLTQggMy5/wUqBBI0VngyCvis0ZEB8L3z1Qk6JgoSY29zbW9zMWhhZnB0bTR6eHk2EhBjb3Ntb3N2YWxwdWIxMjM0QAE=";
    let bytes = base64_decode(@base64);
    let header = ProtoCodecImpl::decode::<TmHeader>(@bytes);
    let header2 = TmHeader {
        height: -10,
        active: true,
        chain_id: "cosmoshub-4",
        time: 0x5fee6600,
        hash: array![0x12, 0x34, 0x56, 0x78],
        indexes: array![0x12345678, 0x9abcdef0],
        proposer: Proposer { address: "cosmos1hafptm4zxy6", pub_key: "cosmosvalpub1234", },
        validator_type: ValidatorType::Light,
        proposers: array![],
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
        time: 0x5fee6600,
        hash: array![],
        indexes: array![],
        proposer: Proposer { address: "", pub_key: "", },
        validator_type: ValidatorType::Full,
        proposers: array![],
    };
    assert_eq!(header2, header, "tmh decode wo field failed");
    let bytes2 = ProtoCodecImpl::encode(@header);
    assert_eq!(bytes, bytes2, "tmh encode wo field failed");
}

#[should_panic]
#[test]
fn test_proto_to_cairo_struct_non_canonical_order() {
    let hex = "2080ccb9ff0508f6ffffffffffffffff01";
    let bytes = hex_decode(@hex);
    ProtoCodecImpl::decode::<TmHeader>(@bytes);
}

#[test]
fn test_repeated_default_value() {
    let base64 =
        "IIDMuf8FKgcSADQAVgB4SiYKEmNvc21vczFoYWZwdG00enh5NhIQY29zbW9zdmFscHViMTIzNEoASiYKEmNvc21vczFoYWZwdG00enh5NhIQY29zbW9zdmFscHViMTIzNA==";
    let bytes = base64_decode(@base64);
    let header = ProtoCodecImpl::decode::<TmHeader>(@bytes);
    let header2 = TmHeader {
        height: 0,
        active: false,
        chain_id: "",
        time: 0x5fee6600,
        hash: array![0x12, 0x00, 0x34, 0x00, 0x56, 0x00, 0x78],
        indexes: array![],
        proposer: Proposer { address: "", pub_key: "", },
        validator_type: ValidatorType::Full,
        proposers: array![
            Proposer { address: "cosmos1hafptm4zxy6", pub_key: "cosmosvalpub1234", },
            Default::<Proposer>::default(),
            Proposer { address: "cosmos1hafptm4zxy6", pub_key: "cosmosvalpub1234", },
        ],
    };
    assert_eq!(header2, header, "repeated default value failed");
    let bytes2 = ProtoCodecImpl::encode(@header);
    assert_eq!(bytes, bytes2, "repeated default value failed");
}

#[test]
fn test_proto_to_any() {
    let header = TmHeader {
        height: -10,
        active: false,
        chain_id: "",
        time: 0x5fee6600,
        hash: array![],
        indexes: array![],
        proposer: Proposer { address: "abc", pub_key: "def", },
        validator_type: ValidatorType::Full,
        proposers: array![
            Proposer { address: "abc", pub_key: "def", },
            Proposer { address: "pqr", pub_key: "stu", }
        ],
    };
    let any: Any = header.clone().into();
    let header2: TmHeader = any.try_into().unwrap();
    assert_eq!(header2, header, "any conversion failed");
}
