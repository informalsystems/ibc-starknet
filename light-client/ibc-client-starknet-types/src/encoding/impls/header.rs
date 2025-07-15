use alloc::vec::Vec;

use hermes_encoding_components::traits::{
    HasEncodeBufferType, MutDecoder, MutDecoderComponent, MutEncoder, MutEncoderComponent,
};
use hermes_prelude::*;
use hermes_protobuf_encoding_components::impls::{
    EncodeByteField, EncodeLengthDelimitedHeader, HasProtoChunksDecodeBuffer, InvalidWireType,
};
use prost::bytes::BufMut;

use crate::header::StarknetHeader;
pub struct EncodeStarknetHeader;

#[cgp_provider(MutEncoderComponent)]
impl<Encoding, Strategy> MutEncoder<Encoding, Strategy, StarknetHeader> for EncodeStarknetHeader
where
    Encoding: HasEncodeBufferType<EncodeBuffer: BufMut> + CanRaiseAsyncError<serde_json::Error>,
    EncodeLengthDelimitedHeader<0>: MutEncoder<Encoding, Strategy, u64>,
    EncodeLengthDelimitedHeader<1>: MutEncoder<Encoding, Strategy, u64>,
    EncodeLengthDelimitedHeader<2>: MutEncoder<Encoding, Strategy, u64>,
{
    fn encode_mut(
        encoding: &Encoding,
        value: &StarknetHeader,
        buffer: &mut Encoding::EncodeBuffer,
    ) -> Result<(), Encoding::Error> {
        EncodeByteField::<0>::encode_mut(
            encoding,
            &serde_json::to_vec(&value.block_header).unwrap(),
            buffer,
        )?;

        EncodeByteField::<1>::encode_mut(
            encoding,
            &serde_json::to_vec(&value.block_signature).unwrap(),
            buffer,
        )?;

        EncodeByteField::<2>::encode_mut(
            encoding,
            &serde_json::to_vec(&value.storage_proof).unwrap(),
            buffer,
        )?;

        Ok(())
    }
}

#[cgp_provider(MutDecoderComponent)]
impl<Encoding, Strategy> MutDecoder<Encoding, Strategy, StarknetHeader> for EncodeStarknetHeader
where
    Encoding: HasProtoChunksDecodeBuffer
        + CanRaiseAsyncError<serde_json::Error>
        + CanRaiseAsyncError<InvalidWireType>
        + for<'a> CanRaiseAsyncError<<Vec<u8> as TryFrom<&'a [u8]>>::Error>,
{
    fn decode_mut<'a>(
        encoding: &Encoding,
        buffer: &mut Encoding::DecodeBuffer<'a>,
    ) -> Result<StarknetHeader, Encoding::Error> {
        let block_header: Vec<u8> =
            <EncodeByteField<0> as MutDecoder<_, Strategy, _>>::decode_mut(encoding, buffer)?;
        let block_signature: Vec<u8> =
            <EncodeByteField<1> as MutDecoder<_, Strategy, _>>::decode_mut(encoding, buffer)?;
        let storage_proof: Vec<u8> =
            <EncodeByteField<2> as MutDecoder<_, Strategy, _>>::decode_mut(encoding, buffer)?;

        let block_header = serde_json::from_slice(&block_header).unwrap();
        let block_signature = serde_json::from_slice(&block_signature).unwrap();
        let storage_proof = serde_json::from_slice(&storage_proof).unwrap();

        Ok(StarknetHeader {
            block_header,
            block_signature,
            storage_proof,
        })
    }
}
