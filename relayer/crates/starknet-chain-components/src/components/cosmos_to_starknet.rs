use cgp::prelude::*;
use hermes_cosmos_chain_components::components::client::{
    CreateClientMessageBuilderComponent, CreateClientMessageOptionsTypeComponent,
    UpdateClientMessageBuilderComponent,
};
use hermes_cosmos_chain_components::impls::client::create_client_message::BuildAnyCreateClientMessage;
use hermes_cosmos_chain_components::impls::types::create_client_options::ProvideNoCreateClientMessageOptionsType;
use hermes_relayer_components::chain::impls::queries::query_and_convert_client_state::QueryAndConvertRawClientState;
use hermes_relayer_components::chain::impls::queries::query_and_convert_consensus_state::QueryAndConvertRawConsensusState;
use hermes_relayer_components::chain::traits::queries::client_state::{
    ClientStateQuerierComponent, ClientStateWithProofsQuerierComponent,
};
use hermes_relayer_components::chain::traits::queries::consensus_state::{
    ConsensusStateQuerierComponent, ConsensusStateWithProofsQuerierComponent,
};

use crate::impls::starknet_to_cosmos::update_client_message::BuildStarknetUpdateClientMessage;

define_components! {
    StarknetToCosmosComponents {
        [
            ClientStateQuerierComponent,
            ClientStateWithProofsQuerierComponent,
        ]:
            QueryAndConvertRawClientState,
        [
            ConsensusStateQuerierComponent,
            ConsensusStateWithProofsQuerierComponent,
        ]:
            QueryAndConvertRawConsensusState,
        CreateClientMessageOptionsTypeComponent:
            ProvideNoCreateClientMessageOptionsType,
        CreateClientMessageBuilderComponent:
            BuildAnyCreateClientMessage,
        UpdateClientMessageBuilderComponent:
            BuildStarknetUpdateClientMessage,
    }
}
