use core::fmt::Debug;

use hermes_encoding_components::traits::{
    CanDecodeMut, CanEncodeMut, MutDecoder, MutDecoderComponent, MutEncoder, MutEncoderComponent,
};
use hermes_prelude::*;

pub struct EncodeArray;

#[cgp_provider(MutEncoderComponent)]
impl<Encoding, Strategy, Value, const SIZE: usize> MutEncoder<Encoding, Strategy, [Value; SIZE]>
    for EncodeArray
where
    Encoding: CanEncodeMut<Strategy, Value>,
{
    fn encode_mut(
        encoding: &Encoding,
        value: &[Value; SIZE],
        buffer: &mut Encoding::EncodeBuffer,
    ) -> Result<(), Encoding::Error> {
        for item in value.iter() {
            encoding.encode_mut(item, buffer)?;
        }

        Ok(())
    }
}

#[cgp_provider(MutDecoderComponent)]
impl<Encoding, Strategy, Value, const SIZE: usize> MutDecoder<Encoding, Strategy, [Value; SIZE]>
    for EncodeArray
where
    Encoding: CanDecodeMut<Strategy, Value>,
    Value: Debug,
{
    fn decode_mut(
        encoding: &Encoding,
        buffer: &mut Encoding::DecodeBuffer<'_>,
    ) -> Result<[Value; SIZE], Encoding::Error> {
        let mut out = Vec::with_capacity(SIZE);

        for _ in 0..SIZE {
            out.push(encoding.decode_mut(buffer)?);
        }

        Ok(out.try_into().unwrap())
    }
}
