use core::marker::PhantomData;

use crate::traits::decode_mut::{CanDecodeMut, MutDecoder};

pub struct DecodeFrom<Source>(pub PhantomData<Source>);

impl<Encoding, Strategy, Source, Value> MutDecoder<Encoding, Strategy, Value> for DecodeFrom<Source>
where
    Encoding: CanDecodeMut<Strategy, Source>,
    Value: From<Source>,
{
    fn decode_mut(
        encoding: &Encoding,
        buffer: &mut Encoding::DecodeBuffer<'_>,
    ) -> Result<Value, Encoding::Error> {
        let source = encoding.decode_mut(buffer)?;
        Ok(source.into())
    }
}
