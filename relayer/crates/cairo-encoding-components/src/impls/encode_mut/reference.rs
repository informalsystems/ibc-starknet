use std::ops::Deref;

use crate::traits::encode_mut::{CanEncodeMut, MutEncoder};

pub struct EncodeDeref;

impl<Encoding, Strategy, Value> MutEncoder<Encoding, Strategy, Value> for EncodeDeref
where
    Encoding: CanEncodeMut<Strategy, Value::Target>,
    Value: Deref,
    Value::Target: Sized,
{
    fn encode_mut(
        encoding: &Encoding,
        value: &Value,
        buffer: &mut Encoding::EncodeBuffer,
    ) -> Result<(), Encoding::Error> {
        encoding.encode_mut(value.deref(), buffer)
    }
}
