use std::convert::Infallible;

use cgp_core::prelude::*;
use eyre::Report;
use hermes_error::handlers::display::DisplayError;
use hermes_error::handlers::identity::ReturnError;
use hermes_error::handlers::infallible::HandleInfallible;
use hermes_error::handlers::report::ReportError;
use hermes_error::handlers::wrap::WrapErrorDetail;
use hermes_error::traits::wrap::WrapError;
use hermes_error::types::Error;
use starknet::providers::ProviderError;

pub struct HandleStarknetError;

delegate_components! {
    HandleStarknetError {
        Error: ReturnError,
        Infallible: HandleInfallible,
        [
            Report,
            ProviderError,
        ]: ReportError,
        [
            <'a> &'a str,
            String,
        ]:
            DisplayError,
        [
            WrapError<&'static str, Error>,
            WrapError<String, Error>,
        ]:
            WrapErrorDetail,
    }
}
