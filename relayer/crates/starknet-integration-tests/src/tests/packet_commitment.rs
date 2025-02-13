use ibc::core::channel::types::commitment::compute_packet_commitment;
use ibc::core::channel::types::timeout::{TimeoutHeight, TimeoutTimestamp};
use ibc::core::client::types::Height;
use ibc::primitives::Timestamp;

#[test]
fn test_cairo_packet_commitment() {
    // https://github.com/informalsystems/ibc-starknet/blob/7967c8045ed6b4453030e01d0df12c47c2d77b37/cairo-contracts/packages/apps/src/transfer/types.cairo#L307
    // https://github.com/informalsystems/ibc-starknet/blob/7967c8045ed6b4453030e01d0df12c47c2d77b37/cairo-contracts/packages/core/src/tests/commitment.cairo#L52-L58

    let timeout_height = TimeoutHeight::At(Height::new(0, 1000).expect("valid height"));
    let timeout_timestamp = TimeoutTimestamp::At(Timestamp::from_nanoseconds(1000 * 1_000_000_000));

    let packet_json_data: &str =
            "{\"denom\":\"0x49d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7\",\"amount\":\"100\",\"sender\":\"0x55534552\",\"receiver\":\"cosmos1wxeyh7zgn4tctjzs0vtqpc6p5cxq5t2muzl7ng\",\"memo\":\"\"}";

    let expected: Vec<u32> = [
        1561496803, 591083406, 1958596266, 2480824962, 846563094, 2634790765, 145282158, 2139799705,
    ]
    .to_vec();

    let actual = compute_packet_commitment(
        packet_json_data.as_ref(),
        &timeout_height,
        &timeout_timestamp,
    )
    .into_vec()
    .chunks(4)
    .map(|chunk| {
        let mut bytes = [0u8; 4];
        bytes.copy_from_slice(chunk);
        u32::from_be_bytes(bytes)
    })
    .collect::<Vec<_>>();

    assert_eq!(expected, actual);
}
