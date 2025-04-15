const FELT_MAX: felt252 = 0x800000000000011000000000000000000000000000000000000000000000000;

// tests the logic at Rust side
// https://github.com/informalsystems/ibc-starknet/blob/main/relayer/crates/cairo-encoding-components/src/impls/encode_mut/i128.rs

#[test]
#[fuzzer]
fn test_negative_integer(x: u64) {
    if x > 0 {
        let num: i128 = x.into() * -1;

        let mut felts: Array<felt252> = array![];
        Serde::<i128>::serialize(@num, ref felts);

        assert_eq!(felts.len(), 1, "felts length mismatch");

        // MAX felt252

        let result = FELT_MAX - ((-num).try_into().unwrap() - 1);

        assert_eq!(felts[0], @result, "wrong serialized value");

        let mut span = felts.span();

        let num2 = Serde::<i128>::deserialize(ref span).unwrap();

        assert_eq!(num, num2, "wrong deserialized value");
    }
}
