use cgp::prelude::*;
use hermes_encoding_components::traits::decode_mut::{
    CanDecodeMut, MutDecoder, MutDecoderComponent,
};
use hermes_encoding_components::traits::encode_mut::{
    CanEncodeMut, MutEncoder, MutEncoderComponent,
};
use starknet::core::types::Felt;

pub struct EncodeByteArray;

#[cgp_provider(MutEncoderComponent)]
impl<Encoding, Strategy, Value> MutEncoder<Encoding, Strategy, Value> for EncodeByteArray
where
    Encoding: CanEncodeMut<Strategy, Felt> + CanEncodeMut<Strategy, usize>,
    Value: AsRef<[u8]>,
{
    fn encode_mut(
        encoding: &Encoding,
        value: &Value,
        buffer: &mut Encoding::EncodeBuffer,
    ) -> Result<(), Encoding::Error> {
        let bytes = value.as_ref();
        let chunks_count = bytes.len() / 31;

        let chunks = bytes.chunks_exact(31);

        let remainder = chunks.remainder();
        let pending_word = Felt::from_bytes_be_slice(remainder);
        let pending_word_len = remainder.len();

        encoding.encode_mut(&chunks_count, buffer)?;

        for chunk in chunks {
            let felt = Felt::from_bytes_be_slice(chunk);
            encoding.encode_mut(&felt, buffer)?;
        }

        encoding.encode_mut(&pending_word, buffer)?;
        encoding.encode_mut(&pending_word_len, buffer)?;

        Ok(())
    }
}

#[cgp_provider(MutDecoderComponent)]
impl<Encoding, Strategy> MutDecoder<Encoding, Strategy, Vec<u8>> for EncodeByteArray
where
    Encoding: CanDecodeMut<Strategy, Felt> + CanDecodeMut<Strategy, usize>,
{
    fn decode_mut(
        encoding: &Encoding,
        buffer: &mut Encoding::DecodeBuffer<'_>,
    ) -> Result<Vec<u8>, Encoding::Error> {
        let chunks_count: usize = encoding.decode_mut(buffer)?;

        let mut out = <Vec<u8>>::new();

        for _ in 0..chunks_count {
            let felt: Felt = encoding.decode_mut(buffer)?;

            let bytes = &felt.to_bytes_be()[1..];

            out.extend(bytes);
        }

        let pending_word: Felt = encoding.decode_mut(buffer)?;
        let pending_word_len: usize = encoding.decode_mut(buffer)?;

        if pending_word_len > 0 {
            let offset = 32 - pending_word_len;
            let bytes = &pending_word.to_bytes_be()[offset..];

            out.extend(bytes);
        }

        Ok(out)
    }
}
