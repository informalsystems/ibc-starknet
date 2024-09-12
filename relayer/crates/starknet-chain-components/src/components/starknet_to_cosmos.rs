use cgp::prelude::*;
use hermes_cosmos_chain_components::components::client::{
    ConsensusStateHeightsQuerierComponent, CreateClientMessageBuilderComponent,
    CreateClientMessageOptionsTypeComponent, UpdateClientMessageBuilderComponent,
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

define_components! {
    StarknetToCosmosComponents {
        [
            ClientStateQuerierComponent,
            ClientStateWithProofsQuerierComponent,
            ConsensusStateQuerierComponent,
            ConsensusStateWithProofsQuerierComponent,
            CreateClientMessageOptionsTypeComponent,
            CreateClientMessageBuilderComponent,
        ]:
            CosmosToCosmosComponents,
        UpdateClientMessageBuilderComponent:
            BuildStarknetUpdateClientMessage,
        ConsensusStateHeightsQuerierComponent:
            QueryStarknetConsensusStateHeightsFromGrpc,
    }
}
