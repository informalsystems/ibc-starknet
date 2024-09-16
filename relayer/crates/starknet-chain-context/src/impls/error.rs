use core::array::TryFromSliceError;
use core::num::TryFromIntError;
use core::str::Utf8Error;
use std::convert::Infallible;
use std::string::FromUtf8Error;

use cairo_lang_starknet_classes::casm_contract_class::StarknetSierraCompilationError;
use cgp::prelude::*;
use eyre::Report;
use hermes_cairo_encoding_components::impls::encode_mut::bool::DecodeBoolError;
use hermes_cairo_encoding_components::impls::encode_mut::end::NonEmptyBuffer;
use hermes_cairo_encoding_components::impls::encode_mut::felt::UnexpectedEndOfBuffer;
use hermes_cairo_encoding_components::impls::encode_mut::variant::VariantIndexOutOfBound;
use hermes_error::handlers::debug::DebugError;
use hermes_error::handlers::display::DisplayError;
use hermes_error::handlers::identity::ReturnError;
use hermes_error::handlers::infallible::HandleInfallible;
use hermes_error::handlers::report::ReportError;
use hermes_error::handlers::wrap::WrapErrorDetail;
use hermes_error::traits::wrap::WrapError;
use hermes_error::types::Error;
use hermes_protobuf_encoding_components::impls::any::TypeUrlMismatchError;
use hermes_protobuf_encoding_components::impls::encode_mut::chunk::{
    InvalidWireType, UnsupportedWireType,
};
use hermes_protobuf_encoding_components::impls::encode_mut::proto_field::decode_required::RequiredFieldTagNotFound;
use hermes_relayer_components::transaction::impls::poll_tx_response::TxNoResponseError;
use hermes_relayer_components::transaction::traits::types::tx_hash::HasTransactionHashType;
use hermes_runtime::types::error::TokioRuntimeError;
use hermes_starknet_chain_components::impls::error::account::RaiseAccountError;
use hermes_starknet_chain_components::impls::error::provider::RaiseProviderError;
use hermes_starknet_chain_components::impls::error::starknet::RaiseStarknetError;
use hermes_starknet_chain_components::impls::send_message::UnexpectedTransactionTraceType;
use hermes_starknet_chain_components::types::event::UnknownEvent;
use ibc::core::client::types::error::ClientError;
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
            FromUtf8Error,
            SignError,
            TryFromSliceError,
            TokioRuntimeError,
            serde_json::error::Error,
            JsonError,
            EncodeError,
            DecodeError,
            ClientError,
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
            <'a> UnknownEvent<'a>,
            <'a, Chain: HasTransactionHashType> TxNoResponseError<'a, Chain>,
        ]:
            DebugError,
        [
            WrapError<&'static str, Error>,
            WrapError<String, Error>,
        ]:
            WrapErrorDetail,
        StarknetError: RaiseStarknetError,
        ProviderError: RaiseProviderError,
        AccountError<SignError>: RaiseAccountError,
    }
}
