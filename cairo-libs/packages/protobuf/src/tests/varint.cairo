use alexandria_math::pow;
use protobuf::varint::{
    encode_varint_to_byte_array, decode_varint_from_byte_array, encode_varint_to_u8_array
};
use protobuf::hex::decode as hex_decode;

fn assert_encode_varint(value: u32, expected: Array<u8>) {
    assert_eq!(encode_varint_to_u8_array(value), expected);
}

#[test]
fn test_encode_varint() {
    assert_encode_varint(pow(2, 0) - 1, array![0x00]);
    assert_encode_varint(pow(2, 0), array![0x01]); // 1
    assert_encode_varint(pow(2, 7) - 1, array![0x7F]); // 127
    assert_encode_varint(pow(2, 7), array![0x80, 0x01]); // 128
    assert_encode_varint(pow(2, 14) - 1, array![0xFF, 0x7F]); // [255, 127]
    assert_encode_varint(pow(2, 14), array![0x80, 0x80, 0x01]); // [128, 128, 1]
    assert_encode_varint(pow(2, 21) - 1, array![0xFF, 0xFF, 0x7F]); // [255, 255, 127]
    assert_encode_varint(pow(2, 21), array![0x80, 0x80, 0x80, 0x01]); // [128, 128, 128, 1]
    assert_encode_varint(pow(2, 28) - 1, array![0xFF, 0xFF, 0xFF, 0x7F]); // [255, 255, 255, 127]
    assert_encode_varint(
        pow(2, 28), array![0x80, 0x80, 0x80, 0x80, 0x01]
    ); // [128, 128, 128, 128, 1]
    assert_encode_varint(
        0xffffffff, array![0xFF, 0xFF, 0xFF, 0xFF, 0x0F]
    ); // [255, 255, 255, 255, 15]
}

#[test]
fn test_encode_varint_u64_default() {
    assert_eq!(encode_varint_to_byte_array(0), "\x00");
    let mut index = 0;
    assert_eq!(decode_varint_from_byte_array(@"\x00", ref index), 0);
}

#[test]
fn test_encode_decode_varint_u64() {
    let value = 0x1234567890ABCDEF;
    let bytes = encode_varint_to_byte_array(value);
    let hex = "ef9baf8589cf959a12";
    let bytes2 = hex_decode(@hex);
    assert_eq!(bytes, bytes2, "invalid encoded bytes");
    let mut index = 0;
    let decoded = decode_varint_from_byte_array(@bytes, ref index);
    assert_eq!(decoded, value, "invalid decoded value");
}

