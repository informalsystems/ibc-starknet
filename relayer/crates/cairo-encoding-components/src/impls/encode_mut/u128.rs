use cgp::prelude::*;
use hermes_encoding_components::traits::{
    CanDecodeMut, MutDecoder, MutDecoderComponent, MutEncoderComponent,
};
use starknet::core::types::Felt;

use crate::impls::encode_mut::from_felt::EncodeFromFelt;

pub struct EncodeU128;

#[cgp_provider(MutDecoderComponent)]
impl<Encoding, Strategy> MutDecoder<Encoding, Strategy, u128> for EncodeU128
where
    Encoding: CanDecodeMut<Strategy, Felt>,
{
    fn decode_mut(
        encoding: &Encoding,
        buffer: &mut Encoding::DecodeBuffer<'_>,
    ) -> Result<u128, Encoding::Error> {
        let felt = encoding.decode_mut(buffer)?;
        let value = felt_to_u128(felt);

        Ok(value)
    }
}

delegate_components! {
    EncodeU128 {
        MutEncoderComponent: EncodeFromFelt,
    }
}

pub fn felt_to_u128(felt: Felt) -> u128 {
    let bytes = &felt.to_bytes_be()[16..];
    u128::from_be_bytes(bytes.try_into().unwrap())
}
