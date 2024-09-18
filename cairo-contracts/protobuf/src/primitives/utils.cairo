pub fn decode_varint_u64(bytes: @ByteArray, ref index: usize) -> u64 {
    let mut value: u64 = 0;
    let mut shift: u64 = 1;
    let mut done = false;
    while index < bytes.len() {
        let byte = bytes[index];
        index += 1;
        // 0x7F == 0x0111_1111
        value = value | ((byte & 0x7F).into() * shift);
        if byte & 0x80 == 0 {
            done = true;
            break;
        }
        // 0x80 == 0x1000_0000
        shift *= 0x80;
    };
    if !done {
        panic!("invalid varint");
    }
    value
}

pub fn encode_varint_u64(value: @u64) -> ByteArray {
    if value == @0 {
        return "\x00";
    }
    let mut bytes = "";
    let mut value = *value;
    while value > 0 {
        let mut byte: u8 = (value & 0x7F).try_into().unwrap();
        value = value / 0x80;
        if value > 0 {
            byte = byte | 0x80;
        }
        bytes.append_byte(byte);
    };
    bytes
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


#[cfg(test)]
mod tests {
    use super::{
        decode_varint_u64, encode_varint_u64, decode_2_complement_64, encode_2_complement_64
    };

    use protobuf::hex::decode as hex_decode;

    #[test]
    fn test_encode_varint_u64_default() {
        assert_eq!(encode_varint_u64(@0), hex_decode(@"00"));
        let mut index = 0;
        assert_eq!(decode_varint_u64(@hex_decode(@"00"), ref index), 0);
    }

    #[test]
    fn test_encode_decode_varint_u64() {
        let value = 0x1234567890ABCDEF;
        let bytes = encode_varint_u64(@value);
        let hex = "ef9baf8589cf959a12";
        let bytes2 = hex_decode(@hex);
        assert_eq!(bytes, bytes2, "invalid encoded bytes");
        let mut index = 0;
        let decoded = decode_varint_u64(@bytes, ref index);
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
}
