/// Convert an input of type `ArrayByte` to an array of `u8`.
pub fn byte_array_to_array_u8(input: @ByteArray) -> Array<u8> {
    let mut output: Array<u8> = array![];
    let mut i = 0;
    while let Option::Some(value) = input.at(i) {
        output.append(value);
        i += 1;
    }
    output
}
