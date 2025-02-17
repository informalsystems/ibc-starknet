use ibc::core::channel::types::commitment::compute_packet_commitment;
use ibc::core::channel::types::timeout::{TimeoutHeight, TimeoutTimestamp};
use ibc::core::client::types::Height;
use ibc::primitives::Timestamp;

#[test]
fn test_cairo_packet_commitment() {
    // https://github.com/informalsystems/ibc-starknet/blob/ce122321883d6dbceb89f1bde3521f9112284a8e/cairo-contracts/packages/apps/src/transfer/types.cairo#L307
    // https://github.com/informalsystems/ibc-starknet/blob/ce122321883d6dbceb89f1bde3521f9112284a8e/cairo-contracts/packages/core/src/tests/commitment.cairo#L52-L58

    let timeout_height = TimeoutHeight::At(Height::new(0, 1000).expect("valid height"));
    let timeout_timestamp = TimeoutTimestamp::At(Timestamp::from_nanoseconds(1000 * 1_000_000_000));

    let packet_json_data: &str =
            "{\"denom\":\"0x49d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7\",\"amount\":\"100\",\"sender\":\"0x55534552\",\"receiver\":\"cosmos1wxeyh7zgn4tctjzs0vtqpc6p5cxq5t2muzl7ng\",\"memo\":\"\"}";

    let expected: Vec<u32> = [
        3066915652, 3854894583, 2733453543, 2666376403, 1143720361, 1661963047, 2055864332,
        3822377424,
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
