use core::time::Duration;

use cgp::core::types::WithType;
use cgp::prelude::*;
use hermes_chain_components::traits::types::chain_id::HasChainIdType;
use hermes_chain_components::traits::types::client_state::{
    ClientStateFieldsComponent, ClientStateFieldsGetter, ClientStateTypeComponent,
    HasClientStateType,
};
use hermes_chain_components::traits::types::height::HasHeightType;
use ibc::clients::tendermint::types::{
    AllowUpdate, ClientState as IbcCometClientState, TrustThreshold,
};
use ibc::core::client::types::Height as CosmosHeight;
use ibc::core::commitment_types::specs::ProofSpecs;
use ibc::core::host::types::identifiers::ChainId;
use ibc::primitives::proto::Any;

use crate::types::cosmos::height::Height;

#[derive(Debug, HasField, HasFields)]
pub struct CometClientState {
    pub latest_height: Height,
    pub trusting_period: Duration,
    pub unbonding_period: Duration,
    pub max_clock_drift: Duration,
    pub status: ClientStatus,
    pub chain_id: ChainId,
}

#[derive(Debug, HasFields)]
pub enum ClientStatus {
    Active,
    Expired,
    Frozen(Height),
}

pub struct UseCometClientState;

delegate_components! {
    UseCometClientState {
        ClientStateTypeComponent:
            WithType<CometClientState>,
    }
}

#[cgp_provider(ClientStateFieldsComponent)]
impl<Chain, Counterparty> ClientStateFieldsGetter<Chain, Counterparty> for UseCometClientState
where
    Chain: HasClientStateType<Counterparty, ClientState = CometClientState>
        + HasHeightType<Height = CosmosHeight>
        + HasChainIdType<ChainId = ChainId>,
{
    fn client_state_latest_height(client_state: &CometClientState) -> CosmosHeight {
        CosmosHeight::new(
            client_state.latest_height.revision_number,
            client_state.latest_height.revision_height,
        )
        .unwrap()
    }

    fn client_state_is_frozen(_client_state: &CometClientState) -> bool {
        false // todo
    }

    fn client_state_has_expired(_client_state: &CometClientState, _elapsed: Duration) -> bool {
        false // todo
    }

    fn client_state_chain_id(client_state: &CometClientState) -> ChainId {
        client_state.chain_id.clone()
    }
}

impl From<CometClientState> for IbcCometClientState {
    fn from(client_state: CometClientState) -> Self {
        Self::new(
            client_state.chain_id,
            TrustThreshold::ONE_THIRD,
            client_state.trusting_period,
            client_state.unbonding_period,
            client_state.max_clock_drift,
            CosmosHeight::new(
                client_state.latest_height.revision_number,
                client_state.latest_height.revision_height,
            )
            .expect("no error"),
            ProofSpecs::cosmos(),
            Vec::new(),
            AllowUpdate {
                after_expiry: false,
                after_misbehaviour: false,
            },
        )
        .expect("no error")
    }
}

impl From<CometClientState> for Any {
    fn from(client_state: CometClientState) -> Self {
        IbcCometClientState::from(client_state).into()
    }
}
