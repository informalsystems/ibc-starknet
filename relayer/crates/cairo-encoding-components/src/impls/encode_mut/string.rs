use std::string::FromUtf8Error;

use cgp::prelude::*;
use hermes_encoding_components::traits::decode_mut::{
    CanDecodeMut, MutDecoder, MutDecoderComponent,
};
use hermes_encoding_components::traits::encode_mut::MutEncoderComponent;

use crate::impls::encode_mut::byte_array::EncodeByteArray;

pub struct EncodeUtf8String;

impl DelegateComponent<MutEncoderComponent> for EncodeUtf8String {
    type Delegate = EncodeByteArray;
}

#[cgp_provider(MutDecoderComponent)]
impl<Encoding, Strategy> MutDecoder<Encoding, Strategy, String> for EncodeUtf8String
where
    Encoding: CanDecodeMut<Strategy, Vec<u8>> + CanRaiseAsyncError<FromUtf8Error>,
{
    fn decode_mut(
        encoding: &Encoding,
        buffer: &mut Encoding::DecodeBuffer<'_>,
    ) -> Result<String, Encoding::Error> {
        let bytes = encoding.decode_mut(buffer)?;

        let string = String::from_utf8(bytes).map_err(Encoding::raise_error)?;

        Ok(string)
    }
}
