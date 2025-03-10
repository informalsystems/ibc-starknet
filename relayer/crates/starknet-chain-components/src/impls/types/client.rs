use core::time::Duration;

use cgp::prelude::*;
use hermes_chain_components::traits::types::chain_id::HasChainIdType;
use hermes_chain_components::traits::types::client_state::{
    ClientStateFieldsComponent, ClientStateTypeComponent,
};
use hermes_chain_components::traits::types::consensus_state::ConsensusStateTypeComponent;
use hermes_relayer_components::chain::traits::types::client_state::{
    ClientStateFieldsGetter, HasClientStateType, ProvideClientStateType,
};
use hermes_relayer_components::chain::traits::types::consensus_state::ProvideConsensusStateType;
use hermes_relayer_components::chain::traits::types::height::HasHeightType;
use ibc::core::host::types::identifiers::ChainId;

use crate::types::client_state::WasmStarknetClientState;
use crate::types::consensus_state::WasmStarknetConsensusState;

pub struct ProvideStarknetIbcClientTypes;

#[cgp_provider(ClientStateTypeComponent)]
impl<Chain: Async, Counterparty> ProvideClientStateType<Chain, Counterparty>
    for ProvideStarknetIbcClientTypes
{
    type ClientState = WasmStarknetClientState;
}

#[cgp_provider(ConsensusStateTypeComponent)]
impl<Chain: Async, Counterparty> ProvideConsensusStateType<Chain, Counterparty>
    for ProvideStarknetIbcClientTypes
{
    type ConsensusState = WasmStarknetConsensusState;
}

#[cgp_provider(ClientStateFieldsComponent)]
impl<Chain, Counterparty> ClientStateFieldsGetter<Chain, Counterparty>
    for ProvideStarknetIbcClientTypes
where
    Chain: HasHeightType<Height = u64>
        + HasClientStateType<Counterparty, ClientState = WasmStarknetClientState>
        + HasChainIdType<ChainId = ChainId>,
{
    fn client_state_latest_height(client_state: &WasmStarknetClientState) -> u64 {
        client_state.client_state.latest_height.revision_height()
    }

    fn client_state_is_frozen(_client_state: &Chain::ClientState) -> bool {
        false
    }

    fn client_state_has_expired(_client_state: &Chain::ClientState, _elapsed: Duration) -> bool {
        false
    }

    fn client_state_chain_id(client_state: &WasmStarknetClientState) -> Chain::ChainId {
        client_state.client_state.chain_id.clone()
    }
}
