use hermes_encoding_components::traits::decoder::Decoder;
use hermes_encoding_components::traits::encoder::Encoder;

use crate::impls::encode_mut::end::DecodeEnd;
use crate::traits::decode_mut::{CanDecodeMut, MutDecoder};
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
    DecodeEnd: MutDecoder<Encoding, Strategy, ()>,
{
    fn decode(encoding: &Encoding, encoded: &Encoding::Encoded) -> Result<Value, Encoding::Error> {
        let mut buffer = Encoding::from_encoded(encoded);

        let value = encoding.decode_mut(&mut buffer)?;
        DecodeEnd::decode_mut(encoding, &mut buffer)?;

        Ok(value)
    }
}
