use cgp::core::types::WithType;
use cgp::prelude::*;
use hermes_chain_components::traits::types::ibc::CounterpartyMessageHeightGetterComponent;
use hermes_cosmos_chain_components::components::client::{
    ChannelOpenAckMessageBuilderComponent, ChannelOpenConfirmMessageBuilderComponent,
    ChannelOpenInitMessageBuilderComponent, ChannelOpenTryMessageBuilderComponent,
    ClientStateFieldsComponent, ClientStateTypeComponent, ConnectionOpenAckMessageBuilderComponent,
    ConnectionOpenConfirmMessageBuilderComponent, ConnectionOpenInitMessageBuilderComponent,
    ConnectionOpenTryMessageBuilderComponent, ConsensusStateHeightsQuerierComponent,
    ConsensusStateTypeComponent, CreateClientMessageBuilderComponent,
    CreateClientMessageOptionsTypeComponent, CreateClientPayloadBuilderComponent,
    CreateClientPayloadOptionsTypeComponent, CreateClientPayloadTypeComponent,
    PacketDstChannelIdGetterComponent, PacketDstPortIdGetterComponent,
    PacketSequenceGetterComponent, PacketSrcChannelIdGetterComponent,
    PacketSrcPortIdGetterComponent, PacketTimeoutHeightGetterComponent,
    PacketTimeoutTimestampGetterComponent, UpdateClientMessageBuilderComponent,
    UpdateClientPayloadBuilderComponent, UpdateClientPayloadTypeComponent,
};
use hermes_cosmos_chain_components::components::cosmos_to_cosmos::CosmosToCosmosComponents;
use hermes_cosmos_chain_components::impls::packet::packet_fields::CosmosPacketFieldReader;
use hermes_relayer_components::chain::traits::queries::client_state::{
    ClientStateQuerierComponent, ClientStateWithProofsQuerierComponent,
};
use hermes_relayer_components::chain::traits::queries::consensus_state::{
    ConsensusStateQuerierComponent, ConsensusStateWithProofsQuerierComponent,
};

use crate::impls::payload_builders::create_client::BuildStarknetCreateClientPayload;
use crate::impls::starknet_to_cosmos::channel_message::BuildStarknetToCosmosChannelHandshakeMessage;
use crate::impls::starknet_to_cosmos::connection_message::BuildStarknetToCosmosConnectionHandshake;
use crate::impls::starknet_to_cosmos::counterparty_message_height::GetCosmosCounterpartyMessageStarknetHeight;
use crate::impls::starknet_to_cosmos::packet_fields::ReadPacketDstStarknetFields;
use crate::impls::starknet_to_cosmos::query_consensus_state_height::QueryStarknetConsensusStateHeightsFromGrpc;
use crate::impls::starknet_to_cosmos::update_client_message::BuildStarknetUpdateClientMessage;
use crate::impls::starknet_to_cosmos::update_client_payload::BuildUpdateCometClientPayload;
use crate::types::cosmos::client_state::UseCometClientState;
use crate::types::cosmos::consensus_state::CometConsensusState;
use crate::types::cosmos::update::CometUpdateHeader;

cgp_preset! {
    StarknetToCosmosComponents {
        [
            ClientStateQuerierComponent,
            ClientStateWithProofsQuerierComponent,
            ConsensusStateQuerierComponent,
            ConsensusStateWithProofsQuerierComponent,
            CreateClientPayloadTypeComponent,
            CreateClientMessageOptionsTypeComponent,
            CreateClientPayloadOptionsTypeComponent,
            CreateClientPayloadBuilderComponent,
            ChannelOpenInitMessageBuilderComponent,
        ]:
            CosmosToCosmosComponents,
        CreateClientMessageBuilderComponent:
            BuildStarknetCreateClientPayload,
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
        CounterpartyMessageHeightGetterComponent:
            GetCosmosCounterpartyMessageStarknetHeight,
        [
            ConnectionOpenInitMessageBuilderComponent,
            ConnectionOpenTryMessageBuilderComponent,
            ConnectionOpenAckMessageBuilderComponent,
            ConnectionOpenConfirmMessageBuilderComponent,
        ]:
            BuildStarknetToCosmosConnectionHandshake,
        [
            ChannelOpenTryMessageBuilderComponent,
            ChannelOpenAckMessageBuilderComponent,
            ChannelOpenConfirmMessageBuilderComponent,
        ]:
            BuildStarknetToCosmosChannelHandshakeMessage,
        [
            PacketSrcChannelIdGetterComponent,
            PacketSrcPortIdGetterComponent,
            PacketDstPortIdGetterComponent,
            PacketSequenceGetterComponent,
            PacketTimeoutTimestampGetterComponent,
        ]:
            CosmosPacketFieldReader,
        [
            PacketTimeoutHeightGetterComponent,
            PacketDstChannelIdGetterComponent,
        ]:
            ReadPacketDstStarknetFields,

    }
}
