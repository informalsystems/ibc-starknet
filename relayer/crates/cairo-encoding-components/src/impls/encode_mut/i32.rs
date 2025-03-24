use cgp::prelude::*;
use hermes_encoding_components::traits::decode_mut::{
    CanDecodeMut, MutDecoder, MutDecoderComponent,
};
use hermes_encoding_components::traits::encode_mut::MutEncoderComponent;
use starknet::core::types::Felt;

use crate::impls::encode_mut::from_felt::EncodeFromFelt;

pub struct EncodeI32;

#[cgp_provider(MutDecoderComponent)]
impl<Encoding, Strategy> MutDecoder<Encoding, Strategy, i32> for EncodeI32
where
    Encoding: CanDecodeMut<Strategy, Felt>,
{
    fn decode_mut(
        encoding: &Encoding,
        buffer: &mut Encoding::DecodeBuffer<'_>,
    ) -> Result<i32, Encoding::Error> {
        let felt = encoding.decode_mut(buffer)?;
        let value = felt_to_i32(felt);

        Ok(value)
    }
}

delegate_components! {
    EncodeI32 {
        MutEncoderComponent: EncodeFromFelt,
    }
}

pub fn felt_to_i32(felt: Felt) -> i32 {
    let bytes = &felt.to_bytes_be()[16..];
    i32::from_be_bytes(bytes.try_into().unwrap())
}
