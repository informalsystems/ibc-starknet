use crate::traits::decode_mut::CanDecodeMut;
use crate::traits::encode_mut::CanEncodeMut;

pub trait CanEncodeAndDecodeMut<Strategy, Value>:
    CanEncodeMut<Strategy, Value> + CanDecodeMut<Strategy, Value>
{
}

impl<Encoding, Strategy, Value> CanEncodeAndDecodeMut<Strategy, Value> for Encoding where
    Encoding: CanEncodeMut<Strategy, Value> + CanDecodeMut<Strategy, Value>
{
}
