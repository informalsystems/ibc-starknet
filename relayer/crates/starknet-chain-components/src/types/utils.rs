use starknet::core::types::Felt;

pub fn felt_to_string(felt: Felt) -> String {
    let felt_as_be_bytes = felt.to_bytes_be();
    let felt_as_string = String::from_utf8_lossy(&felt_as_be_bytes);
    felt_as_string.trim_start_matches('\0').to_string()
}

pub fn string_to_felt(value: &str) -> Option<Felt> {
    (value.len() <= 32).then(|| {
        let mut buf = [0u8; 32];
        buf[32 - value.len()..].copy_from_slice(value.as_bytes());
        Felt::from_bytes_be(&buf)
    })
}

#[cfg(test)]
mod test {
    use starknet::macros::short_string;

    use super::*;

    #[test]
    fn test_felt_to_string() {
        let felt = short_string!("hello world");
        assert_eq!(felt_to_string(felt), "hello world");
        assert_eq!(string_to_felt("hello world"), Some(felt));
    }
}
