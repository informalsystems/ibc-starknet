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
            "{\"denom\":\"2087021424722619777119509474943472645767659996348769578120564519014510906823\",\"amount\":\"100\",\"sender\":\"1431520594\",\"receiver\":\"cosmos1wxeyh7zgn4tctjzs0vtqpc6p5cxq5t2muzl7ng\",\"memo\":\"\"}";

    let expected: Vec<u32> = [
        3458244073, 1576048754, 4210798310, 1002247062, 2365181318, 2763927782, 545147151,
        944653547,
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
