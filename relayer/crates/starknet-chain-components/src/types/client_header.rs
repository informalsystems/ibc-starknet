use hermes_encoding_components::traits::convert::{CanConvert, Converter};
use hermes_encoding_components::traits::decode::CanDecode;
use hermes_encoding_components::traits::encode::CanEncode;
use hermes_encoding_components::traits::types::encoded::HasEncodedType;
use hermes_protobuf_encoding_components::types::strategy::ViaAny;
use ibc::clients::wasm_types::client_message::ClientMessage;
use prost_types::Any;

use super::consensus_state::StarknetConsensusState;

pub struct StarknetClientHeader {
    pub consensus_state: StarknetConsensusState,
}

pub struct ConvertStarknetClientHeader;

impl<Encoding> Converter<Encoding, StarknetClientHeader, Any> for ConvertStarknetClientHeader
where
    Encoding: HasEncodedType<Encoded = Vec<u8>>
        + CanEncode<ViaAny, StarknetConsensusState>
        + CanConvert<ClientMessage, Any>,
{
    fn convert(encoding: &Encoding, header: &StarknetClientHeader) -> Result<Any, Encoding::Error> {
        let consensus_state_bytes = encoding.encode(&header.consensus_state)?;

        let wasm_message = ClientMessage {
            data: consensus_state_bytes,
        };

        encoding.convert(&wasm_message)
    }
}

impl<Encoding> Converter<Encoding, Any, StarknetClientHeader> for ConvertStarknetClientHeader
where
    Encoding: HasEncodedType<Encoded = Vec<u8>>
        + CanDecode<ViaAny, StarknetConsensusState>
        + CanConvert<Any, ClientMessage>,
{
    fn convert(
        encoding: &Encoding,
        header_any: &Any,
    ) -> Result<StarknetClientHeader, Encoding::Error> {
        let client_message = encoding.convert(header_any)?;

        let consensus_state = encoding.decode(&client_message.data)?;

        Ok(StarknetClientHeader { consensus_state })
    }
}
