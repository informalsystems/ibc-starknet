use core::ops::Deref;

use hermes_encoding_components::traits::{CanEncodeMut, MutEncoder, MutEncoderComponent};
use hermes_prelude::*;

pub struct EncodeDeref;

#[cgp_provider(MutEncoderComponent)]
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
