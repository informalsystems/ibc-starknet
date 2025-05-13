#[cgp::re_export_imports]
mod preset {
    use cgp::core::types::WithType;
    use hermes_core::chain_components::traits::{
        AckPacketMessageBuilderComponent, ChannelOpenAckMessageBuilderComponent,
        ChannelOpenConfirmMessageBuilderComponent, ChannelOpenInitMessageBuilderComponent,
        ChannelOpenTryMessageBuilderComponent, ClientStateFieldsComponent,
        ClientStateQuerierComponent, ClientStateTypeComponent,
        ClientStateWithProofsQuerierComponent, ConnectionOpenAckMessageBuilderComponent,
        ConnectionOpenConfirmMessageBuilderComponent, ConnectionOpenInitMessageBuilderComponent,
        ConnectionOpenTryMessageBuilderComponent, ConsensusStateHeightsQuerierComponent,
        ConsensusStateQuerierComponent, ConsensusStateTypeComponent,
        ConsensusStateWithProofsQuerierComponent, CounterpartyMessageHeightGetterComponent,
        CreateClientMessageBuilderComponent, CreateClientMessageOptionsTypeComponent,
        CreateClientPayloadBuilderComponent, CreateClientPayloadOptionsTypeComponent,
        CreateClientPayloadTypeComponent, PacketDstChannelIdGetterComponent,
        PacketDstPortIdGetterComponent, PacketSequenceGetterComponent,
        PacketSrcChannelIdGetterComponent, PacketSrcPortIdGetterComponent,
        PacketTimeoutHeightGetterComponent, PacketTimeoutTimestampGetterComponent,
        ReceivePacketMessageBuilderComponent, TimeoutUnorderedPacketMessageBuilderComponent,
        UpdateClientMessageBuilderComponent, UpdateClientPayloadBuilderComponent,
        UpdateClientPayloadTypeComponent,
    };
    use hermes_core::test_components::chain::traits::IbcTransferredAmountConverterComponent;
    use hermes_cosmos_core::chain_components::impls::{
        BuildCosmosPacketMessages, CosmosPacketFieldReader,
    };
    use hermes_cosmos_core::chain_preset::presets::CosmosToCosmosComponents;
    use hermes_prelude::*;

    use crate::impls::{
        BuildStarknetCreateClientMessage, BuildStarknetToCosmosConnectionHandshake,
        BuildStarknetUpdateClientMessage, ConvertCosmosIbcAmountFromStarknet,
        GetCosmosCounterpartyMessageStarknetHeight, QueryStarknetConsensusStateHeightsFromGrpc,
        ReadPacketDstStarknetFields,
    };
    use crate::types::{CometConsensusState, UseCometClientState};

    cgp_preset! {
        StarknetToCosmosComponents {
            [
                ClientStateQuerierComponent,
                ClientStateWithProofsQuerierComponent,
                ConsensusStateQuerierComponent,
                ConsensusStateWithProofsQuerierComponent,
                CreateClientPayloadTypeComponent,
                UpdateClientPayloadTypeComponent,
                CreateClientMessageOptionsTypeComponent,
                CreateClientPayloadOptionsTypeComponent,
                CreateClientPayloadBuilderComponent,
                UpdateClientPayloadBuilderComponent,
                ChannelOpenInitMessageBuilderComponent,
                ChannelOpenTryMessageBuilderComponent,
                ChannelOpenAckMessageBuilderComponent,
                ChannelOpenConfirmMessageBuilderComponent,
            ]:
                CosmosToCosmosComponents::Provider,
            CreateClientMessageBuilderComponent:
                BuildStarknetCreateClientMessage,
            [
                ClientStateTypeComponent,
                ClientStateFieldsComponent,
            ]:
                UseCometClientState,
            ConsensusStateTypeComponent:
                WithType<CometConsensusState>,
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
            [
                ReceivePacketMessageBuilderComponent,
                AckPacketMessageBuilderComponent,
                TimeoutUnorderedPacketMessageBuilderComponent,
            ]:
                BuildCosmosPacketMessages,

            IbcTransferredAmountConverterComponent:
                ConvertCosmosIbcAmountFromStarknet,
        }
    }
}
