use hermes_encoding_components::traits::convert::{CanConvert, Converter};
use hermes_encoding_components::traits::decode::CanDecode;
use hermes_encoding_components::traits::encode::CanEncode;
use hermes_encoding_components::traits::types::encoded::HasEncodedType;
use hermes_protobuf_encoding_components::types::strategy::ViaAny;
use hermes_wasm_client_components::types::client_state::WasmClientState;
use ibc_client_starknet_types::{ClientState, ProtoClientState, CLIENT_STATE_TYPE_URL};
use prost_types::Any;

pub type StarknetClientState = ClientState;

pub type ProtoStarknetClientState = ProtoClientState;

pub const STARKNET_CLIENT_STATE_TYPE_URL: &str = CLIENT_STATE_TYPE_URL;

#[derive(Debug)]
pub struct WasmStarknetClientState {
    pub client_state: ClientState,
    pub wasm_code_hash: Vec<u8>,
}

impl From<WasmStarknetClientState> for ClientState {
    fn from(value: WasmStarknetClientState) -> Self {
        value.client_state
    }
}

pub struct ConvertWasmStarknetClientState;

impl<Encoding> Converter<Encoding, WasmStarknetClientState, Any> for ConvertWasmStarknetClientState
where
    Encoding: HasEncodedType<Encoded = Vec<u8>>
        + CanEncode<ViaAny, ClientState>
        + CanConvert<WasmClientState, Any>,
{
    fn convert(
        encoding: &Encoding,
        client_state: &WasmStarknetClientState,
    ) -> Result<Any, Encoding::Error> {
        let tendermint_client_state_bytes = encoding.encode(&client_state.client_state)?;

        let latest_height = client_state.client_state.latest_height;

        let wasm_client_state = WasmClientState {
            data: tendermint_client_state_bytes,
            checksum: client_state.wasm_code_hash.clone(),
            latest_height,
        };

        encoding.convert(&wasm_client_state)
    }
}

impl<Encoding> Converter<Encoding, Any, WasmStarknetClientState> for ConvertWasmStarknetClientState
where
    Encoding: HasEncodedType<Encoded = Vec<u8>>
        + CanDecode<ViaAny, ClientState>
        + CanConvert<Any, WasmClientState>,
{
    fn convert(
        encoding: &Encoding,
        client_state_any: &Any,
    ) -> Result<WasmStarknetClientState, Encoding::Error> {
        let wasm_client_state = encoding.convert(client_state_any)?;

        let client_state = encoding.decode(&wasm_client_state.data)?;

        let wrapped_tendermint_client_state = WasmStarknetClientState {
            client_state,
            wasm_code_hash: wasm_client_state.checksum,
        };

        Ok(wrapped_tendermint_client_state)
    }
}
