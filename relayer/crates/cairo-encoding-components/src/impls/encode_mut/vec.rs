use crate::traits::decode_mut::{CanDecodeMut, MutDecoder};
use crate::traits::encode_mut::{CanEncodeMut, MutEncoder};

pub struct EncodeVec;

impl<Encoding, Strategy, Value> MutEncoder<Encoding, Strategy, Vec<Value>> for EncodeVec
where
    Encoding: CanEncodeMut<Strategy, Value>,
{
    fn encode_mut(
        encoding: &Encoding,
        value: &Vec<Value>,
        buffer: &mut Encoding::EncodeBuffer,
    ) -> Result<(), Encoding::Error> {
        for item in value.iter() {
            encoding.encode_mut(item, buffer)?;
        }

        Ok(())
    }
}

impl<Encoding, Strategy, Value> MutDecoder<Encoding, Strategy, Vec<Value>> for EncodeVec
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
