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
        timestamp: &Timestamp,
        buffer: &mut Encoding::EncodeBuffer,
    ) -> Result<(), Encoding::Error> {
        let unix_timestamp = timestamp.nanoseconds() / 1_000_000_000;
        encoding.encode_mut(&product![unix_timestamp], buffer)?;
        Ok(())
    }
}

impl<Encoding, Strategy> MutDecoder<Encoding, Strategy, Timestamp> for EncodeTimestamp
where
    Encoding: CanDecodeMut<Strategy, Product![u64]> + CanRaiseError<&'static str>,
{
    fn decode_mut<'a>(
        encoding: &Encoding,
        buffer: &mut Encoding::DecodeBuffer<'a>,
    ) -> Result<Timestamp, Encoding::Error> {
        let product![unix_timestamp] = encoding.decode_mut(buffer)?;
        Timestamp::from_unix_timestamp(unix_timestamp, 0)
            .map_err(|_| Encoding::raise_error("invalid timestamp"))
    }
}
