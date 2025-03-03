use core::convert::Infallible;
use core::num::ParseIntError;

use cgp::core::component::UseDelegate;
use cgp::core::error::{
    ErrorRaiser, ErrorRaiserComponent, ErrorTypeProviderComponent, ErrorWrapperComponent,
};
use cgp::prelude::*;
use eyre::Report;
use hermes_error::handlers::display::DisplayError;
use hermes_error::handlers::identity::ReturnError;
use hermes_error::handlers::infallible::HandleInfallible;
use hermes_error::handlers::report::ReportError;
use hermes_error::impls::UseHermesError;
use hermes_error::types::Error;
use hermes_relayer_components::error::traits::RetryableErrorComponent;
use hermes_runtime::types::error::TokioRuntimeError;
use ibc::core::host::types::error::{DecodingError, IdentifierError};
use starknet_types_core::felt::FromStrError;

pub struct ProvideCliError;

pub struct CliErrorHandlers;

pub trait CanHandleCliError<Context>: ErrorRaiser<Context, TokioRuntimeError>
where
    Context: HasAsyncErrorType<Error = Error>,
{
}

impl<Context> CanHandleCliError<Context> for ProvideCliError where
    Context: HasAsyncErrorType<Error = Error>
{
}

delegate_components! {
    ProvideCliError {
        [
            ErrorTypeProviderComponent,
            ErrorWrapperComponent,
            RetryableErrorComponent,
        ]: UseHermesError,
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
            IdentifierError,
            TokioRuntimeError,
            toml::de::Error,
            toml::ser::Error,
        ]: ReportError,
        [
            <'a> &'a str,
            String,
        ]:
            DisplayError,
    }
}
