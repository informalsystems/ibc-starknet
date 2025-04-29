use hermes_encoding_components::traits::{
    CanDecodeMut, CanEncodeMut, MutDecoder, MutDecoderComponent, MutEncoder, MutEncoderComponent,
};
use hermes_prelude::*;

pub struct EncodeList;

#[cgp_provider(MutEncoderComponent)]
impl<Encoding, Strategy, Value> MutEncoder<Encoding, Strategy, Vec<Value>> for EncodeList
where
    Encoding: CanEncodeMut<Strategy, Value> + CanEncodeMut<Strategy, usize>,
{
    fn encode_mut(
        encoding: &Encoding,
        value: &Vec<Value>,
        buffer: &mut Encoding::EncodeBuffer,
    ) -> Result<(), Encoding::Error> {
        encoding.encode_mut(&value.len(), buffer)?;

        for item in value.iter() {
            encoding.encode_mut(item, buffer)?;
        }

        Ok(())
    }
}

#[cgp_provider(MutDecoderComponent)]
impl<Encoding, Strategy, Value> MutDecoder<Encoding, Strategy, Vec<Value>> for EncodeList
where
    Encoding: CanDecodeMut<Strategy, Value> + CanDecodeMut<Strategy, usize>,
{
    fn decode_mut(
        encoding: &Encoding,
        buffer: &mut Encoding::DecodeBuffer<'_>,
    ) -> Result<Vec<Value>, Encoding::Error> {
        let count: usize = encoding.decode_mut(buffer)?;

        let mut out = Vec::with_capacity(count);

        for _ in 0..count {
            let value: Value = encoding.decode_mut(buffer)?;
            out.push(value);
        }

        Ok(out)
    }
}
