use cgp::core::types::impls::WithType;
use cgp::prelude::*;
use hermes_cosmos_chain_components::components::client::{
    ClientStateFieldsComponent, ClientStateTypeComponent, ConsensusStateHeightsQuerierComponent,
    ConsensusStateTypeComponent, CreateClientMessageBuilderComponent,
    CreateClientMessageOptionsTypeComponent, CreateClientPayloadBuilderComponent,
    CreateClientPayloadOptionsTypeComponent, CreateClientPayloadTypeComponent,
    UpdateClientMessageBuilderComponent, UpdateClientPayloadBuilderComponent,
    UpdateClientPayloadTypeComponent,
};
use hermes_cosmos_chain_components::components::cosmos_to_cosmos::CosmosToCosmosComponents;
use hermes_relayer_components::chain::traits::queries::client_state::{
    ClientStateQuerierComponent, ClientStateWithProofsQuerierComponent,
};
use hermes_relayer_components::chain::traits::queries::consensus_state::{
    ConsensusStateQuerierComponent, ConsensusStateWithProofsQuerierComponent,
};

use crate::impls::starknet_to_cosmos::query_consensus_state_height::QueryStarknetConsensusStateHeightsFromGrpc;
use crate::impls::starknet_to_cosmos::update_client_message::BuildStarknetUpdateClientMessage;
use crate::impls::starknet_to_cosmos::update_client_payload::BuildUpdateCometClientPayload;
use crate::types::cosmos::client_state::UseCometClientState;
use crate::types::cosmos::consensus_state::CometConsensusState;
use crate::types::cosmos::update::CometUpdateHeader;

define_components! {
    StarknetToCosmosComponents {
        [
            ClientStateQuerierComponent,
            ClientStateWithProofsQuerierComponent,
            ConsensusStateQuerierComponent,
            ConsensusStateWithProofsQuerierComponent,
            CreateClientPayloadTypeComponent,
            CreateClientMessageOptionsTypeComponent,
            CreateClientPayloadOptionsTypeComponent,
            CreateClientMessageBuilderComponent,
            CreateClientPayloadBuilderComponent,
        ]:
            CosmosToCosmosComponents,
        [
            ClientStateTypeComponent,
            ClientStateFieldsComponent,
        ]:
            UseCometClientState,
        ConsensusStateTypeComponent:
            WithType<CometConsensusState>,
        UpdateClientPayloadTypeComponent:
            WithType<CometUpdateHeader>,
        UpdateClientPayloadBuilderComponent:
            BuildUpdateCometClientPayload,
        UpdateClientMessageBuilderComponent:
            BuildStarknetUpdateClientMessage,
        ConsensusStateHeightsQuerierComponent:
            QueryStarknetConsensusStateHeightsFromGrpc,
    }
}
