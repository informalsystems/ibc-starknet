use starknet_ibc_apps::transfer::SUCCESS_ACK;

fn assert_eq_success_ack(case: Array<u8>) {
    assert_eq!(case.into(), SUCCESS_ACK());
}

#[test]
fn test_success_ack_ok() {
    let case = array![123, 34, 114, 101, 115, 117, 108, 116, 34, 58, 34, 65, 81, 61, 61, 34, 125];
    assert_eq_success_ack(case);
}

#[test]
#[should_panic]
fn test_success_ack_with_leading_zero() {
    let case = array![
        123, 34, 114, 101, 115, 117, 108, 116, 34, 58, 34, 65, 81, 61, 61, 34, 125, 0
    ];
    assert_eq_success_ack(case);
}

#[test]
#[should_panic]
fn test_success_ack_with_different_content() {
    let case = array![123, 34, 114, 101, 115, 117, 108, 116, 34, 58, 34, 65, 81, 61, 61, 34, 126];
    assert_eq_success_ack(case);
}

#[test]
#[should_panic]
fn test_success_ack_with_missing_char() {
    let case = array![123, 34, 114, 101, 115, 117, 108, 116, 34, 58, 34, 65, 81, 61, 61, 34];
    assert_eq_success_ack(case);
}

#[test]
#[should_panic]
fn test_success_ack_with_zero() {
    let case = array![0];
    assert_eq_success_ack(case);
}

#[test]
#[should_panic]
fn test_success_ack_with_empty_array() {
    let case = array![];
    assert_eq_success_ack(case);
}
