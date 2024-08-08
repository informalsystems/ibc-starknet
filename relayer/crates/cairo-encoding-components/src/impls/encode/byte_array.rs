use starknet::core::types::Felt;

use crate::traits::encode_mut::{CanEncodeMut, MutEncoder};

pub struct EncodeByteArray;

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
