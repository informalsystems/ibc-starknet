use ibc_utils::bytes::ByteArrayIntoArrayU8;
use crate::char::{digit_to_u4, u4_to_lower_digit, u4_to_upper_digit};

pub fn encode(mut input: Span<u8>) -> Array<u8> {
    encode_upper(input)
}

pub fn encode_upper(mut input: Span<u8>) -> Array<u8> {
    let mut output = array![];
    let mut i = 0;
    while let Some(elem) = input.pop_front() {
        let value = *elem;
        output.append(u4_to_upper_digit(value / 0x10));
        output.append(u4_to_upper_digit(value & 0x0F));
        i += 1;
    }
    output
}

pub fn encode_lower(mut input: Span<u8>) -> Array<u8> {
    let mut output = array![];
    let mut i = 0;
    while let Some(elem) = input.pop_front() {
        let value = *elem;
        output.append(u4_to_lower_digit(value / 0x10));
        output.append(u4_to_lower_digit(value & 0x0F));
        i += 1;
    }
    output
}

pub fn decode(mut input: Span<u8>) -> Array<u8> {
    assert(input.len() % 2 == 0, 'Invalid hex string length');
    let mut output = array![];
    // Since input_len % 2 == 0, we know i += 2 will eventually be
    // equal to input_len
    while let Some(pair) = input.multi_pop_front() {
        let [c0, c1] = (*pair).unbox();
        let value = (digit_to_u4(c0) * 0x10) | digit_to_u4(c1);
        output.append(value);
    }
    output
}

pub fn decode_byte_array(input: ByteArray) -> Array<u8> {
    decode(ByteArrayIntoArrayU8::into(input).span())
}

#[cfg(test)]
mod tests {
    use crate::bytes::ByteArrayIntoArrayU8;
    use super::{
        decode, digit_to_u4, encode_lower, encode_upper, u4_to_lower_digit, u4_to_upper_digit,
    };

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
    fn test_u4_to_upper_digit() {
        assert_eq!(u4_to_upper_digit(0x00), '0');
        assert_eq!(u4_to_upper_digit(0x01), '1');
        assert_eq!(u4_to_upper_digit(0x0A), 'A');
        assert_eq!(u4_to_upper_digit(0x0F), 'F');
    }

    #[test]
    fn test_u4_to_lower_digit() {
        assert_eq!(u4_to_lower_digit(0x00), '0');
        assert_eq!(u4_to_lower_digit(0x01), '1');
        assert_eq!(u4_to_lower_digit(0x0A), 'a');
        assert_eq!(u4_to_lower_digit(0x0F), 'f');
    }

    #[test]
    fn test_encode_upper() {
        let input = ByteArrayIntoArrayU8::into("hello");
        let expected = ByteArrayIntoArrayU8::into("68656C6C6F");
        let actual = encode_upper(input.span());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_encode_lower() {
        let input = ByteArrayIntoArrayU8::into("hello");
        let expected = ByteArrayIntoArrayU8::into("68656c6c6f");
        let actual = encode_lower(input.span());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_decode() {
        // lowercase works too
        let input = ByteArrayIntoArrayU8::into("68656C6C6f");
        let expected = ByteArrayIntoArrayU8::into("hello");
        let actual = decode(input.span());
        assert_eq!(actual, expected);
    }
}
