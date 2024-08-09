use std::convert::Infallible;

use cairo_lang_starknet_classes::casm_contract_class::StarknetSierraCompilationError;
use cgp_core::prelude::*;
use eyre::Report;
use hermes_cairo_encoding_components::impls::encode_mut::felt::UnexpectedEndOfBuffer;
use hermes_error::handlers::debug::DebugError;
use hermes_error::handlers::display::DisplayError;
use hermes_error::handlers::identity::ReturnError;
use hermes_error::handlers::infallible::HandleInfallible;
use hermes_error::handlers::report::ReportError;
use hermes_error::handlers::wrap::WrapErrorDetail;
use hermes_error::traits::wrap::WrapError;
use hermes_error::types::Error;
use hermes_relayer_components::transaction::impls::poll_tx_response::TxNoResponseError;
use hermes_relayer_components::transaction::traits::types::tx_hash::HasTransactionHashType;
use hermes_runtime::types::error::TokioRuntimeError;
use hermes_starknet_chain_components::impls::send_message::UnexpectedTransactionTraceType;
use starknet::accounts::{single_owner, AccountError};
use starknet::core::types::contract::{ComputeClassHashError, JsonError};
use starknet::core::types::RevertedInvocation;
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
            ProviderError,
            SignError,
            TokioRuntimeError,
            AccountError<SignError>,
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
            <'a, Chain: HasTransactionHashType> TxNoResponseError<'a, Chain>,
        ]:
            DebugError,
        [
            WrapError<&'static str, Error>,
            WrapError<String, Error>,
        ]:
            WrapErrorDetail,
    }
}
