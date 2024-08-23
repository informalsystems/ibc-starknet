use cgp_core::Async;
use hermes_relayer_components::chain::traits::types::client_state::ProvideClientStateType;
use hermes_relayer_components::chain::traits::types::consensus_state::ProvideConsensusStateType;

use crate::types::client_state::StarknetClientState;
use crate::types::consensus_state::StarknetConsensusState;

pub struct ProvideStarknetIbcClientTypes;

impl<Chain: Async, Counterparty> ProvideClientStateType<Chain, Counterparty>
    for ProvideStarknetIbcClientTypes
{
    type ClientState = StarknetClientState;
}

impl<Chain: Async, Counterparty> ProvideConsensusStateType<Chain, Counterparty>
    for ProvideStarknetIbcClientTypes
{
    type ConsensusState = StarknetConsensusState;
}
