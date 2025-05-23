use core::ops::Deref;
use core::time::Duration;
use std::sync::{Arc, OnceLock};

use cgp::core::component::UseDelegate;
use cgp::core::error::{ErrorRaiserComponent, ErrorTypeProviderComponent, ErrorWrapperComponent};
use cgp::core::field::WithField;
use futures::lock::Mutex;
use hermes_cairo_encoding_components::types::as_felt::AsFelt;
use hermes_cairo_encoding_components::types::as_starknet_event::AsStarknetEvent;
use hermes_core::chain_components::traits::{
    BlockEventsQuerierComponent, BlockQuerierComponent, BlockTimeQuerierComponent,
    CanBuildAckPacketMessage, CanBuildAckPacketPayload, CanBuildChannelOpenAckMessage,
    CanBuildChannelOpenAckPayload, CanBuildChannelOpenConfirmMessage,
    CanBuildChannelOpenConfirmPayload, CanBuildChannelOpenInitMessage,
    CanBuildChannelOpenTryMessage, CanBuildChannelOpenTryPayload, CanBuildConnectionOpenAckMessage,
    CanBuildConnectionOpenAckPayload, CanBuildConnectionOpenConfirmMessage,
    CanBuildConnectionOpenConfirmPayload, CanBuildConnectionOpenInitMessage,
    CanBuildConnectionOpenInitPayload, CanBuildConnectionOpenTryMessage,
    CanBuildConnectionOpenTryPayload, CanBuildCreateClientMessage, CanBuildCreateClientPayload,
    CanBuildPacketFromWriteAck, CanBuildReceivePacketMessage, CanBuildReceivePacketPayload,
    CanBuildTimeoutUnorderedPacketMessage, CanBuildTimeoutUnorderedPacketPayload,
    CanBuildUpdateClientMessage, CanBuildUpdateClientPayload, CanExtractFromEvent,
    CanExtractFromMessageResponse, CanFilterIncomingPacket, CanFilterOutgoingPacket, CanQueryBlock,
    CanQueryBlockEvents, CanQueryBlockTime, CanQueryChainHeight, CanQueryChainStatus,
    CanQueryChannelEnd, CanQueryChannelEndWithProofs, CanQueryClientState,
    CanQueryClientStateWithLatestHeight, CanQueryClientStateWithProofs, CanQueryConnectionEnd,
    CanQueryConnectionEndWithProofs, CanQueryConsensusState, CanQueryConsensusStateHeight,
    CanQueryConsensusStateHeights, CanQueryConsensusStateWithProofs, CanQueryCounterpartyChainId,
    CanQueryPacketAckCommitment, CanQueryPacketCommitment, CanQueryPacketIsReceived,
    CanQueryPacketReceipt, CanSendMessages, CanSendSingleMessage, ChainStatusQuerierComponent,
    HasAcknowledgementType, HasBlockType, HasChainStatusType, HasChannelEndType, HasChannelIdType,
    HasChannelOpenTryEvent, HasClientIdType, HasClientStateFields, HasClientStateType,
    HasCommitmentPrefixType, HasConnectionEndType, HasConnectionIdType,
    HasConnectionOpenAckPayloadType, HasConnectionOpenConfirmPayloadType,
    HasConnectionOpenInitPayloadType, HasConnectionOpenTryEvent, HasConnectionOpenTryPayloadType,
    HasConsensusStateType, HasCounterpartyMessageHeight, HasCreateClientEvent,
    HasCreateClientMessageOptionsType, HasCreateClientPayloadOptionsType, HasEventType,
    HasIbcCommitmentPrefix, HasInitChannelOptionsType, HasInitConnectionOptionsType,
    HasOutgoingPacketType, HasPacketCommitmentType, HasPacketDstChannelId, HasPacketDstPortId,
    HasPacketReceiptType, HasPacketSequence, HasPacketSrcChannelId, HasPacketSrcPortId,
    HasPacketTimeoutHeight, HasPacketTimeoutTimestamp, HasPortIdType, HasSendPacketEvent,
    HasSequenceType, HasTimeoutType, HasUpdateClientPayloadType, HasWriteAckEvent,
    PacketCommitmentQuerierComponent, PollIntervalGetterComponent,
};
use hermes_core::chain_type_components::traits::{
    ChainIdGetterComponent, HasAddressType, HasChainId, HasCommitmentProofType, HasHeightType,
    HasMessageResponseType, HasTimeType,
};
use hermes_core::encoding_components::traits::{
    DefaultEncodingGetter, DefaultEncodingGetterComponent, EncodingGetter, EncodingGetterComponent,
    EncodingTypeProviderComponent, HasDefaultEncoding,
};
use hermes_core::encoding_components::types::AsBytes;
use hermes_core::logging_components::traits::LoggerComponent;
use hermes_core::relayer_components::error::traits::HasRetryableError;
use hermes_core::relayer_components::transaction::impls::GetGlobalNonceMutex;
use hermes_core::relayer_components::transaction::traits::{
    CanAllocateNonce, CanPollTxResponse, CanQueryNonce, CanQueryTxResponse,
    CanSendMessagesWithSigner, CanSendMessagesWithSignerAndNonce, DefaultSignerGetterComponent,
    HasSignerType, NonceAllocationMutexGetterComponent,
};
use hermes_core::runtime_components::traits::{
    HasRuntime, RuntimeGetterComponent, RuntimeTypeProviderComponent,
};
use hermes_core::test_components::chain::traits::{
    CanAssertEventualAmount, CanBuildIbcTokenTransferMessages, CanCalculateIbcTransferTimeout,
    CanConvertIbcTransferredAmount, CanIbcTransferToken, CanQueryBalance, HasMemoType,
    IbcTransferTimeoutCalculatorComponent, IbcTransferredAmountConverterComponent,
};
use hermes_cosmos::chain_components::types::{
    CosmosInitChannelOptions, CosmosInitConnectionOptions, CosmosUpdateClientPayload,
    Secp256k1KeyPair, Time,
};
use hermes_cosmos::chain_preset::delegate::DelegateCosmosChainComponents;
use hermes_cosmos::error::impls::UseHermesError;
use hermes_cosmos::relayer::contexts::CosmosChain;
use hermes_cosmos::runtime::types::runtime::HermesRuntime;
use hermes_cosmos::tracing_logging_components::contexts::TracingLogger;
use hermes_prelude::*;
use hermes_starknet_chain_components::components::{
    StarknetChainComponents, StarknetToCosmosComponents,
};
use hermes_starknet_chain_components::impls::{StarknetAddress, StarknetCreateClientEvent};
use hermes_starknet_chain_components::traits::{
    AccountFromSignerBuilderComponent, CanCallContract, CanDeclareContract, CanDeployContract,
    CanInvokeContract, CanQueryContractAddress, CanQueryTokenBalance, CanTransferToken,
    CosmosTokenAddressOnStarknetQuerierComponent, HasBlobType, HasSelectorType, HasStarknetClient,
    HasStarknetProofSigner, StarknetAccountTypeProviderComponent, StarknetClientGetterComponent,
    StarknetClientTypeProviderComponent, StarknetProofSignerGetterComponent,
    StarknetProofSignerTypeProviderComponent,
};
use hermes_starknet_chain_components::types::{
    ChannelEnd, ChannelId, ClientId, CometClientState, CometConsensusState, ConnectionEnd,
    ConnectionId, StarknetChainStatus, StarknetCommitmentProof, StarknetCreateClientPayloadOptions,
    StarknetEvent, StarknetMessageResponse, StarknetUpdateClientPayload, StarknetWallet,
    WasmStarknetClientState, WasmStarknetConsensusState, WriteAcknowledgementEvent,
};
use ibc::core::channel::types::packet::Packet;
use ibc::core::host::types::identifiers::{ChainId, PortId as IbcPortId, Sequence};
use ibc::primitives::Timestamp;
use starknet::core::types::Felt;
use starknet::providers::jsonrpc::HttpTransport;
use starknet::providers::JsonRpcClient;

use crate::contexts::{
    StarknetCairoEncoding, StarknetEventEncoding, StarknetProtobufEncoding,
    UseStarknetCairoEncoding,
};
use crate::impls::{BuildStarknetAccount, HandleStarknetChainError};
use crate::types::StarknetAccount;

#[cgp_context(StarknetChainContextComponents: StarknetChainComponents)]
#[derive(Clone)]
pub struct StarknetChain {
    pub fields: Arc<StarknetChainFields>,
}

#[derive(HasField, Clone)]
pub struct StarknetChainFields {
    pub runtime: HermesRuntime,
    pub chain_id: ChainId,
    pub starknet_client: Arc<JsonRpcClient<HttpTransport>>,
    pub ibc_client_contract_address: OnceLock<StarknetAddress>,
    pub ibc_core_contract_address: OnceLock<StarknetAddress>,
    pub ibc_ics20_contract_address: OnceLock<StarknetAddress>,
    pub event_encoding: StarknetEventEncoding,
    pub poll_interval: Duration,
    pub block_time: Duration,
    // FIXME: only needed for demo2
    pub proof_signer: Secp256k1KeyPair,
    pub nonce_mutex: Arc<Mutex<()>>,
    pub signer: StarknetWallet,
}

impl Deref for StarknetChain {
    type Target = StarknetChainFields;

    fn deref(&self) -> &StarknetChainFields {
        &self.fields
    }
}

delegate_components! {
    StarknetChainContextComponents {
        [
            ErrorTypeProviderComponent,
            ErrorWrapperComponent,
        ]: UseHermesError,
        ErrorRaiserComponent:
            UseDelegate<HandleStarknetChainError>,
        RuntimeTypeProviderComponent:
            UseType<HermesRuntime>,
        [
            EncodingTypeProviderComponent<AsFelt>,
            EncodingGetterComponent<AsFelt>,
            DefaultEncodingGetterComponent<AsFelt>,
        ]:
            UseStarknetCairoEncoding,
        EncodingTypeProviderComponent<AsStarknetEvent>:
            UseType<StarknetEventEncoding>,
        EncodingTypeProviderComponent<AsBytes>:
            UseType<StarknetProtobufEncoding>,
        ChainIdGetterComponent:
            UseField<symbol!("chain_id")>,
        RuntimeGetterComponent:
            UseField<symbol!("runtime")>,
        PollIntervalGetterComponent:
            UseField<symbol!("poll_interval")>,
        LoggerComponent:
            TracingLogger,
        [
            StarknetClientTypeProviderComponent,
            StarknetClientGetterComponent,
        ]:
            WithField<symbol!("starknet_client")>,
        StarknetAccountTypeProviderComponent:
            UseType<StarknetAccount>,
        StarknetProofSignerTypeProviderComponent:
            UseType<Secp256k1KeyPair>,
        StarknetProofSignerGetterComponent:
            UseField<symbol!("proof_signer")>,
        DefaultSignerGetterComponent:
            UseField<symbol!("signer")>,
        NonceAllocationMutexGetterComponent:
            GetGlobalNonceMutex<symbol!("nonce_mutex")>,
        BlockTimeQuerierComponent:
            UseField<symbol!("block_time")>,
        AccountFromSignerBuilderComponent:
            BuildStarknetAccount,
    }
}

delegate_components! {
    DelegateCosmosChainComponents {
        StarknetChain: StarknetToCosmosComponents::Provider,
    }
}

#[cgp_provider(EncodingGetterComponent<AsStarknetEvent>)]
impl EncodingGetter<StarknetChain, AsStarknetEvent> for StarknetChainContextComponents {
    fn encoding(chain: &StarknetChain) -> &StarknetEventEncoding {
        &chain.event_encoding
    }
}

#[cgp_provider(DefaultEncodingGetterComponent<AsBytes>)]
impl DefaultEncodingGetter<StarknetChain, AsBytes> for StarknetChainContextComponents {
    fn default_encoding() -> &'static StarknetProtobufEncoding {
        &StarknetProtobufEncoding
    }
}

pub trait CanUseStarknetChain:
    HasRuntime
    + HasHeightType<Height = u64>
    + HasTimeType<Time = Time>
    + HasTimeoutType<Timeout = Timestamp>
    + HasEventType<Event = StarknetEvent>
    + HasMessageResponseType<MessageResponse = StarknetMessageResponse>
    + HasDefaultEncoding<AsBytes, Encoding = StarknetProtobufEncoding>
    + HasDefaultEncoding<AsFelt, Encoding = StarknetCairoEncoding>
    + HasCommitmentProofType<CommitmentProof = StarknetCommitmentProof>
    + HasAddressType<Address = StarknetAddress>
    + HasChainStatusType<ChainStatus = StarknetChainStatus>
    + HasBlockType<Block = StarknetChainStatus>
    + HasChainId<ChainId = ChainId>
    + HasSelectorType<Selector = Felt>
    + HasBlobType<Blob = Vec<Felt>>
    + HasCommitmentPrefixType<CommitmentPrefix = Vec<u8>>
    + HasSignerType
    + HasClientStateType<CosmosChain, ClientState = WasmStarknetClientState>
    + HasConsensusStateType<CosmosChain, ConsensusState = WasmStarknetConsensusState>
    + HasClientIdType<CosmosChain, ClientId = ClientId>
    + HasConnectionIdType<CosmosChain, ConnectionId = ConnectionId>
    + HasConnectionEndType<CosmosChain, ConnectionEnd = ConnectionEnd>
    + HasChannelIdType<CosmosChain, ChannelId = ChannelId>
    + HasChannelEndType<CosmosChain, ChannelEnd = ChannelEnd>
    + HasPortIdType<CosmosChain, PortId = IbcPortId>
    + HasInitConnectionOptionsType<CosmosChain, InitConnectionOptions = CosmosInitConnectionOptions>
    + HasInitChannelOptionsType<CosmosChain, InitChannelOptions = CosmosInitChannelOptions>
    + HasConnectionOpenInitPayloadType<CosmosChain>
    + HasConnectionOpenTryPayloadType<CosmosChain>
    + HasConnectionOpenAckPayloadType<CosmosChain>
    + HasConnectionOpenConfirmPayloadType<CosmosChain>
    + HasOutgoingPacketType<CosmosChain, OutgoingPacket = Packet>
    + HasPacketSrcChannelId<CosmosChain>
    + HasPacketSrcPortId<CosmosChain>
    + HasPacketDstChannelId<CosmosChain>
    + HasPacketDstPortId<CosmosChain>
    + HasPacketSequence<CosmosChain>
    + HasPacketTimeoutHeight<CosmosChain>
    + HasPacketTimeoutTimestamp<CosmosChain>
    + HasStarknetClient
    + CanQueryChainStatus
    + CanQueryBlock
    + CanQueryChainHeight
    + CanQueryBlockEvents
    + CanQueryBlockTime
    + CanSendMessages
    + CanSendSingleMessage
    + CanQueryTxResponse
    + CanPollTxResponse
    + CanCallContract
    + CanInvokeContract
    + CanDeclareContract
    + CanDeployContract
    + CanQueryTokenBalance
    + CanTransferToken
    + HasIbcCommitmentPrefix
    + HasRetryableError
    + CanQueryContractAddress<symbol!("ibc_client_contract_address")>
    + CanQueryContractAddress<symbol!("ibc_core_contract_address")>
    + HasCreateClientEvent<CosmosChain>
    + CanBuildCreateClientPayload<CosmosChain>
    + HasCreateClientMessageOptionsType<CosmosChain, CreateClientMessageOptions = ()>
    + HasCreateClientPayloadOptionsType<
        CosmosChain,
        CreateClientPayloadOptions = StarknetCreateClientPayloadOptions,
    > + CanBuildCreateClientMessage<CosmosChain>
    + CanBuildUpdateClientPayload<CosmosChain, UpdateClientPayload = StarknetUpdateClientPayload>
    + CanBuildUpdateClientMessage<CosmosChain>
    + CanQueryClientState<CosmosChain>
    + CanQueryClientStateWithProofs<CosmosChain>
    + CanQueryClientStateWithLatestHeight<CosmosChain>
    + CanQueryConsensusState<CosmosChain>
    + CanQueryConsensusStateWithProofs<CosmosChain>
    + CanQueryConsensusStateHeights<CosmosChain>
    + CanQueryConsensusStateHeight<CosmosChain>
    + CanQueryConnectionEnd<CosmosChain>
    + CanQueryConnectionEndWithProofs<CosmosChain>
    + CanQueryChannelEnd<CosmosChain>
    + CanQueryChannelEndWithProofs<CosmosChain>
    + CanQueryNonce
    + CanAllocateNonce
    + CanSendMessagesWithSigner
    + CanSendMessagesWithSignerAndNonce
    + HasCounterpartyMessageHeight<CosmosChain>
    + HasInitConnectionOptionsType<CosmosChain>
    + CanBuildConnectionOpenInitPayload<CosmosChain>
    + CanBuildConnectionOpenTryPayload<CosmosChain>
    + CanBuildConnectionOpenAckPayload<CosmosChain>
    + CanBuildConnectionOpenConfirmPayload<CosmosChain>
    + CanBuildConnectionOpenInitMessage<CosmosChain>
    + CanBuildConnectionOpenTryMessage<CosmosChain>
    + CanBuildConnectionOpenAckMessage<CosmosChain>
    + CanBuildConnectionOpenConfirmMessage<CosmosChain>
    + CanBuildChannelOpenTryPayload<CosmosChain>
    + CanBuildChannelOpenAckPayload<CosmosChain>
    + CanBuildChannelOpenConfirmPayload<CosmosChain>
    + CanBuildChannelOpenInitMessage<CosmosChain>
    + CanBuildChannelOpenTryMessage<CosmosChain>
    + CanBuildChannelOpenAckMessage<CosmosChain>
    + CanBuildChannelOpenConfirmMessage<CosmosChain>
    + HasConnectionOpenTryEvent<CosmosChain>
    + HasChannelOpenTryEvent<CosmosChain>
    + CanQueryPacketCommitment<CosmosChain>
    + CanQueryPacketAckCommitment<CosmosChain>
    + CanQueryPacketReceipt<CosmosChain>
    + CanBuildReceivePacketPayload<CosmosChain>
    + CanBuildAckPacketPayload<CosmosChain>
    + CanBuildTimeoutUnorderedPacketPayload<CosmosChain>
    + CanBuildReceivePacketMessage<CosmosChain>
    + CanBuildAckPacketMessage<CosmosChain>
    + CanBuildTimeoutUnorderedPacketMessage<CosmosChain>
    + HasWriteAckEvent<CosmosChain>
    + CanFilterOutgoingPacket<CosmosChain>
    + CanFilterIncomingPacket<CosmosChain>
    + CanQueryPacketIsReceived<CosmosChain>
    + HasPacketCommitmentType<CosmosChain, PacketCommitment = Vec<u8>>
    + HasAcknowledgementType<CosmosChain, Acknowledgement = Vec<u8>>
    + HasPacketReceiptType<CosmosChain, PacketReceipt = Vec<u8>>
    + HasSequenceType<CosmosChain, Sequence = Sequence>
    + CanQueryBalance
    + HasMemoType
    + HasCreateClientEvent<CosmosChain, CreateClientEvent = StarknetCreateClientEvent>
    + HasSendPacketEvent<CosmosChain>
    + HasWriteAckEvent<CosmosChain, WriteAckEvent = WriteAcknowledgementEvent>
    + CanExtractFromMessageResponse<StarknetCreateClientEvent>
    + CanExtractFromEvent<WriteAcknowledgementEvent>
    + CanBuildPacketFromWriteAck<CosmosChain>
    + CanQueryCounterpartyChainId<CosmosChain>
    + CanAssertEventualAmount
    + CanIbcTransferToken<CosmosChain>
    + CanBuildIbcTokenTransferMessages<CosmosChain>
    + HasStarknetProofSigner<ProofSigner = Secp256k1KeyPair>
    + CanUseComponent<PacketCommitmentQuerierComponent, CosmosChain>
{
}

impl CanUseStarknetChain for StarknetChain {}

check_components! {
    CanUseStarknetChain2 for StarknetChain {
        ChainStatusQuerierComponent,
        BlockQuerierComponent,
        BlockEventsQuerierComponent,
        BlockTimeQuerierComponent,
        CosmosTokenAddressOnStarknetQuerierComponent,
        [
            IbcTransferTimeoutCalculatorComponent,
            IbcTransferredAmountConverterComponent,
        ]:
            CosmosChain,
    }
}

pub trait CanUseCosmosChainWithStarknet: HasClientStateType<StarknetChain, ClientState = CometClientState>
    + HasConsensusStateType<StarknetChain, ConsensusState = CometConsensusState>
    + HasUpdateClientPayloadType<StarknetChain, UpdateClientPayload = CosmosUpdateClientPayload>
    + HasInitConnectionOptionsType<StarknetChain, InitConnectionOptions = CosmosInitConnectionOptions>
    + HasChainId<ChainId = ChainId>
    + HasCounterpartyMessageHeight<StarknetChain>
    + HasClientStateFields<StarknetChain>
    + CanQueryClientState<StarknetChain>
    + CanQueryConsensusState<StarknetChain>
    + CanBuildCreateClientMessage<StarknetChain>
    + CanBuildUpdateClientMessage<StarknetChain>
    + CanQueryConsensusStateHeight<StarknetChain>
    + CanBuildCreateClientPayload<StarknetChain>
    + CanBuildUpdateClientPayload<StarknetChain>
    + CanBuildConnectionOpenTryPayload<StarknetChain>
    + HasConnectionEndType<StarknetChain>
    + CanBuildConnectionOpenInitPayload<StarknetChain>
    + CanBuildConnectionOpenAckPayload<StarknetChain>
    + CanBuildConnectionOpenConfirmPayload<StarknetChain>
    + CanBuildConnectionOpenInitMessage<StarknetChain>
    + CanBuildConnectionOpenTryMessage<StarknetChain>
    + CanBuildConnectionOpenAckMessage<StarknetChain>
    + CanBuildConnectionOpenConfirmMessage<StarknetChain>
    + CanBuildChannelOpenInitMessage<StarknetChain>
    + CanBuildChannelOpenTryMessage<StarknetChain>
    + CanBuildChannelOpenAckMessage<StarknetChain>
    + CanBuildChannelOpenConfirmMessage<StarknetChain>
    + HasPacketSrcChannelId<StarknetChain>
    + HasPacketSrcPortId<StarknetChain>
    + HasPacketDstChannelId<StarknetChain>
    + HasPacketDstPortId<StarknetChain>
    + HasPacketSequence<StarknetChain>
    + HasPacketTimeoutHeight<StarknetChain>
    + HasPacketTimeoutTimestamp<StarknetChain>
    + CanBuildReceivePacketPayload<StarknetChain>
    + CanBuildAckPacketPayload<StarknetChain>
    + CanBuildTimeoutUnorderedPacketPayload<StarknetChain>
    + CanBuildReceivePacketMessage<StarknetChain>
    + CanBuildAckPacketMessage<StarknetChain>
    + CanBuildTimeoutUnorderedPacketMessage<StarknetChain>
    + CanFilterOutgoingPacket<StarknetChain>
    + CanFilterIncomingPacket<StarknetChain>
    + HasAcknowledgementType<StarknetChain, Acknowledgement = Vec<u8>>
    + HasSequenceType<StarknetChain, Sequence = Sequence>
    + HasCreateClientEvent<StarknetChain>
    + HasSendPacketEvent<StarknetChain>
    + HasWriteAckEvent<StarknetChain>
    + CanBuildPacketFromWriteAck<StarknetChain>
    + CanQueryCounterpartyChainId<StarknetChain>
    + CanUseComponent<PacketCommitmentQuerierComponent, StarknetChain>
    + CanCalculateIbcTransferTimeout<StarknetChain>
    + CanConvertIbcTransferredAmount<StarknetChain>
{
}

impl CanUseCosmosChainWithStarknet for CosmosChain {}
