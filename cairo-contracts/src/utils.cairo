/// Converts a generic type 'T' to a felt252.
pub trait ToFelt252Trait<T> {
    fn try_to_felt252(self: @T) -> Option<felt252>;
}

// Note: if bytes has leading `\0` (null char), `felt252` forgets that information.
// bytes_to_felt252(felt252_to_bytes(x)).unwrap() == x
pub fn bytes_to_felt252(bytes: @ByteArray) -> Option<felt252> {
    if bytes.len() == 0 {
        return Option::Some('');
    }

    if bytes.len() > 31 {
        return Option::None(());
    }

    let mut result: felt252 = 0;
    let mut multiplier: felt252 = 1;

    // Iterate through the bytes in reverse order
    let mut i = bytes.len();
    loop {
        if i == 0 {
            break;
        }
        i -= 1;

        let byte_value = bytes.at(i).unwrap();
        result += byte_value.into() * multiplier;
        multiplier *= 0x100; // 256
    };

    Option::Some(result)
}

// felt252_to_bytes(bytes_to_felt252(x).unwrap()) == x
pub fn felt252_to_bytes(felt: felt252) -> ByteArray {
    if felt == '' {
        return "";
    }

    let mut result: ByteArray = "";
    let mut remaining: u256 = felt.into();

    loop {
        if remaining == 0 {
            break;
        }

        let byte_value = remaining % 0x100; // 256

        result.append_byte(byte_value.try_into().unwrap());

        remaining /= 0x100; // 256
    };

    result.rev()
}


#[cfg(test)]
mod test {
    use super::{bytes_to_felt252, felt252_to_bytes};

    #[test]
    fn test_empty_string() {
        let bytes = "";
        let result = bytes_to_felt252(@bytes);
        assert!(result == Option::Some(0), "Empty string should convert to 0");
        let result_back = felt252_to_bytes(result.unwrap());
        assert!(result_back == bytes, "Empty string should convert to 0");
    }

    #[test]
    fn test_single_character() {
        let bytes = "A";
        let result = bytes_to_felt252(@bytes);
        assert!(result == Option::Some('A'), "Single character conversion failed");
        let result_back = felt252_to_bytes(result.unwrap());
        assert!(result_back == bytes, "Single character conversion failed");
    }

    #[test]
    fn test_multiple_bytes() {
        let bytes = "abc";
        let result = bytes_to_felt252(@bytes);
        assert!(result == Option::Some('abc'), "Multiple byte conversion failed");
        let result_back = felt252_to_bytes(result.unwrap());
        assert!(result_back == bytes, "Multiple byte conversion failed");
    }

    #[test]
    fn test_max_bytes() {
        let bytes = "abcdefghijklmnopqrstuvwxyz12345"; // 31 characters
        let result = bytes_to_felt252(@bytes);
        assert!(
            result == Option::Some('abcdefghijklmnopqrstuvwxyz12345'), "Max bytes conversion failed"
        );
        let result_back = felt252_to_bytes(result.unwrap());
        assert!(result_back == bytes, "Max bytes conversion failed");
    }

    #[test]
    fn test_too_many_bytes() {
        let bytes = "abcdefghijklmnopqrstuvwxyz123456"; // 32 characters
        let result = bytes_to_felt252(@bytes);
        assert!(result == Option::None(()), "More than characters should return None");
    }

    #[test]
    fn test_special_characters() {
        let bytes = "!@#$";
        let result = bytes_to_felt252(@bytes);
        assert!(result == Option::Some('!@#$'), "Special characters conversion failed");
        let result_back = felt252_to_bytes(result.unwrap());
        assert!(result_back == bytes, "Special characters conversion failed");
    }

    #[test]
    fn test_null_character() {
        let bytes = "abc\0def\0";
        let result = bytes_to_felt252(@bytes);
        assert!(result == Option::Some('abc\0def\0'), "Null character conversion failed");
        let result_back = felt252_to_bytes(result.unwrap());
        assert!(result_back == bytes, "Null character conversion failed");
    }

    #[test]
    fn test_leading_null_characters() {
        let bytes = "\0\0\0abc";
        let result = bytes_to_felt252(@bytes);
        assert!(result == Option::Some('\0\0\0abc'), "Leading null character conversion failed");
        let result_back = felt252_to_bytes(result.unwrap());
        // trims the leading null characters
        assert!(result_back == "abc", "Leading null character conversion failed");
    }
}
