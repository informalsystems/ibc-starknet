use ics23::{
    ArrayU8PartialOrd, array_u8_into_array_u32, decode_hex, encode_hex, felt252_to_u8_array,
    u64_into_array_u32,
};

#[test]
fn test_array_u8_into_array_u32() {
    let array = array![];
    let result = array_u8_into_array_u32(array);
    assert_eq!(result, (array![], 0, 0));

    let array = array![0];
    let result = array_u8_into_array_u32(array);
    assert_eq!(result, (array![], 0, 1));

    let array = array![0, 0, 0, 1];
    let result = array_u8_into_array_u32(array);
    assert_eq!(result, (array![1], 0, 0));

    let array = array![255, 255, 255, 255];
    let result = array_u8_into_array_u32(array);
    assert_eq!(result, (array![4294967295], 0, 0));

    // This corresponds to the following JSON: {"result": "AQ=="}, which
    // represents the successful acknowledgement in ICS-20 application.
    let array = array![123, 34, 114, 101, 115, 117, 108, 116, 34, 58, 34, 65, 81, 61, 61, 34, 125];
    let result = array_u8_into_array_u32(array);
    assert_eq!(result, (array![2065855077, 1937075316, 574235201, 1362967842], 125, 1));

    let array = array![
        123, 34, 114, 101, 115, 117, 108, 116, 34, 58, 34, 65, 81, 61, 61, 34, 125, 126,
    ];
    let result = array_u8_into_array_u32(array);
    assert_eq!(result, (array![2065855077, 1937075316, 574235201, 1362967842], 32126, 2));
}

#[test]
fn test_u64_into_array_u32() {
    assert_eq!(u64_into_array_u32(0), array![0, 0]);
    assert_eq!(u64_into_array_u32(1), array![0, 1]);
    assert_eq!(u64_into_array_u32(4294967296), array![1, 0]);
    assert_eq!(u64_into_array_u32(4294967297), array![1, 1]);
    assert_eq!(u64_into_array_u32(8589934592), array![2, 0]);
    assert_eq!(u64_into_array_u32(8589934593), array![2, 1]);
    assert_eq!(u64_into_array_u32(4294967295), array![0, 4294967295]);
    assert_eq!(u64_into_array_u32(18446744073709551615), array![4294967295, 4294967295]);
}

fn check_hex_codec(hex: @ByteArray) {
    assert_eq!(hex, @encode_hex(decode_hex(hex)));
}

#[test]
fn test_decode_hex() {
    check_hex_codec(@"");
    check_hex_codec(@"00");
    check_hex_codec(@"01");
    check_hex_codec(@"7f");
    check_hex_codec(@"80");
    check_hex_codec(@"ff");
    check_hex_codec(@"0001");
    check_hex_codec(@"fffe");
    check_hex_codec(@"0123456789abcdef");
    check_hex_codec(@"fedcba9876543210");
}

#[test]
fn test_lexicographical_cmp() {
    // 1. Empty arrays (Equal)
    assert!(ArrayTrait::<u8>::new() == ArrayTrait::<u8>::new());

    // 2. Left is empty, right is non-empty (Less)
    assert!(array![] < array![0_u8]);
    assert!(array![] < array![1_u8, 2_u8, 3_u8]);

    // 3. Right is empty, left is non-empty (Greater)
    assert!(array![0_u8] > array![]);
    assert!(array![1_u8, 2_u8, 3_u8] > array![]);

    // 4. Identical arrays (Equal)
    assert!(array![0_u8] == array![0_u8]);
    assert!(array![1_u8, 2_u8, 3_u8] == array![1_u8, 2_u8, 3_u8]);

    // 5. Left is lexicographically smaller
    assert!(array![0_u8] < array![1_u8]);
    assert!(array![1_u8, 2_u8] < array![1_u8, 3_u8]);
    assert!(array![1_u8, 2_u8, 3_u8] < array![1_u8, 2_u8, 4_u8]);

    // 6. Right is lexicographically smaller
    assert!(array![1_u8] > array![0_u8]);
    assert!(array![1_u8, 3_u8] > array![1_u8, 2_u8]);
    assert!(array![1_u8, 2_u8, 4_u8] > array![1_u8, 2_u8, 3_u8]);

    // 7. One array is a strict prefix of the other (Shorter is Less)
    assert!(array![1_u8, 2_u8] < array![1_u8, 2_u8, 3_u8]);
    assert!(array![1_u8, 2_u8, 3_u8] > array![1_u8, 2_u8]);

    // 8. Comparing large arrays with a single difference at the end
    assert!(array![1_u8, 2_u8, 3_u8, 4_u8] < array![1_u8, 2_u8, 3_u8, 5_u8]);
    assert!(array![1_u8, 2_u8, 3_u8, 5_u8] > array![1_u8, 2_u8, 3_u8, 4_u8]);

    // 9. Arrays of different lengths but with the same starting elements
    assert!(array![1_u8, 2_u8, 3_u8] < array![1_u8, 2_u8, 3_u8, 0_u8]);
    assert!(array![1_u8, 2_u8, 3_u8, 0_u8] > array![1_u8, 2_u8, 3_u8]);

    /// 10. Arrays of different lenghts but the longer one is smaller
    assert!(array![0_u8, 1_u8, 2_u8, 5_u8, 10_u8] < array![0_u8, 1_u8, 3_u8, 4_u8]);

    // 11. Arrays with different leading zeros
    assert!(array![0_u8, 1_u8, 2_u8] < array![1_u8, 2_u8, 3_u8]);
    assert!(array![1_u8, 2_u8, 3_u8] > array![0_u8, 1_u8, 2_u8]);
}

#[test]
fn test_wasm_id_felt252_to_u8_array() {
    let raw_value: felt252 = 3820028427552332600290323295860;
    let converted = felt252_to_u8_array(raw_value);
    let expected: ByteArray = "07-tendermint";
    assert(converted == expected, 'failed felt252 to ByteArray');
}

#[test]
fn test_tendermint_id_felt252_to_u8_array() {
    let raw_value: felt252 = 13572566809670509;
    let converted = felt252_to_u8_array(raw_value);
    let expected: ByteArray = "08-wasm";
    assert(converted == expected, 'failed felt252 to ByteArray');
}
