#[cgp::re_export_imports]
mod preset {
    use cgp::core::types::UseDelegatedType;
    use hermes_core::chain_components::impls::{
        BuildChannelHandshakePayload, BuildConnectionHandshakePayload, BuildPacketPayloads,
        ProvideBytesPacketCommitment, ProvideBytesPacketReceipt, ProvideChannelPayloadTypes,
        ProvideCommitmentPrefixBytes, ProvideConnectionPayloadTypes, ProvidePacketPayloadTypes,
        QueryClearedPacketWithEmptyCommitment, QueryConsensusStateHeightsAndFindHeightBefore,
        QueryLatestConsensusStateHeightAsHeights, RetryQueryBlockEvents,
        WaitBlockHeightAndQueryEvents,
    };
    use hermes_core::chain_components::traits::{
        AckCommitmentHashTypeProviderComponent, AckPacketMessageBuilderComponent,
        AckPacketPayloadBuilderComponent, AckPacketPayloadTypeProviderComponent,
        AcknowledgementTypeProviderComponent, BlockEventsQuerierComponent, BlockQuerierComponent,
        BlockTypeComponent, ChainIdTypeProviderComponent, ChainStatusQuerierComponent,
        ChainStatusTypeComponent, ChannelEndQuerierComponent, ChannelEndTypeComponent,
        ChannelEndWithProofsQuerierComponent, ChannelIdTypeComponent,
        ChannelOpenAckMessageBuilderComponent, ChannelOpenAckPayloadBuilderComponent,
        ChannelOpenAckPayloadTypeComponent, ChannelOpenConfirmMessageBuilderComponent,
        ChannelOpenConfirmPayloadBuilderComponent, ChannelOpenConfirmPayloadTypeComponent,
        ChannelOpenInitEventComponent, ChannelOpenInitMessageBuilderComponent,
        ChannelOpenTryEventComponent, ChannelOpenTryMessageBuilderComponent,
        ChannelOpenTryPayloadBuilderComponent, ChannelOpenTryPayloadTypeComponent,
        ClientIdTypeComponent, ClientStateFieldsComponent, ClientStateQuerierComponent,
        ClientStateTypeComponent, ClientStateWithProofsQuerierComponent,
        CommitmentPrefixTypeComponent, CommitmentProofBytesGetterComponent,
        CommitmentProofHeightGetterComponent, CommitmentProofTypeProviderComponent,
        ConnectionEndQuerierComponent, ConnectionEndTypeComponent,
        ConnectionEndWithProofsQuerierComponent, ConnectionIdTypeComponent,
        ConnectionOpenAckMessageBuilderComponent, ConnectionOpenAckPayloadBuilderComponent,
        ConnectionOpenAckPayloadTypeComponent, ConnectionOpenConfirmMessageBuilderComponent,
        ConnectionOpenConfirmPayloadBuilderComponent, ConnectionOpenConfirmPayloadTypeComponent,
        ConnectionOpenInitEventComponent, ConnectionOpenInitMessageBuilderComponent,
        ConnectionOpenInitPayloadBuilderComponent, ConnectionOpenInitPayloadTypeComponent,
        ConnectionOpenTryEventComponent, ConnectionOpenTryMessageBuilderComponent,
        ConnectionOpenTryPayloadBuilderComponent, ConnectionOpenTryPayloadTypeComponent,
        ConsensusStateHeightQuerierComponent, ConsensusStateHeightsQuerierComponent,
        ConsensusStateQuerierComponent, ConsensusStateTypeComponent,
        ConsensusStateWithProofsQuerierComponent, CounterpartyChainIdQuerierComponent,
        CounterpartyConnectionIdQuerierComponent, CounterpartyMessageHeightGetterComponent,
        CreateClientEventComponent, CreateClientMessageBuilderComponent,
        CreateClientMessageOptionsTypeComponent, CreateClientPayloadBuilderComponent,
        CreateClientPayloadOptionsTypeComponent, CreateClientPayloadTypeComponent,
        EventExtractorComponent, EventTypeProviderComponent, HeightAdjusterComponent,
        HeightFieldComponent, HeightIncrementerComponent, HeightTypeProviderComponent,
        IbcCommitmentPrefixGetterComponent, IncomingPacketFilterComponent,
        InitChannelOptionsTypeComponent, InitConnectionOptionsTypeComponent,
        MessageResponseExtractorComponent, MessageSenderComponent, MessageTypeProviderComponent,
        OutgoingPacketFilterComponent, OutgoingPacketTypeComponent,
        PacketAckCommitmentQuerierComponent, PacketCommitmentQuerierComponent,
        PacketCommitmentTypeComponent, PacketDstChannelIdGetterComponent,
        PacketDstPortIdGetterComponent, PacketFromSendPacketEventBuilderComponent,
        PacketFromWriteAckEventBuilderComponent, PacketIsClearedQuerierComponent,
        PacketIsReceivedQuerierComponent, PacketReceiptQuerierComponent,
        PacketReceiptTypeComponent, PacketSequenceGetterComponent,
        PacketSrcChannelIdGetterComponent, PacketSrcPortIdGetterComponent,
        PacketTimeoutHeightGetterComponent, PacketTimeoutTimestampGetterComponent,
        PortIdTypeComponent, ReceivePacketMessageBuilderComponent,
        ReceivePacketPayloadBuilderComponent, ReceivePacketPayloadTypeComponent,
        SendPacketEventComponent, SequenceTypeComponent, TimeoutTypeComponent,
        TimeoutUnorderedPacketMessageBuilderComponent,
        TimeoutUnorderedPacketPayloadBuilderComponent, TimeoutUnorderedPacketPayloadTypeComponent,
        UpdateClientMessageBuilderComponent, UpdateClientPayloadBuilderComponent,
        UpdateClientPayloadTypeComponent, WriteAckEventComponent,
    };
    use hermes_core::chain_type_components::traits::{
        AddressTypeProviderComponent, AmountDenomGetterComponent, AmountTypeProviderComponent,
        DenomTypeComponent, MessageResponseEventsGetterComponent, MessageResponseTypeComponent,
        TimeTypeComponent,
    };
    use hermes_core::relayer_components::components::default::DefaultTxComponents;
    use hermes_core::relayer_components::error::impls::retry::ReturnRetryable;
    use hermes_core::relayer_components::error::traits::RetryableErrorComponent;
    use hermes_core::relayer_components::transaction::impls::PollTimeoutGetterComponent;
    use hermes_core::relayer_components::transaction::traits::{
        MessagesWithSignerAndNonceSenderComponent, MessagesWithSignerSenderComponent,
        NonceAllocatorComponent, NonceQuerierComponent, NonceTypeProviderComponent,
        SignerTypeProviderComponent, TxHashTypeProviderComponent, TxMessageResponseParserComponent,
        TxResponsePollerComponent, TxResponseQuerierComponent, TxResponseTypeProviderComponent,
    };
    use hermes_core::test_components::chain::impls::{
        PollAssertEventualAmount, ProvideDefaultMemo, SendIbcTransferMessage,
    };
    use hermes_core::test_components::chain::traits::{
        AmountMethodsComponent, BalanceQuerierComponent, DefaultMemoGetterComponent,
        EventualAmountAsserterComponent, IbcTokenTransferMessageBuilderComponent,
        IbcTransferTimeoutCalculatorComponent, IbcTransferredAmountConverterComponent,
        MemoTypeProviderComponent, PollAssertDurationGetterComponent, TokenIbcTransferrerComponent,
        WalletSignerComponent, WalletTypeComponent,
    };
    use hermes_cosmos_core::chain_components::impls::{
        CosmosPacketFieldReader, FixedPollTimeoutSecs, ProvideCosmosChainTypes,
        ProvideCosmosInitChannelOptionsType, ProvideCosmosInitConnectionOptionsType,
        ProvideNoCreateClientMessageOptionsType, QueryCounterpartyConnectionId,
    };
    use hermes_prelude::*;
    use ibc::core::host::types::identifiers::ChainId;
    use starknet::core::types::Felt;

    use crate::components::types::StarknetChainTypes;
    use crate::impls::{
        BuildCreateCometClientMessage, BuildInvokeContractCall,
        BuildStarknetChannelHandshakeMessages, BuildStarknetConnectionHandshakeMessages,
        BuildStarknetCreateClientPayload, BuildStarknetIbcTransferMessage,
        BuildStarknetPacketMessages, BuildStarknetUpdateClientPayload,
        BuildUpdateCometClientMessage, CallStarknetContract, ConvertStarknetTokenAddressFromCosmos,
        DeclareSierraContract, DeployStarknetContract, FilterStarknetPackets,
        GetContractAddressFromField, GetCounterpartyCosmosHeightFromStarknetMessage,
        GetOrCreateCosmosTokenAddressOnStarknet, GetStarknetBlockEvents,
        GetStarknetCommitmentPrefix, IbcTransferTimeoutAfterSeconds, InvokeStarknetContract,
        ProvideFeltSelector, ProvidePollAssertDuration, ProvideStarknetBlockType,
        ProvideStarknetChainStatusType, ProvideStarknetHeight, ProvideStarknetIbcClientTypes,
        ProvideStarknetPayloadTypes, ProvideStarknetWallet, ProvideTokenAddressDenom,
        QueryChannelEndFromStarknet, QueryCometClientState, QueryCometConsensusState,
        QueryConnectionEndFromStarknet, QueryCosmosChainIdFromStarknetChannelId,
        QueryErc20TokenBalance, QueryPacketIsReceivedOnStarknet, QueryStarknetAckCommitment,
        QueryStarknetBlock, QueryStarknetChainStatus, QueryStarknetNonce,
        QueryStarknetPacketCommitment, QueryStarknetPacketReceipt, QueryStarknetWalletBalance,
        QueryTransactionReceipt, ReadPacketSrcStarknetFields, SendStarknetMessages,
        StarknetAddress, StarknetMessage, TransferErc20Token, UseStarknetCommitmentProof,
        UseStarknetContractTypes, UseStarknetEvents, UseU256Amount,
    };
    use crate::traits::{
        BlobTypeProviderComponent, ContractAddressQuerierComponent, ContractCallerComponent,
        ContractClassHashTypeProviderComponent, ContractClassTypeProviderComponent,
        ContractDeclarerComponent, ContractDeployerComponent, ContractInvokerComponent,
        CosmosTokenAddressOnStarknetQuerierComponent, InvokeContractMessageBuilderComponent,
        SelectorTypeComponent, TokenBalanceQuerierComponent, TokenTransferComponent,
        TransferTokenMessageBuilderComponent,
    };
    use crate::types::{
        BuildTransferErc20TokenMessage, StarknetEvent, StarknetWallet, TxResponse,
        UseStarknetMessageResponse,
    };

    cgp_preset! {
        StarknetChainComponents {
            ChainIdTypeProviderComponent:
                UseType<ChainId>,
            [
                HeightTypeProviderComponent,
                HeightFieldComponent,
                HeightIncrementerComponent,
                HeightAdjusterComponent,
            ]:
                ProvideStarknetHeight,
            ChainStatusTypeComponent:
                ProvideStarknetChainStatusType,
            BlockTypeComponent:
                ProvideStarknetBlockType,
            AddressTypeProviderComponent:
                UseType<StarknetAddress>,
            BlobTypeProviderComponent:
                UseType<Vec<Felt>>,
            MessageTypeProviderComponent:
                UseType<StarknetMessage>,
            EventTypeProviderComponent:
                UseType<StarknetEvent>,
            [
                MessageResponseTypeComponent,
                MessageResponseEventsGetterComponent,
            ]:
                UseStarknetMessageResponse,
            [
                AmountTypeProviderComponent,
                AmountDenomGetterComponent,
                AmountMethodsComponent,
            ]:
                UseU256Amount,
            DenomTypeComponent:
                ProvideTokenAddressDenom,
            MemoTypeProviderComponent:
                UseType<Option<String>>,
            DefaultMemoGetterComponent:
                ProvideDefaultMemo,
            [
                WalletTypeComponent,
                WalletSignerComponent,
            ]:
                ProvideStarknetWallet,
            SignerTypeProviderComponent:
                UseType<StarknetWallet>,
            NonceTypeProviderComponent:
                UseType<Felt>,
            TokenIbcTransferrerComponent:
                SendIbcTransferMessage,
            IbcTransferTimeoutCalculatorComponent:
                IbcTransferTimeoutAfterSeconds<300>,
            IbcTransferredAmountConverterComponent:
                ConvertStarknetTokenAddressFromCosmos,
            CosmosTokenAddressOnStarknetQuerierComponent:
                GetOrCreateCosmosTokenAddressOnStarknet,
            TxHashTypeProviderComponent:
                UseType<Felt>,
            TxResponseTypeProviderComponent:
                UseType<TxResponse>,
            SelectorTypeComponent:
                ProvideFeltSelector,
            [
                ContractClassTypeProviderComponent,
                ContractClassHashTypeProviderComponent,
            ]:
                UseStarknetContractTypes,
            // FIXME: we may have to define our own chain types,
            // or implement Cairo encoding for the Cosmos types
            [
                PortIdTypeComponent,
                SequenceTypeComponent,
                OutgoingPacketTypeComponent,
                TimeTypeComponent,
                TimeoutTypeComponent,
            ]:
                ProvideCosmosChainTypes,
            [
                ClientIdTypeComponent,
                ConnectionIdTypeComponent,
                ChannelIdTypeComponent,
                ConnectionEndTypeComponent,
                ChannelEndTypeComponent,
            ]:
                WithProvider<UseDelegatedType<StarknetChainTypes>>,
            [
                ClientStateTypeComponent,
                ConsensusStateTypeComponent,
                ClientStateFieldsComponent,
            ]:
                ProvideStarknetIbcClientTypes,
            [
                CreateClientPayloadTypeComponent,
                CreateClientPayloadOptionsTypeComponent,
                UpdateClientPayloadTypeComponent,
            ]:
                ProvideStarknetPayloadTypes,
            // FIXME: define our own Starknet init channel options type
            InitChannelOptionsTypeComponent:
                ProvideCosmosInitChannelOptionsType,
            [
                CommitmentProofTypeProviderComponent,
                CommitmentProofHeightGetterComponent,
                CommitmentProofBytesGetterComponent,
            ]:
                UseStarknetCommitmentProof,
            CommitmentPrefixTypeComponent:
                ProvideCommitmentPrefixBytes,
            PacketCommitmentTypeComponent:
                ProvideBytesPacketCommitment,
            [
                AcknowledgementTypeProviderComponent,
                AckCommitmentHashTypeProviderComponent,
            ]:
                UseType<Vec<u8>>,
            PacketReceiptTypeComponent:
                ProvideBytesPacketReceipt,
            [
                PacketSrcPortIdGetterComponent,
                PacketDstChannelIdGetterComponent,
                PacketDstPortIdGetterComponent,
                PacketSequenceGetterComponent,
                PacketTimeoutHeightGetterComponent,
                PacketTimeoutTimestampGetterComponent,
            ]:
                CosmosPacketFieldReader,
            [
                PacketSrcChannelIdGetterComponent,
            ]:
                ReadPacketSrcStarknetFields,
            ChainStatusQuerierComponent:
                QueryStarknetChainStatus,
            BlockEventsQuerierComponent:
                RetryQueryBlockEvents<
                    5,
                    WaitBlockHeightAndQueryEvents<
                        GetStarknetBlockEvents
                    >>,
            [
                MessagesWithSignerAndNonceSenderComponent,
                TxMessageResponseParserComponent,
            ]:
                SendStarknetMessages,
            [
                MessageSenderComponent,
                MessagesWithSignerSenderComponent,
                NonceAllocatorComponent,
                TxResponsePollerComponent,
            ]:
                DefaultTxComponents::Provider,
            TxResponseQuerierComponent:
                QueryTransactionReceipt,
            PollTimeoutGetterComponent:
                FixedPollTimeoutSecs<300>,
            ContractCallerComponent:
                CallStarknetContract,
            ContractInvokerComponent:
                InvokeStarknetContract,
            ContractDeclarerComponent:
                DeclareSierraContract,
            ContractDeployerComponent:
                DeployStarknetContract,
            InvokeContractMessageBuilderComponent:
                BuildInvokeContractCall,
            NonceQuerierComponent:
                QueryStarknetNonce,
            IbcCommitmentPrefixGetterComponent:
                GetStarknetCommitmentPrefix,
            RetryableErrorComponent:
                ReturnRetryable<false>,
            TransferTokenMessageBuilderComponent:
                BuildTransferErc20TokenMessage,
            TokenTransferComponent:
                TransferErc20Token,
            TokenBalanceQuerierComponent:
                QueryErc20TokenBalance,
            BlockQuerierComponent:
                QueryStarknetBlock,
            BalanceQuerierComponent:
                QueryStarknetWalletBalance,
            [
                CreateClientEventComponent,
                ConnectionOpenInitEventComponent,
                ConnectionOpenTryEventComponent,
                ChannelOpenInitEventComponent,
                ChannelOpenTryEventComponent,
                SendPacketEventComponent,
                WriteAckEventComponent,
                EventExtractorComponent,
                MessageResponseExtractorComponent,
                PacketFromWriteAckEventBuilderComponent,
                PacketFromSendPacketEventBuilderComponent,
            ]:
                UseStarknetEvents,
            CreateClientMessageOptionsTypeComponent:
                ProvideNoCreateClientMessageOptionsType,
            CreateClientPayloadBuilderComponent:
                BuildStarknetCreateClientPayload,
            UpdateClientMessageBuilderComponent:
                BuildUpdateCometClientMessage,
            CreateClientMessageBuilderComponent:
                BuildCreateCometClientMessage,
            UpdateClientPayloadBuilderComponent:
                BuildStarknetUpdateClientPayload,
            [
                ClientStateQuerierComponent,
                ClientStateWithProofsQuerierComponent,
            ]:
                QueryCometClientState,
            [
                ConsensusStateQuerierComponent,
                ConsensusStateWithProofsQuerierComponent,
            ]:
                QueryCometConsensusState,
            ConsensusStateHeightQuerierComponent:
                QueryConsensusStateHeightsAndFindHeightBefore,
            ConsensusStateHeightsQuerierComponent:
                QueryLatestConsensusStateHeightAsHeights,
            ContractAddressQuerierComponent:
                GetContractAddressFromField,
            CounterpartyMessageHeightGetterComponent:
                GetCounterpartyCosmosHeightFromStarknetMessage,
            InitConnectionOptionsTypeComponent:
                ProvideCosmosInitConnectionOptionsType,
            [
                ConnectionOpenInitPayloadTypeComponent,
                ConnectionOpenTryPayloadTypeComponent,
                ConnectionOpenAckPayloadTypeComponent,
                ConnectionOpenConfirmPayloadTypeComponent,
            ]:
                ProvideConnectionPayloadTypes,
            [
                ChannelOpenTryPayloadTypeComponent,
                ChannelOpenAckPayloadTypeComponent,
                ChannelOpenConfirmPayloadTypeComponent,
            ]:
                ProvideChannelPayloadTypes,
            [
                ReceivePacketPayloadTypeComponent,
                AckPacketPayloadTypeProviderComponent,
                TimeoutUnorderedPacketPayloadTypeComponent,
            ]:
                ProvidePacketPayloadTypes,
            [
                ConnectionOpenInitPayloadBuilderComponent,
                ConnectionOpenTryPayloadBuilderComponent,
                ConnectionOpenAckPayloadBuilderComponent,
                ConnectionOpenConfirmPayloadBuilderComponent,
            ]:
                BuildConnectionHandshakePayload,
            [
                ChannelOpenTryPayloadBuilderComponent,
                ChannelOpenAckPayloadBuilderComponent,
                ChannelOpenConfirmPayloadBuilderComponent,
            ]:
                BuildChannelHandshakePayload,
            [
                ConnectionOpenInitMessageBuilderComponent,
                ConnectionOpenTryMessageBuilderComponent,
                ConnectionOpenAckMessageBuilderComponent,
                ConnectionOpenConfirmMessageBuilderComponent,
            ]:
                BuildStarknetConnectionHandshakeMessages,
            [
                ChannelOpenInitMessageBuilderComponent,
                ChannelOpenTryMessageBuilderComponent,
                ChannelOpenAckMessageBuilderComponent,
                ChannelOpenConfirmMessageBuilderComponent,
            ]:
                BuildStarknetChannelHandshakeMessages,
            [
                ReceivePacketMessageBuilderComponent,
                AckPacketMessageBuilderComponent,
                TimeoutUnorderedPacketMessageBuilderComponent,
            ]:
                BuildStarknetPacketMessages,
            [
                ReceivePacketPayloadBuilderComponent,
                AckPacketPayloadBuilderComponent,
                TimeoutUnorderedPacketPayloadBuilderComponent,
            ]:
                BuildPacketPayloads,
            [
                ConnectionEndQuerierComponent,
                ConnectionEndWithProofsQuerierComponent,
            ]:
                QueryConnectionEndFromStarknet,
            [
                ChannelEndQuerierComponent,
                ChannelEndWithProofsQuerierComponent,
            ]:
                QueryChannelEndFromStarknet,
            PacketCommitmentQuerierComponent:
                QueryStarknetPacketCommitment,
            PacketAckCommitmentQuerierComponent:
                QueryStarknetAckCommitment,
            PacketReceiptQuerierComponent:
                QueryStarknetPacketReceipt,
            [
                OutgoingPacketFilterComponent,
                IncomingPacketFilterComponent,
            ]:
                FilterStarknetPackets,
            CounterpartyChainIdQuerierComponent:
                QueryCosmosChainIdFromStarknetChannelId,
            CounterpartyConnectionIdQuerierComponent:
                QueryCounterpartyConnectionId,
            EventualAmountAsserterComponent:
                PollAssertEventualAmount,
            PollAssertDurationGetterComponent:
                ProvidePollAssertDuration<1, 300>,
            IbcTokenTransferMessageBuilderComponent:
                BuildStarknetIbcTransferMessage,
            PacketIsReceivedQuerierComponent:
                QueryPacketIsReceivedOnStarknet,
            PacketIsClearedQuerierComponent:
                QueryClearedPacketWithEmptyCommitment,
        }
    }
}
