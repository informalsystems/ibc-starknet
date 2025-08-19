use protobuf::primitives::array::BytesAsProtoMessage;
use protobuf::types::message::ProtoCodecImpl;

#[test]
fn test_bytes_as_proto_message() {
    let bytes: Array<u8> = array![0x12, 0x34, 0x56, 0x78, 0x90, 0xab, 0xcd, 0xef];
    let encoded = ProtoCodecImpl::encode(@bytes);
    let decoded = ProtoCodecImpl::decode(encoded.span()).unwrap();
    assert_eq!(bytes, decoded);
}

#[test]
fn test_bytes_as_proto_message_empty() {
    let bytes: Array<u8> = array![];
    let encoded = ProtoCodecImpl::encode(@bytes);
    let decoded = ProtoCodecImpl::decode(encoded.span()).unwrap();
    assert_eq!(bytes, decoded);
}

#[test]
fn test_bytes_as_proto_message_zero() {
    let bytes: Array<u8> = array![0x00];
    let encoded = ProtoCodecImpl::encode(@bytes);
    let decoded = ProtoCodecImpl::decode(encoded.span()).unwrap();
    assert_eq!(bytes, decoded);
}
