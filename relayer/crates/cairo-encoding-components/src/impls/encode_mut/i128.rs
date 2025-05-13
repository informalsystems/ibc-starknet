use hermes_encoding_components::traits::{
    CanDecodeMut, CanEncodeMut, MutDecoder, MutDecoderComponent, MutEncoder, MutEncoderComponent,
};
use hermes_prelude::*;
use starknet::core::types::Felt;

pub struct EncodeI128;

#[cgp_provider(MutEncoderComponent)]
impl<Encoding, Strategy> MutEncoder<Encoding, Strategy, i128> for EncodeI128
where
    Encoding: CanEncodeMut<Strategy, Felt>,
{
    fn encode_mut(
        encoding: &Encoding,
        value: &i128,
        buffer: &mut Encoding::EncodeBuffer,
    ) -> Result<(), Encoding::Error> {
        let felt = i128_to_felt(*value);
        encoding.encode_mut(&felt, buffer)
    }
}

#[cgp_provider(MutDecoderComponent)]
impl<Encoding, Strategy> MutDecoder<Encoding, Strategy, i128> for EncodeI128
where
    Encoding: CanDecodeMut<Strategy, Felt>,
{
    fn decode_mut(
        encoding: &Encoding,
        buffer: &mut Encoding::DecodeBuffer<'_>,
    ) -> Result<i128, Encoding::Error> {
        let felt = encoding.decode_mut(buffer)?;
        let value = felt_to_i128(felt);

        Ok(value)
    }
}

pub fn i128_to_felt(value: i128) -> Felt {
    if value >= 0 {
        Felt::from(value as u128)
    } else {
        // Negative numbers are represented as:
        // StarkPrime + value == Felt::MAX + 1 + value
        let mut rt = Felt::MAX;
        rt -= Felt::from(value.unsigned_abs());
        rt += Felt::ONE;
        rt
    }
}

pub fn felt_to_i128(felt: Felt) -> i128 {
    if felt <= i128::MAX.into() {
        felt.try_into().unwrap()
    } else {
        // Negative numbers are represented as:
        // StarkPrime + value == Felt::MAX + 1 + value
        let mut rt = Felt::MAX;
        rt -= felt;
        rt += Felt::ONE;
        -(i128::try_from(rt).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_i128_to_felt() {
        assert_eq!(i128_to_felt(0), Felt::ZERO);
        assert_eq!(i128_to_felt(1), Felt::ONE);
        assert_eq!(i128_to_felt(-1), Felt::MAX);
        assert_eq!(i128_to_felt(-2), Felt::MAX - 1);
        assert_eq!(i128_to_felt(-10), Felt::MAX - 9);
    }

    #[test]
    fn test_felt_to_i128() {
        assert_eq!(felt_to_i128(Felt::ZERO), 0);
        assert_eq!(felt_to_i128(Felt::ONE), 1);
        assert_eq!(felt_to_i128(Felt::MAX), -1);
        assert_eq!(felt_to_i128(Felt::MAX - 1), -2);
        assert_eq!(felt_to_i128(Felt::MAX - 9), -10);
    }
}
