use hermes_encoding_components::traits::{CanEncodeMut, MutEncoder, MutEncoderComponent};
use hermes_prelude::*;

pub struct EncodeIterator;

#[cgp_provider(MutEncoderComponent)]
impl<Encoding, Strategy, Value> MutEncoder<Encoding, Strategy, Value> for EncodeIterator
where
    Encoding: for<'a> CanEncodeMut<Strategy, <&'a Value as IntoIterator>::Item>,
    for<'a> &'a Value: IntoIterator,
{
    fn encode_mut(
        encoding: &Encoding,
        value: &Value,
        buffer: &mut Encoding::EncodeBuffer,
    ) -> Result<(), Encoding::Error> {
        for item in value.into_iter() {
            encoding.encode_mut(&item, buffer)?;
        }

        Ok(())
    }
}
