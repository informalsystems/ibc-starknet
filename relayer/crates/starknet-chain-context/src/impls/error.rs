use core::array::TryFromSliceError;
use core::convert::Infallible;
use core::num::{ParseIntError, TryFromIntError};
use core::str::Utf8Error;
use std::string::FromUtf8Error;

use cairo_lang_starknet_classes::casm_contract_class::StarknetSierraCompilationError;
use cgp::prelude::*;
use eyre::Report;
use hermes_cairo_encoding_components::impls::encode_mut::bool::DecodeBoolError;
use hermes_cairo_encoding_components::impls::encode_mut::end::NonEmptyBuffer;
use hermes_cairo_encoding_components::impls::encode_mut::felt::UnexpectedEndOfBuffer;
use hermes_cairo_encoding_components::impls::encode_mut::variant::VariantIndexOutOfBound;
use hermes_chain_components::impls::payload_builders::packet::InvalidTimeoutReceipt;
use hermes_chain_components::traits::types::packet::HasOutgoingPacketType;
use hermes_chain_type_components::traits::types::address::HasAddressType;
use hermes_chain_type_components::traits::types::height::HasHeightType;
use hermes_error::handlers::debug::DebugError;
use hermes_error::handlers::display::DisplayError;
use hermes_error::handlers::identity::ReturnError;
use hermes_error::handlers::infallible::HandleInfallible;
use hermes_error::handlers::report::ReportError;
use hermes_error::types::Error;
use hermes_protobuf_encoding_components::impls::any::TypeUrlMismatchError;
use hermes_protobuf_encoding_components::impls::encode_mut::chunk::{
    InvalidWireType, UnsupportedWireType,
};
use hermes_protobuf_encoding_components::impls::encode_mut::proto_field::decode_required::RequiredFieldTagNotFound;
use hermes_relayer_components::chain::impls::queries::consensus_state_height::NoConsensusStateAtLessThanHeight;
use hermes_relayer_components::chain::traits::send_message::EmptyMessageResponse;
use hermes_relayer_components::chain::traits::types::ibc::HasClientIdType;
use hermes_relayer_components::transaction::impls::poll_tx_response::TxNoResponseError;
use hermes_relayer_components::transaction::traits::types::tx_hash::HasTransactionHashType;
use hermes_runtime::types::error::TokioRuntimeError;
use hermes_starknet_chain_components::impls::error::account::RaiseAccountError;
use hermes_starknet_chain_components::impls::error::provider::RaiseProviderError;
use hermes_starknet_chain_components::impls::error::starknet::RaiseStarknetError;
use hermes_starknet_chain_components::impls::queries::consensus_state::ConsensusStateNotFound;
use hermes_starknet_chain_components::impls::queries::contract_address::ContractAddressNotFound;
use hermes_starknet_chain_components::impls::send_message::UnexpectedTransactionTraceType;
use hermes_starknet_chain_components::types::event::UnknownEvent;
use hermes_test_components::chain::impls::assert::poll_assert_eventual_amount::EventualAmountTimeoutError;
use hermes_test_components::chain::impls::ibc_transfer::MissingSendPacketEventError;
use hermes_test_components::chain::traits::types::amount::HasAmountType;
use ibc::core::channel::types::error::ChannelError;
use ibc::core::client::types::error::ClientError;
use ibc::core::host::types::error::{DecodingError, IdentifierError};
use ibc::primitives::TimestampError;
use prost::{DecodeError, EncodeError};
use starknet::accounts::{single_owner, AccountError};
use starknet::core::types::contract::{ComputeClassHashError, JsonError};
use starknet::core::types::{RevertedInvocation, StarknetError};
use starknet::providers::ProviderError;
use starknet::signers::local_wallet;

pub struct HandleStarknetChainError;

pub type SignError = single_owner::SignError<local_wallet::SignError>;

delegate_components! {
    HandleStarknetChainError {
        Error: ReturnError,
        Infallible: HandleInfallible,
        [
            Report,
            TryFromIntError,
            Utf8Error,
            ParseIntError,
            FromUtf8Error,
            SignError,
            TryFromSliceError,
            TokioRuntimeError,
            serde_json::error::Error,
            JsonError,
            EncodeError,
            DecodeError,
            DecodingError,
            ClientError,
            ChannelError,
            TimestampError,
            IdentifierError,
            ComputeClassHashError,
            StarknetSierraCompilationError,
        ]: ReportError,
        [
            <'a> &'a str,
            String,
        ]:
            DisplayError,
        [
            RevertedInvocation,
            UnexpectedTransactionTraceType,
            UnexpectedEndOfBuffer,
            NonEmptyBuffer,
            VariantIndexOutOfBound,
            DecodeBoolError,
            TypeUrlMismatchError,
            InvalidWireType,
            UnsupportedWireType,
            RequiredFieldTagNotFound,
            ContractAddressNotFound,
            EmptyMessageResponse,
            ConsensusStateNotFound,
            MissingSendPacketEventError,
            <'a> UnknownEvent<'a>,
            <'a, Chain: HasAddressType + HasAmountType> EventualAmountTimeoutError<'a, Chain>,
            <'a, Chain: HasTransactionHashType> TxNoResponseError<'a, Chain>,
            <'a, Chain: HasClientIdType<Counterparty>, Counterparty: HasHeightType>
                NoConsensusStateAtLessThanHeight<'a, Chain, Counterparty>,
            <'a, Chain: HasHeightType, Counterparty: HasOutgoingPacketType<Chain>>
                InvalidTimeoutReceipt<'a, Chain, Counterparty>,
        ]:
            DebugError,
        StarknetError: RaiseStarknetError,
        ProviderError: RaiseProviderError,
        AccountError<SignError>: RaiseAccountError,
    }
}
