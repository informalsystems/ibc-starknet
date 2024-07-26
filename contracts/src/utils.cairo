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

pub fn felt252_to_bytes(value: felt252) -> ByteArray {
    if value == '' {
        return "";
    }

    let mut result: ByteArray = "";
    let mut remaining: u256 = value.into();

    loop {
        let byte_value = (remaining % 0x100); // 256
        remaining /= 0x100; // 256

        result.append_byte(byte_value.try_into().unwrap());
        if remaining == 0 {
            break;
        }
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
        assert!(bytes == result_back, "Empty string should convert to 0");
    }

    #[test]
    fn test_single_character() {
        let bytes = "A";
        let result = bytes_to_felt252(@bytes);
        assert!(result == Option::Some('A'), "Single character conversion failed");
        let result_back = felt252_to_bytes(result.unwrap());
        assert!(bytes == result_back, "Single character conversion failed");
    }

    #[test]
    fn test_multiple_bytes() {
        let bytes = "abc";
        let result = bytes_to_felt252(@bytes);
        assert!(result == Option::Some('abc'), "Multiple byte conversion failed");
        let result_back = felt252_to_bytes(result.unwrap());
        assert!(bytes == result_back, "Multiple byte conversion failed");
    }

    #[test]
    fn test_max_bytes() {
        let bytes = "abcdefghijklmnopqrstuvwxyz12345"; // 31 characters
        let result = bytes_to_felt252(@bytes);
        assert!(
            result == Option::Some('abcdefghijklmnopqrstuvwxyz12345'), "Max bytes conversion failed"
        );
        let result_back = felt252_to_bytes(result.unwrap());
        assert!(bytes == result_back, "Max bytes conversion failed");
    }

    #[test]
    fn test_too_many_bytes() {
        let bytes = "abcdefghijklmnopqrstuvwxyz123456"; // 32 characters
        let result = bytes_to_felt252(@bytes);
        assert!(result == Option::None(()), "More than characters should return None");
    }

    #[test]
    fn test_leading_zeros() {
        let bytes = "\0\0ab";
        let result = bytes_to_felt252(@bytes);
        assert!(result == Option::Some('ab'), "Leading zeros not handled correctly");
        let result_back = felt252_to_bytes(result.unwrap());
        assert!(bytes == result_back, "Leading zeros not handled correctly");
    }

    #[test]
    fn test_all_zeros() {
        let bytes = "\0\0\0\0";
        let result = bytes_to_felt252(@bytes);
        assert!(result == Option::Some(0), "All zeros should convert to 0");
        let result_back = felt252_to_bytes(result.unwrap());
        assert!(bytes == result_back, "All zeros should convert to 0");
    }

    #[test]
    fn test_special_characters() {
        let bytes = "!@#$";
        let result = bytes_to_felt252(@bytes);
        assert!(result == Option::Some('!@#$'), "Special characters conversion failed");
        let result_back = felt252_to_bytes(result.unwrap());
        assert!(bytes == result_back, "Special characters conversion failed");
    }
}
