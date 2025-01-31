use cgp::prelude::*;
use hermes_encoding_components::traits::decode_mut::{CanDecodeMut, MutDecoder};
use hermes_encoding_components::traits::encode_mut::{CanEncodeMut, MutEncoder};
pub use ibc::primitives::Timestamp;

pub struct EncodeTimestamp;

impl<Encoding, Strategy> MutEncoder<Encoding, Strategy, Timestamp> for EncodeTimestamp
where
    Encoding: CanEncodeMut<Strategy, Product![u64]>,
{
    fn encode_mut(
        encoding: &Encoding,
        value: &Timestamp,
        buffer: &mut Encoding::EncodeBuffer,
    ) -> Result<(), Encoding::Error> {
        let unix_secs = value.nanoseconds() / 1_000_000_000;
        encoding.encode_mut(&product![unix_secs], buffer)?;
        Ok(())
    }
}

impl<Encoding, Strategy> MutDecoder<Encoding, Strategy, Timestamp> for EncodeTimestamp
where
    Encoding: CanDecodeMut<Strategy, Product![u64]>,
{
    fn decode_mut<'a>(
        encoding: &Encoding,
        buffer: &mut Encoding::DecodeBuffer<'a>,
    ) -> Result<Timestamp, Encoding::Error> {
        let product![unix_secs] = encoding.decode_mut(buffer)?;
        Ok(Timestamp::from_nanoseconds(unix_secs * 1_000_000_000))
    }
}
