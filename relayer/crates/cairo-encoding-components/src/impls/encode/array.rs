use core::fmt::Debug;

use crate::traits::decode_mut::{CanDecodeMut, MutDecoder};
use crate::traits::encode_mut::{CanEncodeMut, MutEncoder};

pub struct EncodeArray;

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

impl<Encoding, Strategy, Value, const SIZE: usize> MutDecoder<Encoding, Strategy, [Value; SIZE]>
    for EncodeArray
where
    Encoding: CanDecodeMut<Strategy, Value>,
    Value: Debug,
{
    fn decode_mut(
        encoding: &Encoding,
        buffer: &mut Encoding::DecodeBuffer,
    ) -> Result<[Value; SIZE], Encoding::Error> {
        let mut out = Vec::with_capacity(SIZE);

        for _ in 0..SIZE {
            out.push(encoding.decode_mut(buffer)?);
        }

        Ok(out.try_into().unwrap())
    }
}
