use hermes_encoding_components::traits::decoder::Decoder;
use hermes_encoding_components::traits::encoder::Encoder;

use crate::traits::decode_mut::CanDecodeMut;
use crate::traits::encode_mut::CanEncodeMut;

pub struct EncodeWithMutBuffer;

impl<Encoding, Strategy, Value> Encoder<Encoding, Strategy, Value> for EncodeWithMutBuffer
where
    Encoding: CanEncodeMut<Strategy, Value>,
{
    fn encode(encoding: &Encoding, value: &Value) -> Result<Encoding::Encoded, Encoding::Error> {
        let mut buffer = Default::default();

        encoding.encode_mut(value, &mut buffer)?;

        Ok(Encoding::to_encoded(buffer))
    }
}

impl<Encoding, Strategy, Value> Decoder<Encoding, Strategy, Value> for EncodeWithMutBuffer
where
    Encoding: CanDecodeMut<Strategy, Value>,
{
    fn decode(encoding: &Encoding, encoded: &Encoding::Encoded) -> Result<Value, Encoding::Error> {
        let mut buffer = Encoding::from_encoded(encoded);

        encoding.decode_mut(&mut buffer)
    }
}
