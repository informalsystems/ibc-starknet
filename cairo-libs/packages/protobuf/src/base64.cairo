pub fn base64_char_to_u6(value: u8) -> u8 {
    if 'A' <= value && value <= 'Z' {
        value - 'A'
    } else if 'a' <= value && value <= 'z' {
        value - 'a' + 26
    } else if '0' <= value && value <= '9' {
        value - '0' + 52
    } else if value == '+' {
        62
    } else if value == '/' {
        63
    } else {
        panic!("invalid Base64 character: {}", value)
    }
}

pub fn u6_to_base64_char(value: u8) -> u8 {
    if value < 26 {
        'A' + value
    } else if value < 52 {
        'a' + value - 26
    } else if value < 62 {
        '0' + value - 52
    } else if value == 62 {
        '+'
    } else if value == 63 {
        '/'
    } else {
        panic!("invalid u6 Base64 value: {}", value)
    }
}

pub fn encode(input: @ByteArray) -> ByteArray {
    let mut output = "";
    let input_len = input.len();
    let mut i = 0;

    while i < input_len {
        let b0: u32 = input[i].into();
        let b1: u32 = if i + 1 < input_len {
            input[i + 1].into()
        } else {
            0
        };
        let b2: u32 = if i + 2 < input_len {
            input[i + 2].into()
        } else {
            0
        };

        // 0x10000 = 2^16; 0x100 = 2^8(= 2^16 / 2^8); 0x40 = 2^6
        let triple: u32 = b0 * 0x10000 + b1 * 0x100 + b2;

        // 0x40000 = 2^18; 0x3f = 2^6 - 1
        let c0 = u6_to_base64_char(((triple / 0x40000) & 0x3f).try_into().unwrap());
        // 0x1000 = 2^12
        let c1 = u6_to_base64_char(((triple / 0x1000) & 0x3f).try_into().unwrap());
        let c2 = if i + 1 < input_len {
            // 0x40 = 2^6
            u6_to_base64_char(((triple / 0x40) & 0x3f).try_into().unwrap())
        } else {
            '='
        };
        let c3 = if i + 2 < input_len {
            u6_to_base64_char((triple & 0x3f).try_into().unwrap())
        } else {
            '='
        };

        output.append_byte(c0);
        output.append_byte(c1);
        output.append_byte(c2);
        output.append_byte(c3);

        i += 3;
    }
    output
}

pub fn decode(input: @ByteArray) -> ByteArray {
    let input_len = input.len();
    assert(input_len % 4 == 0, 'invalid Base64 string length');
    let mut output = "";
    let mut i = 0;

    // Since input_len % 4 == 0, we know i += 4 will eventually be
    // equal to input_len
    while let (Some(c0), Some(c1), Some(c2), Some(c3)) =
        (input.at(i), input.at(i + 1), input.at(i + 2), input.at(i + 3)) {
        let sextet0: u32 = base64_char_to_u6(c0).into();
        let sextet1: u32 = base64_char_to_u6(c1).into();
        let sextet2: u32 = if c2 != '=' {
            base64_char_to_u6(c2).into()
        } else {
            0
        };
        let sextet3: u32 = if c3 != '=' {
            base64_char_to_u6(c3).into()
        } else {
            0
        };

        let triple: u32 = sextet0 * 0x40000 + sextet1 * 0x1000 + sextet2 * 0x40 + sextet3;

        let b0 = ((triple / 0x10000) & 0xff).try_into().unwrap();
        let b1 = ((triple / 0x100) & 0xff).try_into().unwrap();
        let b2 = (triple & 0xff).try_into().unwrap();

        output.append_byte(b0);
        if c2 != '=' {
            output.append_byte(b1);
        }
        if c3 != '=' {
            output.append_byte(b2);
        }

        i += 4;
    }
    output
}

#[cfg(test)]
mod tests {
    use super::{base64_char_to_u6, decode, encode, u6_to_base64_char};

    #[test]
    fn test_u6_to_base64_char() {
        assert_eq!(u6_to_base64_char(0), 'A');
        assert_eq!(u6_to_base64_char(25), 'Z');
        assert_eq!(u6_to_base64_char(26), 'a');
        assert_eq!(u6_to_base64_char(51), 'z');
        assert_eq!(u6_to_base64_char(52), '0');
        assert_eq!(u6_to_base64_char(61), '9');
        assert_eq!(u6_to_base64_char(62), '+');
        assert_eq!(u6_to_base64_char(63), '/');
    }

    #[test]
    fn test_base64_char_to_u6() {
        assert_eq!(base64_char_to_u6('A'), 0);
        assert_eq!(base64_char_to_u6('Z'), 25);
        assert_eq!(base64_char_to_u6('a'), 26);
        assert_eq!(base64_char_to_u6('z'), 51);
        assert_eq!(base64_char_to_u6('0'), 52);
        assert_eq!(base64_char_to_u6('9'), 61);
        assert_eq!(base64_char_to_u6('+'), 62);
        assert_eq!(base64_char_to_u6('/'), 63);
    }

    #[test]
    fn test_round_trip_base64() {
        let test_cases = array![
            ("", ""),
            ("Man", "TWFu"),
            ("foobar", "Zm9vYmFy"),
            ("f", "Zg=="),
            ("foob", "Zm9vYg=="),
            ("fo", "Zm8="),
            ("fooba", "Zm9vYmE="),
            ("hello", "aGVsbG8="),
            ("hi", "aGk="),
            (
                "The quick brown fox jumps over the lazy dog",
                "VGhlIHF1aWNrIGJyb3duIGZveCBqdW1wcyBvdmVyIHRoZSBsYXp5IGRvZw==",
            ),
        ];

        for (input, expected_encoded) in test_cases {
            let encoded = encode(@input);
            assert_eq!(@encoded, @expected_encoded, "Encoding failed for input: '{}'", input);

            let decoded = decode(@encoded);
            assert_eq!(@decoded, @input, "Decoding failed for encoded string: '{}'", encoded);
        }
    }
}
