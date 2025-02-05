use starknet_ibc_core::commitment::{
    u64_into_array_u32, array_u8_into_array_u32, compute_ack_commitment
};
use starknet_ibc_testkit::dummies::{ERC20, PACKET_COMMITMENT_ON_SN};

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

// Snapshot test to ensure the computation of packet commitment stays
// consistent.
#[test]
fn test_compute_packet_commitment() {
    let commitment = PACKET_COMMITMENT_ON_SN(ERC20());
    let expected: [u32; 8] = [
        1561496803, 591083406, 1958596266, 2480824962, 846563094, 2634790765, 145282158, 2139799705
    ];
    assert_eq!(commitment, expected.into());
}

// Snapshot test to ensure the computation of ack commitment stays consistent.
#[test]
fn test_compute_ack_commitment() {
    let ack = array![123, 34, 114, 101, 115, 117, 108, 116, 34, 58, 34, 65, 81, 61, 61, 34, 125];
    let commitment = compute_ack_commitment(ack.into());
    let expected: [u32; 8] = [
        150427006, 3575129854, 416826642, 3206868085, 2022319, 555983991, 3748831401, 4083419772
    ];
    assert_eq!(commitment, expected.into());
}
