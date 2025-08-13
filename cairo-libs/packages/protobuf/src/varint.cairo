use protobuf::errors::ProtobufErrors;

/// Implements variable-value encoding based on LEB128
#[inline]
pub fn encode_varint_to_u8_array(value: u128) -> Array<u8> {
    let mut result: Array<u8> = array![];
    let mut value = value;
    for _ in 0..19_u32 {
        if value < 0x80 {
            result.append(value.try_into().unwrap());
            break;
        } else {
            let remaining = (value & 0x7F) | 0x80;
            result.append(remaining.try_into().unwrap());
            value /= 0x80;
        };
    }
    result
}

/// Decodes a LEB128-encoded variable length integer from a slice that might contain a number of
/// ints, returning the list of values + number of bytes read.
#[inline]
pub fn decode_varint_from_u8_array(ref bytes: Array<u8>) -> (u64, u32) {
    let len = bytes.len();
    assert(len > 0, ProtobufErrors::INVALID_VARINT_SIZE);

    let mut num_of_read = 0;
    let mut value = 0;
    let mut shift = 1;
    while num_of_read != 10 {
        let byte = bytes.pop_front().unwrap();
        assert(!(num_of_read == 9 && byte > 0x01), ProtobufErrors::OVERFLOWED_VARINT);

        num_of_read += 1;
        if num_of_read == 1 && byte < 0x80 {
            value = byte.try_into().unwrap();
            break;
        }

        value = value | ((byte & 0x7F).into() * shift); // 0x7F == 0x0111_1111
        shift *= 0x80; // 0x80 == 0x1000_0000

        if byte & 0x80 == 0 {
            break;
        }
    }

    (value, num_of_read)
}

#[inline]
pub fn decode_varint_from_u8_array_with_index(
    bytes: Span<u8>, ref index: usize,
) -> Result<u128, felt252> {
    let mut value: u128 = 0;
    let mut shift: u128 = 1;
    let mut done = false;
    loop {
        let byte = *bytes[index];
        index += 1;
        // 0x7F == 0x0111_1111
        value = value | ((byte & 0x7F).into() * shift);
        if byte & 0x80 == 0 {
            done = true;
            break;
        }
        // 0x80 == 0x1000_0000
        shift *= 0x80;
    }
    if !done {
        return Result::Err(ProtobufErrors::INVALID_VARINT);
    }
    Result::Ok(value)
}


#[inline]
pub fn decode_varint_from_byte_array(bytes: @ByteArray, ref index: usize) -> Result<u128, felt252> {
    let mut value: u128 = 0;
    let mut shift: u128 = 1;
    let mut done = false;
    while let Option::Some(byte) = bytes.at(index) {
        index += 1;
        // 0x7F == 0x0111_1111
        value = value | ((byte & 0x7F).into() * shift);
        if byte & 0x80 == 0 {
            done = true;
            break;
        }
        // 0x80 == 0x1000_0000
        shift *= 0x80;
    }
    if !done {
        return Result::Err(ProtobufErrors::INVALID_VARINT);
    }
    Result::Ok(value)
}
