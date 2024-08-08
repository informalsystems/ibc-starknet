use cgp_core::prelude::DelegateComponent;
use starknet::core::types::Felt;

use crate::impls::encode::from_felt::EncodeFromFelt;
use crate::traits::decode_mut::{CanDecodeMut, MutDecoder};
use crate::traits::encode_mut::MutEncoderComponent;

pub struct EncodeU128;

impl<Encoding, Strategy> MutDecoder<Encoding, Strategy, u128> for EncodeU128
where
    Encoding: CanDecodeMut<Strategy, Felt>,
{
    fn decode_mut(
        encoding: &Encoding,
        buffer: &mut Encoding::DecodeBuffer<'_>,
    ) -> Result<u128, Encoding::Error> {
        let felt = encoding.decode_mut(buffer)?;
        let bytes = &felt.to_bytes_be()[16..];
        let value = u128::from_be_bytes(bytes.try_into().unwrap());

        Ok(value)
    }
}

impl DelegateComponent<MutEncoderComponent> for EncodeU128 {
    type Delegate = EncodeFromFelt;
}
