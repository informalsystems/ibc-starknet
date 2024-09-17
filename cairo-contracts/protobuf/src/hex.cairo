pub fn digit_to_u4(value: u8) -> u8 {
    if '0' <= value && value <= '9' {
        value - '0'
    } else if 'A' <= value && value <= 'F' {
        value - 'A' + 10
    } else if 'a' <= value && value <= 'f' {
        value - 'a' + 10
    } else {
        panic!("Invalid u4 hex digit: {}", value)
    }
}

pub fn digit_to_u8(high: u8, low: u8) -> u8 {
    return digit_to_u4(high) * 0x10 | digit_to_u4(low);
}

pub fn u4_to_digit(value: u8) -> u8 {
    if value < 10 {
        '0' + value
    } else if value < 16 {
        // always uppercase
        'A' + value - 10
    } else {
        panic!("Invalid u4 hex value: {}", value)
    }
}

pub fn u8_to_digit(value: u8) -> (u8, u8) {
    let high = u4_to_digit(value / 0x10);
    let low = u4_to_digit(value & 0x0F);
    (high, low)
}

pub fn encode(input: @ByteArray) -> ByteArray {
    let mut output = "";
    let mut i = 0;
    while i < input.len() {
        let (high, low) = u8_to_digit(input[i]);
        output.append_byte(high);
        output.append_byte(low);
        i += 1;
    };
    output
}

pub fn decode(input: @ByteArray) -> ByteArray {
    assert(input.len() % 2 == 0, 'Invalid hex string length');
    let mut output = "";
    let mut i = 0;
    while i < input.len() {
        let value = digit_to_u8(input[i], input[i + 1]);
        output.append_byte(value);
        i += 2;
    };
    output
}

#[cfg(test)]
mod tests {
    use super::{u8_to_digit, digit_to_u8, encode, decode};

    #[test]
    fn test_u8_to_digit() {
        assert_eq!(u8_to_digit(0x00), ('0', '0'));
        assert_eq!(u8_to_digit(0x01), ('0', '1'));
        assert_eq!(u8_to_digit(0x0A), ('0', 'A'));
        assert_eq!(u8_to_digit(0x0F), ('0', 'F'));
        assert_eq!(u8_to_digit(0x10), ('1', '0'));
        assert_eq!(u8_to_digit(0x1F), ('1', 'F'));
        assert_eq!(u8_to_digit(0xA0), ('A', '0'));
        assert_eq!(u8_to_digit(0xAF), ('A', 'F'));
        assert_eq!(u8_to_digit(0xF0), ('F', '0'));
        assert_eq!(u8_to_digit(0xFF), ('F', 'F'));
    }

    #[test]
    fn test_digit_to_u8() {
        assert_eq!(digit_to_u8('0', '0'), 0x00);
        assert_eq!(digit_to_u8('0', '1'), 0x01);
        assert_eq!(digit_to_u8('0', 'A'), 0x0A);
        assert_eq!(digit_to_u8('0', 'F'), 0x0F);
        assert_eq!(digit_to_u8('1', '0'), 0x10);
        assert_eq!(digit_to_u8('1', 'F'), 0x1F);
        assert_eq!(digit_to_u8('A', '0'), 0xA0);
        assert_eq!(digit_to_u8('A', 'F'), 0xAF);
        assert_eq!(digit_to_u8('F', '0'), 0xF0);
        assert_eq!(digit_to_u8('F', 'F'), 0xFF);
    }

    #[test]
    fn test_encode() {
        let input = "hello";
        let expected = "68656C6C6F";
        let actual = encode(@input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_decode() {
        // lowercase works too
        let input = "68656C6C6f";
        let expected = "hello";
        let actual = decode(@input);
        assert_eq!(actual, expected);
    }
}
