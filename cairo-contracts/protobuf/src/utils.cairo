pub fn byte_array_to_array_u8(input: @ByteArray) -> Array<u8> {
    let mut output: Array<u8> = array![];
    let mut i = 0;
    while i < input.len() {
        output.append(input[i]);
        i += 1;
    };
    output
}

pub fn array_u8_to_byte_array(input: @Array<u8>) -> ByteArray {
    let mut output = "";
    let mut i = 0;
    while i < input.len() {
        output.append_byte(*input[i]);
        i += 1;
    };
    output
}

#[cfg(test)]
mod tests {
    use super::{byte_array_to_array_u8, array_u8_to_byte_array};

    #[test]
    fn test_byte_array_to_array_u8() {
        let input = "hello";
        let expected = array![0x68, 0x65, 0x6c, 0x6c, 0x6f];
        let actual = byte_array_to_array_u8(@input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_array_u8_to_byte_array() {
        let input = array![0x68, 0x65, 0x6c, 0x6c, 0x6f];
        let expected = "hello";
        let actual = array_u8_to_byte_array(@input);
        assert_eq!(actual, expected);
    }
}
