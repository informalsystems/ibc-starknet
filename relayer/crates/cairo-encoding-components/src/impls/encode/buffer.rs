use cgp::prelude::*;
use hermes_encoding_components::traits::decode::{Decoder, DecoderComponent};
use hermes_encoding_components::traits::decode_mut::{CanDecodeMut, MutDecoder};
use hermes_encoding_components::traits::encode::{Encoder, EncoderComponent};
use hermes_encoding_components::traits::encode_mut::CanEncodeMut;
use hermes_encoding_components::traits::types::decode_buffer::CanBuildDecodeBuffer;
use hermes_encoding_components::traits::types::encode_buffer::CanFinalizedEncodeBuffer;

use crate::impls::encode_mut::end::DecodeEnd;

pub struct EncodeWithMutBuffer;

#[cgp_provider(EncoderComponent)]
impl<Encoding, Strategy, Value> Encoder<Encoding, Strategy, Value> for EncodeWithMutBuffer
where
    Encoding: CanEncodeMut<Strategy, Value> + CanFinalizedEncodeBuffer,
{
    fn encode(encoding: &Encoding, value: &Value) -> Result<Encoding::Encoded, Encoding::Error> {
        let mut buffer = Default::default();

        encoding.encode_mut(value, &mut buffer)?;

        Ok(Encoding::to_encoded(buffer))
    }
}

#[cgp_provider(DecoderComponent)]
impl<Encoding, Strategy, Value> Decoder<Encoding, Strategy, Value> for EncodeWithMutBuffer
where
    Encoding: CanDecodeMut<Strategy, Value> + CanBuildDecodeBuffer,
    DecodeEnd: MutDecoder<Encoding, Strategy, ()>,
{
    fn decode(encoding: &Encoding, encoded: &Encoding::Encoded) -> Result<Value, Encoding::Error> {
        let mut buffer = Encoding::from_encoded(encoded);

        let value = encoding.decode_mut(&mut buffer)?;
        DecodeEnd::decode_mut(encoding, &mut buffer)?;

        Ok(value)
    }
}
