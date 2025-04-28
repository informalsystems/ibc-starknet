use core::fmt::Debug;

use cgp::prelude::*;
use hermes_encoding_components::traits::{
    CanDecodeMut, CanEncodeMut, MutDecoder, MutDecoderComponent, MutEncoder, MutEncoderComponent,
};
use starknet::core::types::Felt;

pub struct EncodeBool;

pub struct DecodeBoolError {
    pub felt: Felt,
}

#[cgp_provider(MutEncoderComponent)]
impl<Encoding, Strategy> MutEncoder<Encoding, Strategy, bool> for EncodeBool
where
    Encoding: CanEncodeMut<Strategy, Felt>,
{
    fn encode_mut(
        encoding: &Encoding,
        flag: &bool,
        buffer: &mut Encoding::EncodeBuffer,
    ) -> Result<(), Encoding::Error> {
        let felt = if *flag { Felt::ONE } else { Felt::ZERO };
        encoding.encode_mut(&felt, buffer)?;

        Ok(())
    }
}

#[cgp_provider(MutDecoderComponent)]
impl<Encoding, Strategy> MutDecoder<Encoding, Strategy, bool> for EncodeBool
where
    Encoding: CanDecodeMut<Strategy, Felt> + CanRaiseAsyncError<DecodeBoolError>,
{
    fn decode_mut(
        encoding: &Encoding,
        buffer: &mut Encoding::DecodeBuffer<'_>,
    ) -> Result<bool, Encoding::Error> {
        let felt = encoding.decode_mut(buffer)?;

        let flag = if felt == Felt::ONE {
            true
        } else if felt == Felt::ZERO {
            false
        } else {
            return Err(Encoding::raise_error(DecodeBoolError { felt }));
        };

        Ok(flag)
    }
}

impl Debug for DecodeBoolError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "failed to decode bool from felt. expected felt value to be either zero or one, but got: {:?}", self.felt)
    }
}
