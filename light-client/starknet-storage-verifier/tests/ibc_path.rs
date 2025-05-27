use starknet_macros::felt;
use starknet_storage_verifier::ibc::ibc_path_to_storage_key;

#[test]
fn test_convert_storage_value_next_ack() {
    let path = "nextSequenceAck/ports/transfer/channels/channel-0";
    let converted_value = ibc_path_to_storage_key(path.parse().unwrap());
    let expected_converted_value =
        felt!("0x40e7d1b0eebf8ad90ff0971fef79ed56071951831a3b7a2f5468e2a4ffd13da");
    assert_eq!(expected_converted_value, converted_value,);
}

#[test]
fn test_convert_storage_value_next_send() {
    let path = "nextSequenceSend/ports/transfer/channels/channel-0";
    let converted_value = ibc_path_to_storage_key(path.parse().unwrap());
    let expected_converted_value =
        felt!("0x7d2dbb5e7da159de7ab5ca01fdfecf987653e835db548297e9837464a50a4a7");
    assert_eq!(expected_converted_value, converted_value,);
}

#[test]
fn test_convert_storage_value_next_recv() {
    let path = "nextSequenceRecv/ports/transfer/channels/channel-0";
    let converted_value = ibc_path_to_storage_key(path.parse().unwrap());
    let expected_converted_value =
        felt!("0x75bd43c16fefe6fd7486617a2ad6e8160c7255695caaf8dd0e363d1ebefddb5");
    assert_eq!(expected_converted_value, converted_value,);
}

#[test]
fn test_convert_storage_value_commitment() {
    let path = "commitments/ports/transfer/channels/channel-0/sequences/1";
    let converted_value = ibc_path_to_storage_key(path.parse().unwrap());
    let expected_converted_value =
        felt!("0x468b2a346069c05a210dcf2cb3ae6039eaeabd23123e56eea19cabcd13ec9c1");
    assert_eq!(expected_converted_value, converted_value,);
}

#[test]
fn test_convert_storage_value_ack() {
    let path = "acks/ports/transfer/channels/channel-0/sequences/1";
    let converted_value = ibc_path_to_storage_key(path.parse().unwrap());
    let expected_converted_value =
        felt!("0x50c97475781242a4c5c4196c7ed534148077f619c0e809bde0da57e70867f4e");
    assert_eq!(expected_converted_value, converted_value,);
}

#[test]
fn test_convert_storage_value_receipt() {
    let path = "receipts/ports/transfer/channels/channel-0/sequences/1";
    let converted_value = ibc_path_to_storage_key(path.parse().unwrap());
    let expected_converted_value =
        felt!("0x615f05024772c4656282e9886a3dab879d2a6e22073ca7db97f128f6462f4eb");
    assert_eq!(expected_converted_value, converted_value,);
}

#[test]
fn test_convert_storage_value_channel_end() {
    let path = "channelEnds/ports/transfer/channels/channel-0";
    let converted_value = ibc_path_to_storage_key(path.parse().unwrap());
    let expected_converted_value =
        felt!("0x1a7525b2bdb08c4cecfa8c56d2e75de5124e7572d4617233f972f8596ff4d87");
    assert_eq!(expected_converted_value, converted_value,);
}

#[test]
fn test_convert_storage_value_connection_end() {
    let path = "connections/connection-0";
    let converted_value = ibc_path_to_storage_key(path.parse().unwrap());
    let expected_converted_value =
        felt!("0x549d43c11501acf2519d28321102f75bb326edea3d7575f6ee27597fd346693");
    assert_eq!(expected_converted_value, converted_value,);
}
