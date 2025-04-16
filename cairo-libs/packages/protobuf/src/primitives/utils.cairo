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
        encode_2_complement_64, little_endian_to_u64, u64_to_little_endian,
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
}
