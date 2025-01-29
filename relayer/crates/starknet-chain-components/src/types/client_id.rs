use cgp::prelude::*;
use hermes_encoding_components::traits::decode_mut::{CanDecodeMut, MutDecoder};
use hermes_encoding_components::traits::encode_mut::{CanEncodeMut, MutEncoder};
pub use ibc::core::host::types::identifiers::ClientId;
use starknet::core::types::Felt;

use super::utils::{felt_to_string, string_to_felt};

pub struct EncodeClientId;

impl<Encoding, Strategy> MutEncoder<Encoding, Strategy, ClientId> for EncodeClientId
where
    Encoding: CanEncodeMut<Strategy, Product![Felt, u64]>,
{
    fn encode_mut(
        encoding: &Encoding,
        value: &ClientId,
        buffer: &mut Encoding::EncodeBuffer,
    ) -> Result<(), Encoding::Error> {
        let (client_type, sequence) = value.as_str().rsplit_once('-').expect("valid client id");
        let seq_u64 = sequence.parse::<u64>().expect("valid sequence");
        let client_type_felt = string_to_felt(client_type).expect("valid client type");
        encoding.encode_mut(&product![client_type_felt, seq_u64], buffer)?;
        Ok(())
    }
}

impl<Encoding, Strategy> MutDecoder<Encoding, Strategy, ClientId> for EncodeClientId
where
    Encoding: CanDecodeMut<Strategy, Product![Felt, u64]> + CanRaiseAsyncError<&'static str>,
{
    fn decode_mut<'a>(
        encoding: &Encoding,
        buffer: &mut Encoding::DecodeBuffer<'a>,
    ) -> Result<ClientId, Encoding::Error> {
        let product![client_type_felt, seq_u64] = encoding.decode_mut(buffer)?;
        ClientId::new(&felt_to_string(client_type_felt), seq_u64)
            .map_err(|_| Encoding::raise_error("invalid client id"))
    }
}
