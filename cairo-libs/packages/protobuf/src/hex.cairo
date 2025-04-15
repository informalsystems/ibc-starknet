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

pub fn encode(input: @ByteArray) -> ByteArray {
    let input_len = input.len();
    let mut output = "";
    let mut i = 0;
    while let Some(value) = input.at(i) {
        output.append_byte(u4_to_digit(value / 0x10));
        output.append_byte(u4_to_digit(value & 0x0F));
        i += 1;
    }
    output
}

pub fn decode(input: @ByteArray) -> ByteArray {
    let input_len = input.len();
    assert(input_len % 2 == 0, 'Invalid hex string length');
    let mut output = "";
    let mut i = 0;
    // Since input_len % 2 == 0, we know i += 2 will eventually be
    // equal to input_len
    while let (Some(c0), Some(c1)) = (input.at(i), input.at(i + 1)) {
        let value = (digit_to_u4(c0) * 0x10) | digit_to_u4(c1);
        output.append_byte(value);
        i += 2;
    }
    output
}

#[cfg(test)]
mod tests {
    use super::{decode, digit_to_u4, encode, u4_to_digit};

    #[test]
    fn test_digit_to_u4() {
        assert_eq!(digit_to_u4('0'), 0x00);
        assert_eq!(digit_to_u4('1'), 0x01);
        assert_eq!(digit_to_u4('A'), 0x0A);
        assert_eq!(digit_to_u4('F'), 0x0F);
        assert_eq!(digit_to_u4('a'), 0x0A);
        assert_eq!(digit_to_u4('f'), 0x0F);
    }

    #[test]
    fn test_u4_to_digit() {
        assert_eq!(u4_to_digit(0x00), '0');
        assert_eq!(u4_to_digit(0x01), '1');
        assert_eq!(u4_to_digit(0x0A), 'A');
        assert_eq!(u4_to_digit(0x0F), 'F');
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
