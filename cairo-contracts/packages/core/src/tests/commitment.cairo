use starknet_ibc_core::commitment::compute_ack_commitment;
use starknet_ibc_testkit::dummies::{ERC20, PACKET_COMMITMENT_ON_SN};

// Snapshot test to ensure the computation of packet commitment stays
// consistent.
#[test]
fn test_compute_packet_commitment() {
    let commitment = PACKET_COMMITMENT_ON_SN(ERC20());
    let expected: [u32; 8] = [
        1561496803, 591083406, 1958596266, 2480824962, 846563094, 2634790765, 145282158, 2139799705,
    ];
    assert_eq!(commitment, expected.into());
}

// Snapshot test to ensure the computation of ack commitment stays consistent.
#[test]
fn test_compute_ack_commitment() {
    let ack = array![123, 34, 114, 101, 115, 117, 108, 116, 34, 58, 34, 65, 81, 61, 61, 34, 125];
    let commitment = compute_ack_commitment(ack.into());
    let expected: [u32; 8] = [
        150427006, 3575129854, 416826642, 3206868085, 2022319, 555983991, 3748831401, 4083419772,
    ];
    assert_eq!(commitment, expected.into());
}
