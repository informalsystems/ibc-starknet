use starknet::core::types::U256;

use crate::traits::decode_mut::{CanDecodeMut, MutDecoder};
use crate::traits::encode_mut::{CanEncodeMut, MutEncoder};

pub struct EncodeU256;

impl<Encoding, Strategy> MutEncoder<Encoding, Strategy, U256> for EncodeU256
where
    Encoding: CanEncodeMut<Strategy, u128>,
{
    fn encode_mut(
        encoding: &Encoding,
        value: &U256,
        buffer: &mut Encoding::EncodeBuffer,
    ) -> Result<(), Encoding::Error> {
        encoding.encode_mut(&value.low(), buffer)?;
        encoding.encode_mut(&value.high(), buffer)?;

        Ok(())
    }
}

impl<Encoding, Strategy> MutDecoder<Encoding, Strategy, U256> for EncodeU256
where
    Encoding: CanDecodeMut<Strategy, u128>,
{
    fn decode_mut(
        encoding: &Encoding,
        buffer: &mut Encoding::DecodeBuffer<'_>,
    ) -> Result<U256, Encoding::Error> {
        let low = encoding.decode_mut(buffer)?;
        let high = encoding.decode_mut(buffer)?;

        Ok(U256::from_words(low, high))
    }
}
