use crate::traits::decode_mut::{CanDecodeMut, MutDecoder};
use crate::traits::encode_mut::{CanEncodeMut, MutEncoder};
use crate::types::tagged::Tagged;

pub struct EncodeTagged;

impl<Encoding, Strategy, Tag, Value> MutEncoder<Encoding, Strategy, Tagged<Tag, Value>>
    for EncodeTagged
where
    Encoding: CanEncodeMut<Strategy, Value>,
{
    fn encode_mut(
        encoding: &Encoding,
        value: &Tagged<Tag, Value>,
        buffer: &mut Encoding::EncodeBuffer,
    ) -> Result<(), Encoding::Error> {
        encoding.encode_mut(&value.value, buffer)
    }
}

impl<Encoding, Strategy, Tag, Value> MutDecoder<Encoding, Strategy, Tagged<Tag, Value>>
    for EncodeTagged
where
    Encoding: CanDecodeMut<Strategy, Value>,
{
    fn decode_mut(
        encoding: &Encoding,
        buffer: &mut Encoding::DecodeBuffer<'_>,
    ) -> Result<Tagged<Tag, Value>, Encoding::Error> {
        let value = encoding.decode_mut(buffer)?;
        Ok(value.into())
    }
}
