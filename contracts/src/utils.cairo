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

#[cfg(test)]
mod test {
    use super::bytes_to_felt252;

    #[test]
    fn test_empty_string() {
        let bytes = "";
        let result = bytes_to_felt252(@bytes);
        assert!(result == Option::Some(0), "Empty string should convert to 0");
    }

    #[test]
    fn test_single_character() {
        let bytes = "A";
        let result = bytes_to_felt252(@bytes);
        assert!(result == Option::Some('A'), "Single character conversion failed");
    }

    #[test]
    fn test_multiple_bytes() {
        let bytes = "abc";
        let result = bytes_to_felt252(@bytes);
        assert!(result == Option::Some('abc'), "Multiple byte conversion failed");
    }

    #[test]
    fn test_max_bytes() {
        let bytes = "abcdefghijklmnopqrstuvwxyz12345"; // 31 characters
        let result = bytes_to_felt252(@bytes);
        assert!(
            result == Option::Some('abcdefghijklmnopqrstuvwxyz12345'), "Max bytes conversion failed"
        );
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
    }

    #[test]
    fn test_all_zeros() {
        let bytes = "\0\0\0\0";
        let result = bytes_to_felt252(@bytes);
        assert!(result == Option::Some(0), "All zeros should convert to 0");
    }

    #[test]
    fn test_special_characters() {
        let bytes = "!@#$";
        let result = bytes_to_felt252(@bytes);
        assert!(result == Option::Some('!@#$'), "Special characters conversion failed");
    }
}
