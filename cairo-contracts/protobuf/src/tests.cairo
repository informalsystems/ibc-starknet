use protobuf::types::message::{ProtoMessage, ProtoCodecImpl};
use protobuf::types::tag::WireType;
use protobuf::primitives::array::{ByteArrayAsProtoMessage, ArrayAsProtoMessage};
use protobuf::primitives::numeric::{BoolAsProtoMessage, NumberAsProtoMessage, I64AsProtoMessage};

#[derive(Default, Debug, Drop, PartialEq, Serde)]
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

        ProtoCodecImpl::decode_length_delimited_raw(1, ref value.address, serialized, ref index);
        ProtoCodecImpl::decode_length_delimited_raw(2, ref value.pub_key, serialized, ref index);

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


#[derive(Default, Debug, Drop, PartialEq, Serde)]
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

        ProtoCodecImpl::decode_length_delimited_raw(1, ref value.height, serialized, ref index);
        ProtoCodecImpl::decode_length_delimited_raw(2, ref value.active, serialized, ref index);
        ProtoCodecImpl::decode_length_delimited_raw(3, ref value.chain_id, serialized, ref index);
        ProtoCodecImpl::decode_length_delimited_raw(4, ref value.time, serialized, ref index);
        ProtoCodecImpl::decode_length_delimited_raw(5, ref value.hash, serialized, ref index);
        ProtoCodecImpl::decode_length_delimited_raw(6, ref value.indexes, serialized, ref index);
        ProtoCodecImpl::decode_length_delimited_raw(7, ref value.proposer, serialized, ref index);
        ProtoCodecImpl::decode_length_delimited_raw(
            8, ref value.validator_type, serialized, ref index
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
    let bytes = array![210, 149, 252, 216, 206, 177, 170, 170, 171, 1];
    let mut bytes_array = "";
    while bytes_array.len() < bytes.len() {
        bytes_array.append_byte(*bytes[bytes_array.len()]);
    };
    let num = ProtoCodecImpl::decode::<u64>(@bytes_array);
    assert_eq!(num, 0xab54a98ceb1f0ad2, "number decode failed");
    let bytes = ProtoCodecImpl::encode(@num);
    assert_eq!(bytes_array, bytes, "num encode failed");
}

#[test]
fn test_proto_byte_array() {
    let bytes = array![
        0x48, 0x65, 0x6C, 0x6C, 0x6F, 0x2C, 0x20, 0x57, 0x6F, 0x72, 0x6C, 0x64, 0x21
    ];
    let mut bytes_array = "";
    while bytes_array.len() < bytes.len() {
        bytes_array.append_byte(*bytes[bytes_array.len()]);
    };
    let byte_array = ProtoCodecImpl::decode::<ByteArray>(@bytes_array);
    assert_eq!(byte_array, "Hello, World!", "byte array decode failed");
    let bytes = ProtoCodecImpl::encode(@byte_array);
    assert_eq!(bytes_array, bytes, "byte array encode failed");
}

#[test]
fn test_proto_array_u8() {
    let bytes = array![0x12, 0x34, 0x56, 0x78];
    let mut bytes_array = "";
    while bytes_array.len() < bytes.len() {
        bytes_array.append_byte(*bytes[bytes_array.len()]);
    };
    let array = ProtoCodecImpl::decode::<Array<u8>>(@bytes_array);
    assert_eq!(array, array![0x12, 0x34, 0x56, 0x78], "array decode failed");
    let bytes = ProtoCodecImpl::encode(@array);
    assert_eq!(bytes_array, bytes, "array encode failed");
}

#[test]
fn test_proto_array_u64() {
    let bytes = array![0xf8, 0xac, 0xd1, 0x91, 0x01, 0xf0, 0xbd, 0xf3, 0x0d5, 0x09];
    let mut bytes_array = "";
    while bytes_array.len() < bytes.len() {
        bytes_array.append_byte(*bytes[bytes_array.len()]);
    };
    let array = ProtoCodecImpl::decode::<Array<u64>>(@bytes_array);
    assert_eq!(array, array![0x12345678, 0x9abcdef0], "array decode failed");
    let bytes = ProtoCodecImpl::encode(@array);
    assert_eq!(bytes_array, bytes, "array encode failed");
}

#[test]
fn test_proto_to_cairo_struct() {
    let proto_bytes: Array<u8> = array![
        8,
        246,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        1,
        16,
        1,
        26,
        11,
        99,
        111,
        115,
        109,
        111,
        115,
        104,
        117,
        98,
        45,
        52,
        32,
        128,
        204,
        185,
        255,
        5,
        42,
        4,
        18,
        52,
        86,
        120,
        50,
        10,
        248,
        172,
        209,
        145,
        1,
        240,
        189,
        243,
        213,
        9,
        58,
        38,
        10,
        18,
        99,
        111,
        115,
        109,
        111,
        115,
        49,
        104,
        97,
        102,
        112,
        116,
        109,
        52,
        122,
        120,
        121,
        54,
        18,
        16,
        99,
        111,
        115,
        109,
        111,
        115,
        118,
        97,
        108,
        112,
        117,
        98,
        49,
        50,
        51,
        52,
        64,
        1
    ];
    let mut bytes_array = "";
    while bytes_array.len() < proto_bytes.len() {
        bytes_array.append_byte(*proto_bytes[bytes_array.len()]);
    };
    let header = ProtoCodecImpl::decode::<TmHeader>(@bytes_array);
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
    let bytes_array2 = ProtoCodecImpl::encode(@header);
    assert_eq!(bytes_array, bytes_array2, "tm header encode failed");
}

#[test]
fn test_proto_to_cairo_struct_absent_field() {
    let proto_bytes: Array<u8> = array![
        8, 246, 255, 255, 255, 255, 255, 255, 255, 255, 1, 32, 128, 204, 185, 255, 5
    ];
    let mut bytes_array = "";
    while bytes_array.len() < proto_bytes.len() {
        bytes_array.append_byte(*proto_bytes[bytes_array.len()]);
    };
    let header = ProtoCodecImpl::decode::<TmHeader>(@bytes_array);
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
    let bytes_array2 = ProtoCodecImpl::encode(@header);
    assert_eq!(bytes_array, bytes_array2, "tmh encode wo field failed");
}

#[test]
#[should_panic]
fn test_proto_to_cairo_struct_non_canonical_order() {
    let proto_bytes: Array<u8> = array![
        32, 128, 204, 185, 255, 5, 8, 246, 255, 255, 255, 255, 255, 255, 255, 255, 1,
    ];
    let mut bytes_array = "";
    while bytes_array.len() < proto_bytes.len() {
        bytes_array.append_byte(*proto_bytes[bytes_array.len()]);
    };
    ProtoCodecImpl::decode::<TmHeader>(@bytes_array);
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
