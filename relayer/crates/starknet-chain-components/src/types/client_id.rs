use cgp::prelude::*;
use hermes_encoding_components::traits::decode_mut::{CanDecodeMut, MutDecoder};
use hermes_encoding_components::traits::encode_mut::{CanEncodeMut, MutEncoder};
use hermes_wasm_encoding_components::components::{MutDecoderComponent, MutEncoderComponent};
pub use ibc::core::host::types::identifiers::ClientId;
use starknet::core::types::Felt;

use super::utils::{felt_to_string, string_to_felt};

pub struct EncodeClientId;

#[cgp_provider(MutEncoderComponent)]
impl<Encoding, Strategy> MutEncoder<Encoding, Strategy, ClientId> for EncodeClientId
where
    Encoding: CanEncodeMut<Strategy, Product![Felt, u64]> + CanRaiseError<&'static str>,
{
    fn encode_mut(
        encoding: &Encoding,
        value: &ClientId,
        buffer: &mut Encoding::EncodeBuffer,
    ) -> Result<(), Encoding::Error> {
        // FIXME: add `sequence_number` method at `ibc-rs`
        let (client_type, sequence) = value
            .as_str()
            .rsplit_once('-')
            .ok_or_else(|| Encoding::raise_error("invalid client id"))?;
        let seq_u64 = sequence
            .parse::<u64>()
            .map_err(|_| Encoding::raise_error("invalid sequence"))?;
        let client_type_felt = string_to_felt(client_type)
            .ok_or_else(|| Encoding::raise_error("invalid client type"))?;
        encoding.encode_mut(&product![client_type_felt, seq_u64], buffer)?;
        Ok(())
    }
}

#[cgp_provider(MutDecoderComponent)]
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
