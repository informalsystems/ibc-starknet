use std::sync::Arc;

use cgp::core::component::UseDelegate;
use cgp::core::error::{ErrorRaiserComponent, ErrorTypeComponent};
use cgp::core::field::impls::use_field::WithField;
use cgp::core::types::impls::WithType;
use cgp::prelude::*;
use hermes_cairo_encoding_components::types::as_felt::AsFelt;
use hermes_chain_type_components::traits::types::commitment_proof::HasCommitmentProofType;
use hermes_chain_type_components::traits::types::height::HasHeightType;
use hermes_chain_type_components::traits::types::message_response::HasMessageResponseType;
use hermes_cosmos_chain_components::components::delegate::DelegateCosmosChainComponents;
use hermes_cosmos_chain_components::types::connection::CosmosInitConnectionOptions;
use hermes_cosmos_relayer::contexts::chain::CosmosChain;
use hermes_encoding_components::impls::default_encoding::GetDefaultEncoding;
use hermes_encoding_components::traits::has_encoding::{
    DefaultEncodingGetter, EncodingGetterComponent, HasDefaultEncoding, ProvideEncodingType,
};
use hermes_encoding_components::types::AsBytes;
use hermes_error::impls::ProvideHermesError;
use hermes_logging_components::contexts::no_logger::ProvideNoLogger;
use hermes_logging_components::traits::has_logger::{
    GlobalLoggerGetterComponent, HasLogger, LoggerGetterComponent, LoggerTypeComponent,
};
use hermes_relayer_components::chain::traits::commitment_prefix::{
    HasCommitmentPrefixType, HasIbcCommitmentPrefix,
};
use hermes_relayer_components::chain::traits::message_builders::ack_packet::CanBuildAckPacketMessage;
use hermes_relayer_components::chain::traits::message_builders::channel_handshake::{
    CanBuildChannelOpenAckMessage, CanBuildChannelOpenConfirmMessage,
    CanBuildChannelOpenInitMessage, CanBuildChannelOpenTryMessage,
};
use hermes_relayer_components::chain::traits::message_builders::connection_handshake::{
    CanBuildConnectionOpenAckMessage, CanBuildConnectionOpenConfirmMessage,
    CanBuildConnectionOpenInitMessage, CanBuildConnectionOpenTryMessage,
};
use hermes_relayer_components::chain::traits::message_builders::create_client::CanBuildCreateClientMessage;
use hermes_relayer_components::chain::traits::message_builders::receive_packet::CanBuildReceivePacketMessage;
use hermes_relayer_components::chain::traits::message_builders::timeout_unordered_packet::CanBuildTimeoutUnorderedPacketMessage;
use hermes_relayer_components::chain::traits::message_builders::update_client::CanBuildUpdateClientMessage;
use hermes_relayer_components::chain::traits::packet::fields::{
    HasPacketDstChannelId, HasPacketDstPortId, HasPacketSequence, HasPacketSrcChannelId,
    HasPacketSrcPortId, HasPacketTimeoutHeight, HasPacketTimeoutTimestamp,
};
use hermes_relayer_components::chain::traits::packet::filter::{
    CanFilterIncomingPacket, CanFilterOutgoingPacket,
};
use hermes_relayer_components::chain::traits::payload_builders::ack_packet::CanBuildAckPacketPayload;
use hermes_relayer_components::chain::traits::payload_builders::channel_handshake::{
    CanBuildChannelOpenAckPayload, CanBuildChannelOpenConfirmPayload, CanBuildChannelOpenTryPayload,
};
use hermes_relayer_components::chain::traits::payload_builders::connection_handshake::{
    CanBuildConnectionOpenAckPayload, CanBuildConnectionOpenConfirmPayload,
    CanBuildConnectionOpenInitPayload, CanBuildConnectionOpenTryPayload,
};
use hermes_relayer_components::chain::traits::payload_builders::create_client::CanBuildCreateClientPayload;
use hermes_relayer_components::chain::traits::payload_builders::receive_packet::CanBuildReceivePacketPayload;
use hermes_relayer_components::chain::traits::payload_builders::timeout_unordered_packet::CanBuildTimeoutUnorderedPacketPayload;
use hermes_relayer_components::chain::traits::payload_builders::update_client::CanBuildUpdateClientPayload;
use hermes_relayer_components::chain::traits::queries::chain_status::{
    CanQueryChainHeight, CanQueryChainStatus,
};
use hermes_relayer_components::chain::traits::queries::channel_end::{
    CanQueryChannelEnd, CanQueryChannelEndWithProofs,
};
use hermes_relayer_components::chain::traits::queries::client_state::{
    CanQueryClientState, CanQueryClientStateWithLatestHeight, CanQueryClientStateWithProofs,
};
use hermes_relayer_components::chain::traits::queries::connection_end::{
    CanQueryConnectionEnd, CanQueryConnectionEndWithProofs,
};
use hermes_relayer_components::chain::traits::queries::consensus_state::{
    CanQueryConsensusState, CanQueryConsensusStateWithProofs,
};
use hermes_relayer_components::chain::traits::queries::consensus_state_height::{
    CanQueryConsensusStateHeight, CanQueryConsensusStateHeights,
};
use hermes_relayer_components::chain::traits::queries::packet_acknowledgement::CanQueryPacketAcknowledgement;
use hermes_relayer_components::chain::traits::queries::packet_commitment::CanQueryPacketCommitment;
use hermes_relayer_components::chain::traits::queries::packet_is_received::CanQueryPacketIsReceived;
use hermes_relayer_components::chain::traits::queries::packet_receipt::CanQueryPacketReceipt;
use hermes_relayer_components::chain::traits::send_message::{
    CanSendMessages, CanSendSingleMessage,
};
use hermes_relayer_components::chain::traits::types::chain_id::ChainIdGetter;
use hermes_relayer_components::chain::traits::types::client_state::{
    HasClientStateFields, HasClientStateType,
};
use hermes_relayer_components::chain::traits::types::connection::{
    HasConnectionEndType, HasConnectionOpenAckPayloadType, HasConnectionOpenConfirmPayloadType,
    HasConnectionOpenInitPayloadType, HasConnectionOpenTryPayloadType,
    HasInitConnectionOptionsType,
};
use hermes_relayer_components::chain::traits::types::consensus_state::HasConsensusStateType;
use hermes_relayer_components::chain::traits::types::create_client::HasCreateClientEvent;
use hermes_relayer_components::chain::traits::types::event::HasEventType;
use hermes_relayer_components::chain::traits::types::ibc::{
    HasClientIdType, HasConnectionIdType, HasCounterpartyMessageHeight,
};
use hermes_relayer_components::chain::traits::types::ibc_events::channel::HasChannelOpenTryEvent;
use hermes_relayer_components::chain::traits::types::ibc_events::connection::HasConnectionOpenTryEvent;
use hermes_relayer_components::chain::traits::types::ibc_events::write_ack::HasWriteAckEvent;
use hermes_relayer_components::chain::traits::types::packet::HasOutgoingPacketType;
use hermes_relayer_components::chain::traits::types::update_client::HasUpdateClientPayloadType;
use hermes_relayer_components::error::traits::retry::HasRetryableError;
use hermes_relayer_components::transaction::traits::poll_tx_response::CanPollTxResponse;
use hermes_relayer_components::transaction::traits::query_tx_response::CanQueryTxResponse;
use hermes_relayer_components::transaction::traits::submit_tx::CanSubmitTx;
use hermes_runtime::types::runtime::HermesRuntime;
use hermes_runtime_components::traits::runtime::{
    HasRuntime, RuntimeGetterComponent, RuntimeTypeComponent,
};
use hermes_starknet_chain_components::components::chain::{
    IsStarknetChainComponents, StarknetChainComponents,
};
use hermes_starknet_chain_components::components::starknet_to_cosmos::StarknetToCosmosComponents;
use hermes_starknet_chain_components::impls::account::GetStarknetAccountField;
use hermes_starknet_chain_components::impls::provider::GetStarknetProviderField;
use hermes_starknet_chain_components::traits::account::{
    HasStarknetAccount, StarknetAccountGetterComponent, StarknetAccountTypeComponent,
};
use hermes_starknet_chain_components::traits::client::JsonRpcClientGetter;
use hermes_starknet_chain_components::traits::contract::call::CanCallContract;
use hermes_starknet_chain_components::traits::contract::declare::CanDeclareContract;
use hermes_starknet_chain_components::traits::contract::deploy::CanDeployContract;
use hermes_starknet_chain_components::traits::contract::invoke::CanInvokeContract;
use hermes_starknet_chain_components::traits::provider::{
    HasStarknetProvider, StarknetProviderGetterComponent, StarknetProviderTypeComponent,
};
use hermes_starknet_chain_components::traits::queries::address::CanQueryContractAddress;
use hermes_starknet_chain_components::traits::queries::token_balance::CanQueryTokenBalance;
use hermes_starknet_chain_components::traits::transfer::CanTransferToken;
use hermes_starknet_chain_components::traits::types::blob::HasBlobType;
use hermes_starknet_chain_components::traits::types::method::HasSelectorType;
use hermes_starknet_chain_components::types::client_id::ClientId;
use hermes_starknet_chain_components::types::client_state::WasmStarknetClientState;
use hermes_starknet_chain_components::types::commitment_proof::StarknetCommitmentProof;
use hermes_starknet_chain_components::types::connection_id::{ConnectionEnd, ConnectionId};
use hermes_starknet_chain_components::types::consensus_state::WasmStarknetConsensusState;
use hermes_starknet_chain_components::types::cosmos::client_state::CometClientState;
use hermes_starknet_chain_components::types::cosmos::consensus_state::CometConsensusState;
use hermes_starknet_chain_components::types::cosmos::update::CometUpdateHeader;
use hermes_starknet_chain_components::types::event::StarknetEvent;
use hermes_starknet_chain_components::types::message_response::StarknetMessageResponse;
use hermes_starknet_test_components::impls::types::wallet::ProvideStarknetWalletType;
use hermes_test_components::chain::traits::types::address::HasAddressType;
use hermes_test_components::chain::traits::types::wallet::WalletTypeComponent;
use ibc::core::channel::types::packet::Packet;
use starknet::accounts::SingleOwnerAccount;
use starknet::core::types::Felt;
use starknet::providers::jsonrpc::HttpTransport;
use starknet::providers::JsonRpcClient;
use starknet::signers::LocalWallet;

use crate::contexts::encoding::cairo::StarknetCairoEncoding;
use crate::contexts::encoding::protobuf::StarknetProtobufEncoding;
use crate::impls::error::HandleStarknetChainError;

#[derive(HasField, Clone)]
pub struct StarknetChain {
    pub runtime: HermesRuntime,
    pub chain_id: Felt,
    pub rpc_client: Arc<JsonRpcClient<HttpTransport>>,
    pub account: SingleOwnerAccount<Arc<JsonRpcClient<HttpTransport>>, LocalWallet>,
    pub ibc_client_contract_address: Option<Felt>,
    pub ibc_core_contract_address: Option<Felt>,
}

pub struct StarknetChainContextComponents;

impl HasComponents for StarknetChain {
    type Components = StarknetChainContextComponents;
}

delegate_components! {
    StarknetChainContextComponents {
        ErrorTypeComponent: ProvideHermesError,
        ErrorRaiserComponent: UseDelegate<HandleStarknetChainError>,
        RuntimeTypeComponent: WithType<HermesRuntime>,
        RuntimeGetterComponent: WithField<symbol!("runtime")>,
        [
            LoggerTypeComponent,
            LoggerGetterComponent,
            GlobalLoggerGetterComponent,
        ]:
            ProvideNoLogger,
        EncodingGetterComponent: GetDefaultEncoding,
        [
            StarknetProviderTypeComponent,
            StarknetProviderGetterComponent,
        ]:
            GetStarknetProviderField<symbol!("rpc_client")>,
        [
            StarknetAccountTypeComponent,
            StarknetAccountGetterComponent,
        ]:
            GetStarknetAccountField<symbol!("account")>,
        WalletTypeComponent:
            ProvideStarknetWalletType,
    }
}

impl<Name> DelegateComponent<Name> for StarknetChainContextComponents
where
    Self: IsStarknetChainComponents<Name>,
{
    type Delegate = StarknetChainComponents;
}

delegate_components! {
    DelegateCosmosChainComponents {
        StarknetChain: StarknetToCosmosComponents,
    }
}

impl ProvideEncodingType<StarknetChain, AsFelt> for StarknetChainContextComponents {
    type Encoding = StarknetCairoEncoding;
}

impl DefaultEncodingGetter<StarknetChain, AsFelt> for StarknetChainContextComponents {
    fn default_encoding() -> &'static StarknetCairoEncoding {
        &StarknetCairoEncoding
    }
}

impl ProvideEncodingType<StarknetChain, AsBytes> for StarknetChainContextComponents {
    type Encoding = StarknetProtobufEncoding;
}

impl DefaultEncodingGetter<StarknetChain, AsBytes> for StarknetChainContextComponents {
    fn default_encoding() -> &'static StarknetProtobufEncoding {
        &StarknetProtobufEncoding
    }
}

impl JsonRpcClientGetter<StarknetChain> for StarknetChainContextComponents {
    fn json_rpc_client(chain: &StarknetChain) -> &JsonRpcClient<HttpTransport> {
        &chain.rpc_client
    }
}

impl ChainIdGetter<StarknetChain> for StarknetChainContextComponents {
    fn chain_id(chain: &StarknetChain) -> &Felt {
        &chain.chain_id
    }
}

pub trait CanUseStarknetChain:
    HasRuntime
    + HasLogger
    + HasHeightType<Height = u64>
    + HasEventType<Event = StarknetEvent>
    + HasMessageResponseType<MessageResponse = StarknetMessageResponse>
    + HasDefaultEncoding<AsBytes, Encoding = StarknetProtobufEncoding>
    + HasDefaultEncoding<AsFelt, Encoding = StarknetCairoEncoding>
    + HasCommitmentProofType<CommitmentProof = StarknetCommitmentProof>
    + HasAddressType<Address = Felt>
    + HasSelectorType<Selector = Felt>
    + HasBlobType<Blob = Vec<Felt>>
    + HasCommitmentPrefixType<CommitmentPrefix = Vec<u8>>
    + HasCommitmentProofType
    + HasClientStateType<CosmosChain, ClientState = WasmStarknetClientState>
    + HasConsensusStateType<CosmosChain, ConsensusState = WasmStarknetConsensusState>
    + HasClientIdType<CosmosChain, ClientId = ClientId>
    + HasConnectionIdType<CosmosChain, ConnectionId = ConnectionId>
    + HasConnectionEndType<CosmosChain, ConnectionEnd = ConnectionEnd>
    + HasInitConnectionOptionsType<CosmosChain, InitConnectionOptions = CosmosInitConnectionOptions>
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
    + HasStarknetProvider
    + HasStarknetAccount
    + CanQueryChainStatus
    + CanQueryChainHeight
    + CanSendMessages
    + CanSendSingleMessage
    + CanSubmitTx
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
    + CanBuildCreateClientMessage<CosmosChain>
    + CanBuildUpdateClientPayload<CosmosChain>
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
    + CanQueryPacketAcknowledgement<CosmosChain>
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
{
}

impl CanUseStarknetChain for StarknetChain {}

pub trait CanUseCosmosChainWithStarknet: HasClientStateType<StarknetChain, ClientState = CometClientState>
    + HasConsensusStateType<StarknetChain, ConsensusState = CometConsensusState>
    + HasUpdateClientPayloadType<StarknetChain, UpdateClientPayload = CometUpdateHeader>
    + HasInitConnectionOptionsType<StarknetChain, InitConnectionOptions = CosmosInitConnectionOptions>
    + HasCounterpartyMessageHeight<StarknetChain>
    + HasClientStateFields<StarknetChain>
    + CanQueryClientState<StarknetChain>
    + CanQueryConsensusState<StarknetChain>
    + CanBuildCreateClientMessage<StarknetChain>
    + CanBuildUpdateClientPayload<StarknetChain>
    + CanBuildUpdateClientMessage<StarknetChain>
    + CanQueryConsensusStateHeight<StarknetChain>
    + CanBuildCreateClientPayload<StarknetChain>
    + CanBuildUpdateClientPayload<StarknetChain>
    + CanBuildConnectionOpenTryPayload<StarknetChain>
    + HasConnectionEndType<StarknetChain>
    + CanBuildConnectionOpenInitPayload<StarknetChain>
    + CanBuildConnectionOpenTryPayload<StarknetChain>
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
{
}

impl CanUseCosmosChainWithStarknet for CosmosChain {}
