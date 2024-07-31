use std::convert::Infallible;

use cgp_core::prelude::*;
use eyre::Report;
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
use starknet::accounts::{single_owner, AccountError};
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
            AccountError<SignError>,
        ]: ReportError,
        [
            <'a> &'a str,
            String,
        ]:
            DisplayError,
        [
            <'a, Chain: HasTransactionHashType> TxNoResponseError<'a, Chain>
        ]:
            DebugError,
        [
            WrapError<&'static str, Error>,
            WrapError<String, Error>,
        ]:
            WrapErrorDetail,
    }
}
