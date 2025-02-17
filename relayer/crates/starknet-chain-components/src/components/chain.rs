use cgp::core::component::WithProvider;
use cgp::core::types::UseDelegatedType;
use cgp::prelude::*;
use hermes_chain_components::impls::payload_builders::channel::BuildChannelHandshakePayload;
use hermes_chain_components::impls::payload_builders::connection::BuildConnectionHandshakePayload;
use hermes_chain_components::impls::payload_builders::packet::BuildPacketPayloads;
use hermes_chain_components::impls::queries::block_events::{
    RetryQueryBlockEvents, WaitBlockHeightAndQueryEvents,
};
use hermes_chain_components::impls::queries::consensus_state_height::QueryConsensusStateHeightsAndFindHeightBefore;
use hermes_chain_components::impls::queries::consensus_state_heights::QueryLatestConsensusStateHeightAsHeights;
use hermes_chain_components::impls::types::ack::ProvideBytesAcknowlegement;
use hermes_chain_components::impls::types::commitment::ProvideBytesPacketCommitment;
use hermes_chain_components::impls::types::commitment_prefix::ProvideCommitmentPrefixBytes;
use hermes_chain_components::impls::types::payloads::channel::ProvideChannelPayloadTypes;
use hermes_chain_components::impls::types::payloads::connection::ProvideConnectionPayloadTypes;
use hermes_chain_components::impls::types::payloads::packet::ProvidePacketPayloadTypes;
use hermes_chain_components::impls::types::receipt::ProvideBytesPacketReceipt;
use hermes_chain_components::traits::commitment_prefix::IbcCommitmentPrefixGetterComponent;
pub use hermes_chain_components::traits::packet::from_send_packet::PacketFromSendPacketEventBuilderComponent;
pub use hermes_cosmos_chain_components::components::client::*;
use hermes_cosmos_chain_components::impls::channel::init_channel_options::ProvideCosmosInitChannelOptionsType;
use hermes_cosmos_chain_components::impls::connection::init_connection_options::ProvideCosmosInitConnectionOptionsType;
use hermes_cosmos_chain_components::impls::packet::packet_fields::CosmosPacketFieldReader;
use hermes_cosmos_chain_components::impls::types::chain::ProvideCosmosChainTypes;
use hermes_cosmos_chain_components::impls::types::create_client_options::ProvideNoCreateClientMessageOptionsType;
pub use hermes_relayer_components::chain::traits::queries::chain_status::ChainStatusQuerierComponent;
pub use hermes_relayer_components::chain::traits::send_message::MessageSenderComponent;
pub use hermes_relayer_components::chain::traits::types::chain_id::ChainIdTypeComponent;
pub use hermes_relayer_components::chain::traits::types::client_state::ClientStateTypeComponent;
pub use hermes_relayer_components::chain::traits::types::consensus_state::ConsensusStateTypeComponent;
pub use hermes_relayer_components::chain::traits::types::event::EventTypeComponent;
pub use hermes_relayer_components::chain::traits::types::height::{
    HeightFieldComponent, HeightTypeComponent,
};
pub use hermes_relayer_components::chain::traits::types::message::MessageTypeComponent;
pub use hermes_relayer_components::chain::traits::types::status::ChainStatusTypeComponent;
use hermes_relayer_components::error::impls::retry::ReturnRetryable;
pub use hermes_relayer_components::error::traits::retry::RetryableErrorComponent;
pub use hermes_relayer_components::transaction::impls::poll_tx_response::PollTimeoutGetterComponent;
use hermes_relayer_components::transaction::impls::poll_tx_response::PollTxResponse;
pub use hermes_relayer_components::transaction::traits::poll_tx_response::TxResponsePollerComponent;
pub use hermes_relayer_components::transaction::traits::query_tx_response::TxResponseQuerierComponent;
pub use hermes_relayer_components::transaction::traits::submit_tx::TxSubmitterComponent;
pub use hermes_relayer_components::transaction::traits::types::transaction::TransactionTypeComponent;
pub use hermes_relayer_components::transaction::traits::types::tx_hash::TransactionHashTypeComponent;
pub use hermes_relayer_components::transaction::traits::types::tx_response::TxResponseTypeComponent;
use hermes_test_components::chain::impls::assert::default_assert_duration::ProvideDefaultPollAssertDuration;
use hermes_test_components::chain::impls::assert::poll_assert_eventual_amount::PollAssertEventualAmount;
use hermes_test_components::chain::impls::ibc_transfer::SendIbcTransferMessage;
pub use hermes_test_components::chain::traits::assert::eventual_amount::EventualAmountAsserterComponent;
pub use hermes_test_components::chain::traits::assert::poll_assert::PollAssertDurationGetterComponent;
use hermes_test_components::chain::traits::messages::ibc_transfer::IbcTokenTransferMessageBuilderComponent;
use hermes_test_components::chain::traits::queries::balance::BalanceQuerierComponent;
use hermes_test_components::chain::traits::transfer::ibc_transfer::TokenIbcTransferrerComponent;
use hermes_test_components::chain::traits::transfer::string_memo::ProvideStringMemoType;
pub use hermes_test_components::chain::traits::types::address::AddressTypeComponent;
pub use hermes_test_components::chain::traits::types::amount::AmountTypeComponent;
pub use hermes_test_components::chain::traits::types::denom::DenomTypeComponent;
use hermes_test_components::chain::traits::types::memo::MemoTypeComponent;

use crate::components::types::StarknetChainTypes;
use crate::impls::commitment_prefix::GetStarknetCommitmentPrefix;
use crate::impls::contract::call::CallStarknetContract;
use crate::impls::contract::declare::DeclareSierraContract;
use crate::impls::contract::deploy::DeployStarknetContract;
use crate::impls::contract::invoke::InvokeStarknetContract;
use crate::impls::contract::message::BuildInvokeContractCall;
use crate::impls::counterparty_message_height::GetCounterpartyCosmosHeightFromStarknetMessage;
use crate::impls::events::UseStarknetEvents;
use crate::impls::messages::channel::BuildStarknetChannelHandshakeMessages;
use crate::impls::messages::connection::BuildStarknetConnectionHandshakeMessages;
use crate::impls::messages::create_client::BuildCreateCometClientMessage;
use crate::impls::messages::ibc_transfer::BuildStarknetIbcTransferMessage;
use crate::impls::messages::packet::BuildStarknetPacketMessages;
use crate::impls::messages::update_client::BuildUpdateCometClientMessage;
use crate::impls::packet_fields::ReadPacketSrcStarknetFields;
use crate::impls::packet_filter::FilterStarknetPackets;
use crate::impls::payload_builders::create_client::BuildStarknetCreateClientPayload;
use crate::impls::payload_builders::update_client::BuildStarknetUpdateClientPayload;
use crate::impls::queries::ack_commitment::QueryStarknetAckCommitment;
use crate::impls::queries::balance::QueryStarknetWalletBalance;
use crate::impls::queries::block_events::GetStarknetBlockEvents;
use crate::impls::queries::channel_end::QueryChannelEndFromStarknet;
use crate::impls::queries::client_state::QueryCometClientState;
use crate::impls::queries::connection_end::QueryConnectionEndFromStarknet;
use crate::impls::queries::consensus_state::QueryCometConsensusState;
use crate::impls::queries::contract_address::GetContractAddressFromField;
use crate::impls::queries::counterparty_chain_id::QueryCosmosChainIdFromStarknetChannelId;
use crate::impls::queries::packet_commitment::QueryStarknetPacketCommitment;
use crate::impls::queries::packet_receipt::QueryStarknetPacketReceipt;
use crate::impls::queries::packet_received::QueryPacketIsReceivedOnStarknet;
use crate::impls::queries::status::QueryStarknetChainStatus;
use crate::impls::queries::token_balance::QueryErc20TokenBalance;
use crate::impls::send_message::SendCallMessages;
use crate::impls::submit_tx::SubmitCallTransaction;
use crate::impls::transfer::TransferErc20Token;
use crate::impls::tx_response::{DefaultPollTimeout, QueryTransactionReceipt};
use crate::impls::types::address::ProvideFeltAddressType;
use crate::impls::types::amount::ProvideU256Amount;
use crate::impls::types::blob::ProvideFeltBlobType;
use crate::impls::types::chain_id::ProvideFeltChainId;
use crate::impls::types::client::ProvideStarknetIbcClientTypes;
use crate::impls::types::commitment_proof::UseStarknetCommitmentProof;
use crate::impls::types::contract::ProvideStarknetContractTypes;
use crate::impls::types::denom::ProvideTokenAddressDenom;
use crate::impls::types::event::ProvideStarknetEvent;
use crate::impls::types::height::ProvideStarknetHeight;
use crate::impls::types::message::ProvideCallMessage;
use crate::impls::types::method::ProvideFeltSelector;
use crate::impls::types::payloads::ProvideStarknetPayloadTypes;
use crate::impls::types::status::ProvideStarknetChainStatusType;
use crate::impls::types::transaction::ProvideCallTransaction;
use crate::impls::types::tx_hash::ProvideFeltTxHash;
use crate::impls::types::tx_response::ProvideStarknetTxResponse;
pub use crate::traits::contract::call::ContractCallerComponent;
pub use crate::traits::contract::declare::ContractDeclarerComponent;
pub use crate::traits::contract::deploy::ContractDeployerComponent;
pub use crate::traits::contract::invoke::ContractInvokerComponent;
pub use crate::traits::contract::message::InvokeContractMessageBuilderComponent;
pub use crate::traits::messages::transfer::TransferTokenMessageBuilderComponent;
pub use crate::traits::queries::address::ContractAddressQuerierComponent;
pub use crate::traits::queries::token_balance::TokenBalanceQuerierComponent;
pub use crate::traits::transfer::TokenTransferComponent;
pub use crate::traits::types::blob::BlobTypeComponent;
pub use crate::traits::types::contract_class::{
    ContractClassHashTypeComponent, ContractClassTypeComponent,
};
pub use crate::traits::types::method::SelectorTypeComponent;
use crate::types::message_response::UseStarknetMessageResponse;
use crate::types::messages::erc20::transfer::BuildTransferErc20TokenMessage;

cgp_preset! {
    StarknetChainComponents {
        ChainIdTypeComponent:
            ProvideFeltChainId,
        [
            HeightTypeComponent,
            HeightFieldComponent,
            HeightIncrementerComponent,
        ]:
            ProvideStarknetHeight,
        ChainStatusTypeComponent:
            ProvideStarknetChainStatusType,
        AddressTypeComponent:
            ProvideFeltAddressType,
        BlobTypeComponent:
            ProvideFeltBlobType,
        MessageTypeComponent:
            ProvideCallMessage,
        EventTypeComponent:
            ProvideStarknetEvent,
        [
            MessageResponseTypeComponent,
            MessageResponseEventsGetterComponent,
        ]:
            UseStarknetMessageResponse,
        AmountTypeComponent:
            ProvideU256Amount,
        DenomTypeComponent:
            ProvideTokenAddressDenom,
        MemoTypeComponent:
            ProvideStringMemoType,
        TokenIbcTransferrerComponent:
            SendIbcTransferMessage,
        TransactionTypeComponent:
            ProvideCallTransaction,
        TransactionHashTypeComponent:
            ProvideFeltTxHash,
        TxResponseTypeComponent:
            ProvideStarknetTxResponse,
        SelectorTypeComponent:
            ProvideFeltSelector,
        [
            ContractClassTypeComponent,
            ContractClassHashTypeComponent,
        ]:
            ProvideStarknetContractTypes,
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
            CommitmentProofTypeComponent,
            CommitmentProofHeightGetterComponent,
            CommitmentProofBytesGetterComponent,
        ]:
            UseStarknetCommitmentProof,
        CommitmentPrefixTypeComponent:
            ProvideCommitmentPrefixBytes,
        PacketCommitmentTypeComponent:
            ProvideBytesPacketCommitment,
        AcknowledgementTypeComponent:
            ProvideBytesAcknowlegement,
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
        MessageSenderComponent:
            SendCallMessages,
        TxSubmitterComponent:
            SubmitCallTransaction,
        TxResponseQuerierComponent:
            QueryTransactionReceipt,
        TxResponsePollerComponent:
            PollTxResponse,
        PollTimeoutGetterComponent:
            DefaultPollTimeout,
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
            AckPacketPayloadTypeComponent,
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
        PacketAcknowledgementQuerierComponent:
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
        EventualAmountAsserterComponent:
            PollAssertEventualAmount,
        PollAssertDurationGetterComponent:
            ProvideDefaultPollAssertDuration,
        IbcTokenTransferMessageBuilderComponent:
            BuildStarknetIbcTransferMessage,
        PacketIsReceivedQuerierComponent:
            QueryPacketIsReceivedOnStarknet,
    }
}
