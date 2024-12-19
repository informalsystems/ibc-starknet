use core::convert::Infallible;
use core::num::ParseIntError;

use cgp::core::component::UseDelegate;
use cgp::core::error::{ErrorRaiser, ErrorRaiserComponent, ErrorTypeComponent};
use cgp::prelude::*;
use eyre::Report;
use hermes_error::handlers::display::DisplayError;
use hermes_error::handlers::identity::ReturnError;
use hermes_error::handlers::infallible::HandleInfallible;
use hermes_error::handlers::report::ReportError;
use hermes_error::handlers::wrap::WrapErrorDetail;
use hermes_error::impls::ProvideHermesError;
use hermes_error::traits::wrap::WrapError;
use hermes_error::types::Error;
use hermes_relayer_components::error::traits::retry::RetryableErrorComponent;
use hermes_runtime::types::error::TokioRuntimeError;
use ibc::core::host::types::error::DecodingError;
use starknet_types_core::felt::FromStrError;

pub struct ProvideCliError;

pub struct CliErrorHandlers;

pub trait CanHandleCliError<Context>: ErrorRaiser<Context, TokioRuntimeError>
where
    Context: HasErrorType<Error = Error>,
{
}

impl<Context> CanHandleCliError<Context> for ProvideCliError where
    Context: HasErrorType<Error = Error>
{
}

delegate_components! {
    ProvideCliError {
        [
            ErrorTypeComponent,
            RetryableErrorComponent,
        ]: ProvideHermesError,
        ErrorRaiserComponent: UseDelegate<CliErrorHandlers>,
    }
}

delegate_components! {
    CliErrorHandlers {
        Error: ReturnError,
        Infallible: HandleInfallible,
        [
            Report,
            FromStrError,
            ParseIntError,
            DecodingError,
            TokioRuntimeError,
            toml::de::Error,
            toml::ser::Error,
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
