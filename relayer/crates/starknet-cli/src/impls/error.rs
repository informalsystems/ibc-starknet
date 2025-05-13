use core::convert::Infallible;
use core::num::ParseIntError;

use cgp::core::component::UseDelegate;
use cgp::core::error::{
    ErrorRaiser, ErrorRaiserComponent, ErrorTypeProviderComponent, ErrorWrapperComponent,
};
use eyre::Report;
use hermes_core::relayer_components::error::traits::RetryableErrorComponent;
use hermes_cosmos::error::handlers::{DisplayError, HandleInfallible, ReportError, ReturnError};
use hermes_cosmos::error::impls::UseHermesError;
use hermes_cosmos::error::types::Error;
use hermes_cosmos::runtime::types::error::TokioRuntimeError;
use hermes_prelude::*;
use ibc::core::channel::types::error::ChannelError;
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
            ChannelError,
            TokioRuntimeError,
            std::io::Error,
            toml::de::Error,
            toml::ser::Error,
            serde_json::Error,
        ]: ReportError,
        [
            <'a> &'a str,
            String,
        ]:
            DisplayError,
    }
}
