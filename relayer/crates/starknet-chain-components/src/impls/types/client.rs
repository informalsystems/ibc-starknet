use core::time::Duration;

use hermes_core::chain_components::traits::{
    ClientStateFieldsComponent, ClientStateFieldsGetter, ClientStateTypeComponent,
    ConsensusStateFieldComponent, ConsensusStateFieldGetter, ConsensusStateTypeComponent,
    HasChainIdType, HasClientStateType, HasConsensusStateType, HasHeightType, HasTimeType,
    ProvideClientStateType, ProvideConsensusStateType,
};
use hermes_cosmos_core::tendermint_proto;
use hermes_prelude::*;
use ibc::core::host::types::identifiers::ChainId;
use tendermint::Time;

use crate::types::{WasmStarknetClientState, WasmStarknetConsensusState};

pub struct StarknetRecoverClientPayload;

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

#[cgp_provider(ConsensusStateFieldComponent)]
impl<Chain, Counterparty> ConsensusStateFieldGetter<Chain, Counterparty>
    for ProvideStarknetIbcClientTypes
where
    Chain: HasConsensusStateType<Counterparty, ConsensusState = WasmStarknetConsensusState>,
    Counterparty: HasTimeType<Time = Time>,
{
    fn consensus_state_timestamp(
        consensus_state: &WasmStarknetConsensusState,
    ) -> Counterparty::Time {
        let protobuf_time: tendermint_proto::google::protobuf::Timestamp =
            consensus_state.consensus_state.time.into();
        protobuf_time
            .try_into()
            .expect("failed to convert Tendermint Protobuf Timestamp to Tendermint Time")
    }
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

    fn client_state_is_frozen(client_state: &WasmStarknetClientState) -> bool {
        // We use u8 to define if the client is frozen. If the value is set to 0 it is
        // not frozen. Any other value means the client is frozen
        client_state.client_state.is_frozen != 0
    }

    fn client_state_has_expired(_client_state: &Chain::ClientState, elapsed: Duration) -> bool {
        // WasmStarknetClientState can't expire
        false
    }

    fn client_state_chain_id(client_state: &WasmStarknetClientState) -> Chain::ChainId {
        client_state.client_state.chain_id.clone()
    }

    fn client_state_trusting_period(client_state: &WasmStarknetClientState) -> Option<Duration> {
        None
    }
}
