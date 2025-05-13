use std::string::FromUtf8Error;

use hermes_encoding_components::traits::{
    HasDecodeBufferType, MutDecoder, MutDecoderComponent, MutEncoderComponent,
};
use hermes_prelude::*;

use crate::impls::EncodeByteArray;

pub struct EncodeUtf8String;

delegate_components! {
    EncodeUtf8String {
        MutEncoderComponent: EncodeByteArray,
    }
}

#[cgp_provider(MutDecoderComponent)]
impl<Encoding, Strategy> MutDecoder<Encoding, Strategy, String> for EncodeUtf8String
where
    EncodeByteArray: MutDecoder<Encoding, Strategy, Vec<u8>>,
    Encoding: HasDecodeBufferType + CanRaiseAsyncError<FromUtf8Error>,
{
    fn decode_mut(
        encoding: &Encoding,
        buffer: &mut Encoding::DecodeBuffer<'_>,
    ) -> Result<String, Encoding::Error> {
        let bytes = EncodeByteArray::decode_mut(encoding, buffer)?;

        let string = String::from_utf8(bytes).map_err(Encoding::raise_error)?;

        Ok(string)
    }
}
