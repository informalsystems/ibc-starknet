use std::string::FromUtf8Error;

use cgp_core::error::CanRaiseError;
use cgp_core::prelude::DelegateComponent;

use crate::impls::encode_mut::byte_array::EncodeByteArray;
use crate::traits::decode_mut::{CanDecodeMut, MutDecoder};
use crate::traits::encode_mut::MutEncoderComponent;

pub struct EncodeUtf8String;

impl DelegateComponent<MutEncoderComponent> for EncodeUtf8String {
    type Delegate = EncodeByteArray;
}

impl<Encoding, Strategy> MutDecoder<Encoding, Strategy, String> for EncodeUtf8String
where
    Encoding: CanDecodeMut<Strategy, Vec<u8>> + CanRaiseError<FromUtf8Error>,
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