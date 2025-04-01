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
    use super::{decode_2_complement_64, encode_2_complement_64};

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
}
