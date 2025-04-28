use cgp::prelude::*;
use hermes_core::encoding_components::traits::{
    CanDecodeMut, CanEncodeMut, MutDecoder, MutDecoderComponent, MutEncoder, MutEncoderComponent,
};
pub use ibc::primitives::Timestamp;

pub struct EncodeTimestamp;

#[cgp_provider(MutEncoderComponent)]
impl<Encoding, Strategy> MutEncoder<Encoding, Strategy, Timestamp> for EncodeTimestamp
where
    Encoding: CanEncodeMut<Strategy, Product![u64]>,
{
    fn encode_mut(
        encoding: &Encoding,
        value: &Timestamp,
        buffer: &mut Encoding::EncodeBuffer,
    ) -> Result<(), Encoding::Error> {
        let unix_nanos = value.nanoseconds();
        encoding.encode_mut(&product![unix_nanos], buffer)?;
        Ok(())
    }
}

#[cgp_provider(MutDecoderComponent)]
impl<Encoding, Strategy> MutDecoder<Encoding, Strategy, Timestamp> for EncodeTimestamp
where
    Encoding: CanDecodeMut<Strategy, Product![u64]>,
{
    fn decode_mut<'a>(
        encoding: &Encoding,
        buffer: &mut Encoding::DecodeBuffer<'a>,
    ) -> Result<Timestamp, Encoding::Error> {
        let product![unix_nanos] = encoding.decode_mut(buffer)?;
        Ok(Timestamp::from_nanoseconds(unix_nanos))
    }
}
