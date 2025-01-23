use ics23::{array_u8_into_array_u32, u64_into_array_u32, encode_hex};

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
        123, 34, 114, 101, 115, 117, 108, 116, 34, 58, 34, 65, 81, 61, 61, 34, 125, 126
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

#[test]
fn test_encode_hex() {
    assert_eq!("", encode_hex(array![]));
    assert_eq!("00", encode_hex(array![0]));
    assert_eq!("01", encode_hex(array![1]));
    assert_eq!("7f", encode_hex(array![127]));
    assert_eq!("80", encode_hex(array![128]));
    assert_eq!("ff", encode_hex(array![255]));
    assert_eq!("0001", encode_hex(array![0, 1]));
    assert_eq!("fffe", encode_hex(array![255, 254]));
    assert_eq!("0123456789abcdef", encode_hex(array![1, 35, 69, 103, 137, 171, 205, 239]));
}
