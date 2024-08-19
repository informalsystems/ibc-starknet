use core::num::TryFromIntError;
use std::convert::Infallible;
use std::string::FromUtf8Error;

use cairo_lang_starknet_classes::casm_contract_class::StarknetSierraCompilationError;
use cgp_core::prelude::*;
use eyre::Report;
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
use hermes_relayer_components::chain::traits::types::event::HasEventType;
use hermes_relayer_components::transaction::impls::poll_tx_response::TxNoResponseError;
use hermes_relayer_components::transaction::traits::types::tx_hash::HasTransactionHashType;
use hermes_runtime::types::error::TokioRuntimeError;
use hermes_starknet_chain_components::impls::error::account::RaiseAccountError;
use hermes_starknet_chain_components::impls::error::provider::RaiseProviderError;
use hermes_starknet_chain_components::impls::error::starknet::RaiseStarknetError;
use hermes_starknet_chain_components::impls::send_message::UnexpectedTransactionTraceType;
use hermes_starknet_chain_components::traits::event::{EventSelectorMissing, UnknownEvent};
use starknet::accounts::{single_owner, AccountError};
use starknet::core::types::contract::{ComputeClassHashError, JsonError};
use starknet::core::types::{RevertedInvocation, StarknetError};
use starknet::providers::ProviderError;
use starknet::signers::local_wallet;

pub struct HandleStarknetError;

pub type SignError = single_owner::SignError<local_wallet::SignError>;

delegate_components! {
    HandleStarknetError {
        Error: ReturnError,
        Infallible: HandleInfallible,
        [
            Report,
            TryFromIntError,
            FromUtf8Error,
            SignError,
            TokioRuntimeError,
            serde_json::error::Error,
            JsonError,
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
            <'a, Chain: HasTransactionHashType> TxNoResponseError<'a, Chain>,
            <'a, Chain: HasEventType> EventSelectorMissing<'a, Chain>,
            <'a, Chain: HasEventType> UnknownEvent<'a, Chain>,
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
