use crate::array::reverse_array;
use crate::bytes::SpanU8IntoByteArray;

pub fn u32_from_u8(val: [u8; 4]) -> u32 {
    let [b0, b1, b2, b3] = val;
    let p0 = b0.into() * 0x1_00_00_00;
    let p1 = b1.into() * 0x1_00_00;
    let p2 = b2.into() * 0x1_00;
    let p3 = b3.into();
    p0 | p1 | p2 | p3
}

pub fn u32_to_u8(value: u32) -> [u8; 4] {
    let b0 = ((value / 0x1_00_00_00) & 0xFF).try_into().unwrap();
    let b1 = ((value / 0x1_00_00) & 0xFF).try_into().unwrap();
    let b2 = ((value / 0x1_00) & 0xFF).try_into().unwrap();
    let b3 = (value & 0xFF).try_into().unwrap();
    [b0, b1, b2, b3]
}

pub fn u64_from_u8(val: [u8; 8]) -> u64 {
    let [b0, b1, b2, b3, b4, b5, b6, b7] = val;

    let p0 = b0.into() * 0x1_00_00_00_00_00_00_00;
    let p1 = b1.into() * 0x1_00_00_00_00_00_00;
    let p2 = b2.into() * 0x1_00_00_00_00_00;
    let p3 = b3.into() * 0x1_00_00_00_00;
    let p4 = b4.into() * 0x1_00_00_00;
    let p5 = b5.into() * 0x1_00_00;
    let p6 = b6.into() * 0x1_00;
    let p7 = b7.into();

    p0 | p1 | p2 | p3 | p4 | p5 | p6 | p7
}

pub fn u64_to_u8(value: u64) -> [u8; 8] {
    let b0 = ((value / 0x1_00_00_00_00_00_00_00) & 0xFF).try_into().unwrap();
    let b1 = ((value / 0x1_00_00_00_00_00_00) & 0xFF).try_into().unwrap();
    let b2 = ((value / 0x1_00_00_00_00_00) & 0xFF).try_into().unwrap();
    let b3 = ((value / 0x1_00_00_00_00) & 0xFF).try_into().unwrap();
    let b4 = ((value / 0x1_00_00_00) & 0xFF).try_into().unwrap();
    let b5 = ((value / 0x1_00_00) & 0xFF).try_into().unwrap();
    let b6 = ((value / 0x1_00) & 0xFF).try_into().unwrap();
    let b7 = (value & 0xFF).try_into().unwrap();
    [b0, b1, b2, b3, b4, b5, b6, b7]
}

pub fn felt252_to_array_u8(value: felt252) -> Array<u8> {
    let mut value_bytes: Array<u8> = array![];
    let mut i = 0;
    let mut current_value: u256 = value.into();
    while current_value != 0 && i != 31 {
        let low = current_value % 0x100;
        let lsb_u8: u8 = low.try_into().unwrap();
        value_bytes.append(lsb_u8);
        i += 1;
        current_value = current_value / 0x100;
    }
    reverse_array(value_bytes)
}

pub fn felt252_to_byte_array(value: felt252) -> ByteArray {
    SpanU8IntoByteArray::into(felt252_to_array_u8(value).span())
}

pub fn u64_into_array_u32(value: u64) -> Array<u32> {
    let mut array: Array<u32> = ArrayTrait::new();
    let upper = (value / 0x100000000).try_into().unwrap();
    let lower = (value % 0x100000000).try_into().unwrap();
    array.append(upper);
    array.append(lower);
    array
}

pub fn next_power_of_two(num: u32) -> u32 {
    if num == 0 {
        return 1;
    }
    let mut n = num - 1;
    n = n | (n / 2); // n |= n >> 1;
    n = n | (n / 4); // n |= n >> 2;
    n = n | (n / 16); // n |= n >> 4;
    n = n | (n / 256); // n |= n >> 8;
    n = n | (n / 65536); // n |= n >> 16;

    // we can stop, as `num` is u32.

    n + 1
}

pub fn encode_2_complement_128(value: @i128) -> u128 {
    let value = *value;
    if value < 0 {
        ((-value).try_into().unwrap() ^ 0xFFFFFFFFFFFFFFFF) + 1
    } else {
        value.try_into().unwrap()
    }
}

pub fn decode_2_complement_128(value: @u128) -> i128 {
    let value = *value;
    if value & 0x8000000000000000 != 0 {
        -((value - 1) ^ 0xFFFFFFFFFFFFFFFF).try_into().unwrap()
    } else {
        value.try_into().unwrap()
    }
}

pub fn encode_2_complement_64(value: @i64) -> u64 {
    let value = *value;
    if value < 0 {
        ((-value).try_into().unwrap() ^ 0xFFFFFFFFFFFFFFFF) + 1
    } else {
        value.try_into().unwrap()
    }
}

pub fn decode_2_complement_64(value: @u64) -> i64 {
    let value = *value;
    if value & 0x8000000000000000 != 0 {
        -((value - 1) ^ 0xFFFFFFFFFFFFFFFF).try_into().unwrap()
    } else {
        value.try_into().unwrap()
    }
}


pub fn encode_2_complement_32(value: @i32) -> u32 {
    let value = *value;
    if value < 0 {
        ((-value).try_into().unwrap() ^ 0xFFFFFFFF) + 1
    } else {
        value.try_into().unwrap()
    }
}

pub fn decode_2_complement_32(value: @u32) -> i32 {
    let value = *value;
    if value & 0x80000000 != 0 {
        -((value - 1) ^ 0xFFFFFFFF).try_into().unwrap()
    } else {
        value.try_into().unwrap()
    }
}

pub fn u64_to_little_endian(value: @u64) -> [u8; 8] {
    let mut value = *value;

    let byte0 = (value % 0x100).try_into().unwrap();

    value = value / 0x100;
    let byte1 = (value % 0x100).try_into().unwrap();

    value = value / 0x100;
    let byte2 = (value % 0x100).try_into().unwrap();

    value = value / 0x100;
    let byte3 = (value % 0x100).try_into().unwrap();

    value = value / 0x100;
    let byte4 = (value % 0x100).try_into().unwrap();

    value = value / 0x100;
    let byte5 = (value % 0x100).try_into().unwrap();

    value = value / 0x100;
    let byte6 = (value % 0x100).try_into().unwrap();

    value = value / 0x100;
    let byte7 = (value % 0x100).try_into().unwrap();

    [byte0, byte1, byte2, byte3, byte4, byte5, byte6, byte7]
}

pub fn little_endian_to_u64(value: @[u8; 8]) -> u64 {
    let value = value.span();

    let mut result: u64 = 0;

    result = result | ((*value[7]).into());

    result = result * 0x100;
    result = result | ((*value[6]).into());

    result = result * 0x100;
    result = result | ((*value[5]).into());

    result = result * 0x100;
    result = result | ((*value[4]).into());

    result = result * 0x100;
    result = result | ((*value[3]).into());

    result = result * 0x100;
    result = result | ((*value[2]).into());

    result = result * 0x100;
    result = result | ((*value[1]).into());

    result = result * 0x100;
    result = result | ((*value[0]).into());

    result
}

#[cfg(test)]
mod tests {
    use super::{
        decode_2_complement_128, decode_2_complement_64, encode_2_complement_128,
        encode_2_complement_64, little_endian_to_u64, next_power_of_two, u32_from_u8, u32_to_u8,
        u64_from_u8, u64_to_little_endian, u64_to_u8,
    };

    #[test]
    fn test_128_encode_decode_2_complement_zero() {
        let value: i128 = 0;
        let encoded = encode_2_complement_128(@value);
        assert_eq!(encoded, 0, "invalid encoded value");
        let decoded = decode_2_complement_128(@encoded);
        assert_eq!(decoded, value, "invalid decoded value");
    }

    #[test]
    fn test_128_encode_decode_2_complement_one() {
        let value: i128 = 1;
        let encoded = encode_2_complement_128(@value);
        assert_eq!(encoded, 1, "invalid encoded value");
        let decoded = decode_2_complement_128(@encoded);
        assert_eq!(decoded, value, "invalid decoded value");
    }

    #[test]
    fn test_128_encode_decode_2_complement_neg_one() {
        let value: i128 = -0x1;
        let encoded = encode_2_complement_128(@value);
        assert_eq!(encoded, 0xFFFFFFFFFFFFFFFF, "invalid encoded value");
        let decoded = decode_2_complement_128(@encoded);
        assert_eq!(decoded, value, "invalid decoded value");
    }

    #[test]
    fn test_encode_decode_2_complement_zero() {
        let value = 0;
        let encoded = encode_2_complement_64(@value);
        assert_eq!(encoded, 0, "invalid encoded value");
        let decoded = decode_2_complement_64(@encoded);
        assert_eq!(decoded, value, "invalid decoded value");
    }

    #[test]
    fn test_encode_decode_2_complement_one() {
        let value = 1;
        let encoded = encode_2_complement_64(@value);
        assert_eq!(encoded, 1, "invalid encoded value");
        let decoded = decode_2_complement_64(@encoded);
        assert_eq!(decoded, value, "invalid decoded value");
    }

    #[test]
    fn test_encode_decode_2_complement_neg_one() {
        let value = -0x1;
        let encoded = encode_2_complement_64(@value);
        assert_eq!(encoded, 0xFFFFFFFFFFFFFFFF, "invalid encoded value");
        let decoded = decode_2_complement_64(@encoded);
        assert_eq!(decoded, value, "invalid decoded value");
    }

    #[test]
    fn test_little_endian_encode() {
        let value = 0x123456789ABCDEF0;
        let encoded = u64_to_little_endian(@value);
        assert_eq!(
            encoded, [0xF0, 0xDE, 0xBC, 0x9A, 0x78, 0x56, 0x34, 0x12], "invalid encoded value",
        );
    }

    #[test]
    fn test_little_endian_decode() {
        let bytes = [0xF0, 0xDE, 0xBC, 0x9A, 0x78, 0x56, 0x34, 0x12];
        let decoded = little_endian_to_u64(@bytes);
        assert_eq!(decoded, 0x123456789ABCDEF0, "invalid decoded value");
    }

    #[test]
    #[fuzzer]
    fn fuzz_little_endian_encode(x: u64) {
        let encoded = u64_to_little_endian(@x);
        let decoded = little_endian_to_u64(@encoded);
        assert_eq!(decoded, x, "invalid decoded value");
    }

    #[test]
    #[fuzzer]
    fn fuzz_little_endian_decode(b1: u8, b2: u8, b3: u8, b4: u8, b5: u8, b6: u8, b7: u8, b8: u8) {
        let bytes = [b1, b2, b3, b4, b5, b6, b7, b8];
        let decoded = little_endian_to_u64(@bytes);
        let encoded = u64_to_little_endian(@decoded);
        assert_eq!(encoded, bytes, "invalid encoded value");
    }

    #[test]
    fn test_next_power_of_two() {
        assert_eq!(next_power_of_two(0), 1);
        assert_eq!(next_power_of_two(1), 1);
        assert_eq!(next_power_of_two(2), 2);
        assert_eq!(next_power_of_two(3), 4);
        assert_eq!(next_power_of_two(4), 4);
        assert_eq!(next_power_of_two(5), 8);
        assert_eq!(next_power_of_two(6), 8);
        assert_eq!(next_power_of_two(7), 8);
        assert_eq!(next_power_of_two(8), 8);
        assert_eq!(next_power_of_two(9), 16);
    }

    #[test]
    fn test_u32_to_u8() {
        let val_u32 = 0x12345678;
        let [b0, b1, b2, b3] = [0x12, 0x34, 0x56, 0x78];
        let actual = u32_to_u8(val_u32);
        assert_eq!([b0, b1, b2, b3], actual, "u32 to u8 fail")
    }

    #[test]
    fn test_u8_to_u32() {
        let [b0, b1, b2, b3] = [0x12, 0x34, 0x56, 0x78];
        let val_u32 = 0x12345678;
        let actual = u32_from_u8([b0, b1, b2, b3]);
        assert_eq!(val_u32, actual, "u32 from u8 fail")
    }

    #[test]
    fn test_u8_to_u64() {
        let [b0, b1, b2, b3, b4, b5, b6, b7] = [0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
        let val_u32 = 0x1234567890ABCDEF;
        let actual = u64_from_u8([b0, b1, b2, b3, b4, b5, b6, b7]);
        assert_eq!(val_u32, actual, "u64 from u8 fail")
    }

    #[test]
    fn test_u64_to_u8() {
        let val_u32 = 0x1234567890ABCDEF;
        let [b0, b1, b2, b3, b4, b5, b6, b7] = [0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];
        let actual = u64_to_u8(val_u32);
        assert_eq!([b0, b1, b2, b3, b4, b5, b6, b7], actual, "u64 to u8 fail")
    }
}
