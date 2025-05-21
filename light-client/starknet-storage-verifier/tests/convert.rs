use starknet_macros::felt;
use starknet_storage_verifier::value::convert_storage_value;

#[test]
fn test_convert_storage_value_next_ack() {
    let path = "nextSequenceAck/ports/transfer/channels/channel-0";
    let converted_value = convert_storage_value(path);
    let expected_converted_value =
        felt!("0x5e66359665503ac3e2b1ba1f501b205eafb6fc8245b150b582d211be984809");
    assert_eq!(expected_converted_value, converted_value,);
}

#[test]
fn test_convert_storage_value_next_send() {
    let path = "nextSequenceSend/ports/transfer/channels/channel-0";
    let converted_value = convert_storage_value(path);
    let expected_converted_value =
        felt!("0x482fe9fb522fa0612fba38b07450f64552e45c3bf8d7bc016750a20661d0a32");
    assert_eq!(expected_converted_value, converted_value,);
}

#[test]
fn test_convert_storage_value_next_recv() {
    let path = "nextSequenceRecv/ports/transfer/channels/channel-0";
    let converted_value = convert_storage_value(path);
    let expected_converted_value =
        felt!("0x2c5d7959850dd27e4c48418937dc6111ab8296eb7cd11f4f1fada8fc682b7a6");
    assert_eq!(expected_converted_value, converted_value,);
}

#[test]
fn test_convert_storage_value_commitment() {
    let path = "commitments/ports/transfer/channels/channel-0/sequences/1";
    let converted_value = convert_storage_value(path);
    let expected_converted_value =
        felt!("0x4ebb4e50079c5a8fb2c3f930140fe962a536a04de1b497096c030c4247e185a");
    assert_eq!(expected_converted_value, converted_value,);
}

#[test]
fn test_convert_storage_value_ack() {
    let path = "acks/ports/transfer/channels/channel-0/sequences/1";
    let converted_value = convert_storage_value(path);
    let expected_converted_value =
        felt!("0x74b1f690db8f9af2955e740d0780847f3c32f67ed69a896368dcf37982aefb2");
    assert_eq!(expected_converted_value, converted_value,);
}

#[test]
fn test_convert_storage_value_receipt() {
    let path = "receipts/ports/transfer/channels/channel-0/sequences/1";
    let converted_value = convert_storage_value(path);
    let expected_converted_value =
        felt!("0x73e8c053278111020f75cbc1e556b2d03ea598b19d7748535a6fb7054c51d17");
    assert_eq!(expected_converted_value, converted_value,);
}

#[test]
fn test_convert_storage_value_channel_end() {
    let path = "channelEnds/ports/transfer/channels/channel-0";
    let converted_value = convert_storage_value(path);
    let expected_converted_value =
        felt!("0x71e478d871b699d378f44c8a63e3f52673593fd7557b8485f3046042088fa35");
    assert_eq!(expected_converted_value, converted_value,);
}
