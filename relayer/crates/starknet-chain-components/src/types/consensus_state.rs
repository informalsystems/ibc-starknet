use cgp::prelude::*;
use hermes_encoding_components::traits::convert::Converter;
use hermes_wasm_encoding_components::components::ConverterComponent;
use hermes_wasm_encoding_components::impls::strategies::consensus_state::{
    DecodeViaWasmConsensusState, EncodeViaWasmConsensusState,
};
pub use ibc_client_starknet_types::StarknetConsensusState;
use prost_types::Any;

#[derive(Debug)]
pub struct WasmStarknetConsensusState {
    pub consensus_state: StarknetConsensusState,
}

pub struct ConvertWasmStarknetConsensusState;

#[cgp_provider(ConverterComponent)]
impl<Encoding> Converter<Encoding, WasmStarknetConsensusState, Any>
    for ConvertWasmStarknetConsensusState
where
    Encoding: HasAsyncErrorType,
    EncodeViaWasmConsensusState: Converter<Encoding, StarknetConsensusState, Any>,
{
    fn convert(
        encoding: &Encoding,
        consensus_state: &WasmStarknetConsensusState,
    ) -> Result<Any, Encoding::Error> {
        EncodeViaWasmConsensusState::convert(encoding, &consensus_state.consensus_state)
    }
}

#[cgp_provider(ConverterComponent)]
impl<Encoding> Converter<Encoding, Any, WasmStarknetConsensusState>
    for ConvertWasmStarknetConsensusState
where
    Encoding: HasAsyncErrorType,
    DecodeViaWasmConsensusState: Converter<Encoding, Any, StarknetConsensusState>,
{
    fn convert(
        encoding: &Encoding,
        consensus_state_any: &Any,
    ) -> Result<WasmStarknetConsensusState, Encoding::Error> {
        let consensus_state = DecodeViaWasmConsensusState::convert(encoding, consensus_state_any)?;

        Ok(WasmStarknetConsensusState { consensus_state })
    }
}
