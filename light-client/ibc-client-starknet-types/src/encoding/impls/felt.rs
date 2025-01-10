use core::str::FromStr;

use cgp::prelude::{CanRaiseError, HasErrorType};
use hermes_encoding_components::traits::decode_mut::{CanDecodeMut, MutDecoder};
use hermes_encoding_components::traits::encode_mut::{CanEncodeMut, MutEncoder};
use hermes_encoding_components::traits::types::encode_buffer::HasEncodeBufferType;
use starknet::core::types::Felt;

pub struct EncodeFelt;

impl<Encoding, Strategy> MutEncoder<Encoding, Strategy, Felt> for EncodeFelt
where
    Encoding: HasEncodeBufferType + CanEncodeMut<Strategy, String> + HasErrorType,
{
    fn encode_mut(
        encoding: &Encoding,
        felt: &Felt,
        buffer: &mut Encoding::EncodeBuffer,
    ) -> Result<(), Encoding::Error> {
        encoding.encode_mut(&felt.to_string(), buffer)?;

        Ok(())
    }
}

impl<Encoding, Strategy> MutDecoder<Encoding, Strategy, Felt> for EncodeFelt
where
    Encoding: CanDecodeMut<Strategy, String> + CanRaiseError<&'static str>,
{
    fn decode_mut(
        encoding: &Encoding,
        buffer: &mut Encoding::DecodeBuffer<'_>,
    ) -> Result<Felt, Encoding::Error> {
        let felt_str = encoding.decode_mut(buffer)?;

        Felt::from_str(&felt_str).map_err(|_| Encoding::raise_error("invalid felt"))
    }
}
