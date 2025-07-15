use alloc::vec::Vec;

use hermes_encoding_components::traits::{
    HasDecodeBufferType, HasEncodeBufferType, MutDecoder, MutDecoderComponent, MutEncoder,
    MutEncoderComponent,
};
use hermes_prelude::*;
use hermes_protobuf_encoding_components::impls::EncodeByteField;

use crate::header::StarknetHeader;
pub struct EncodeStarknetHeader;

#[cgp_provider(MutEncoderComponent)]
impl<Encoding, Strategy> MutEncoder<Encoding, Strategy, StarknetHeader> for EncodeStarknetHeader
where
    Encoding: HasEncodeBufferType + CanRaiseAsyncError<serde_json::Error>,
    EncodeByteField<1>: MutEncoder<Encoding, Strategy, Vec<u8>>,
    EncodeByteField<2>: MutEncoder<Encoding, Strategy, Vec<u8>>,
    EncodeByteField<3>: MutEncoder<Encoding, Strategy, Vec<u8>>,
{
    fn encode_mut(
        encoding: &Encoding,
        value: &StarknetHeader,
        buffer: &mut Encoding::EncodeBuffer,
    ) -> Result<(), Encoding::Error> {
        let StarknetHeader {
            block_header,
            block_signature,
            storage_proof,
        } = value;

        let block_header = serde_json::to_vec(block_header).map_err(Encoding::raise_error)?;
        let block_signature = serde_json::to_vec(block_signature).map_err(Encoding::raise_error)?;
        let storage_proof = serde_json::to_vec(storage_proof).map_err(Encoding::raise_error)?;

        <EncodeByteField<1>>::encode_mut(encoding, &block_header, buffer)?;
        <EncodeByteField<2>>::encode_mut(encoding, &block_signature, buffer)?;
        <EncodeByteField<3>>::encode_mut(encoding, &storage_proof, buffer)?;

        Ok(())
    }
}

#[cgp_provider(MutDecoderComponent)]
impl<Encoding, Strategy> MutDecoder<Encoding, Strategy, StarknetHeader> for EncodeStarknetHeader
where
    Encoding: HasDecodeBufferType + CanRaiseAsyncError<serde_json::Error>,
    EncodeByteField<1>: MutDecoder<Encoding, Strategy, Vec<u8>>,
    EncodeByteField<2>: MutDecoder<Encoding, Strategy, Vec<u8>>,
    EncodeByteField<3>: MutDecoder<Encoding, Strategy, Vec<u8>>,
{
    fn decode_mut(
        encoding: &Encoding,
        buffer: &mut Encoding::DecodeBuffer<'_>,
    ) -> Result<StarknetHeader, Encoding::Error> {
        let block_header = <EncodeByteField<1>>::decode_mut(encoding, buffer)?;
        let block_signature = <EncodeByteField<2>>::decode_mut(encoding, buffer)?;
        let storage_proof = <EncodeByteField<3>>::decode_mut(encoding, buffer)?;

        let block_header = serde_json::from_slice(&block_header).map_err(Encoding::raise_error)?;
        let block_signature =
            serde_json::from_slice(&block_signature).map_err(Encoding::raise_error)?;
        let storage_proof =
            serde_json::from_slice(&storage_proof).map_err(Encoding::raise_error)?;

        Ok(StarknetHeader {
            block_header,
            block_signature,
            storage_proof,
        })
    }
}
