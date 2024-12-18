use core::num::TryFromIntError;
use std::convert::Infallible;
use std::string::FromUtf8Error;

use cgp::prelude::*;
use eyre::Report;
use hermes_error::handlers::debug::DebugError;
use hermes_error::handlers::display::DisplayError;
use hermes_error::handlers::identity::ReturnError;
use hermes_error::handlers::infallible::HandleInfallible;
use hermes_error::handlers::report::ReportError;
use hermes_error::handlers::wrap::WrapErrorDetail;
use hermes_error::traits::wrap::WrapError;
use hermes_error::types::Error;
use hermes_relayer_components::chain::traits::send_message::EmptyMessageResponse;
use hermes_relayer_components::chain::traits::types::chain_id::HasChainIdType;
use hermes_relayer_components::relay::impls::channel::open_init::MissingChannelInitEventError;
use hermes_relayer_components::relay::impls::channel::open_try::MissingChannelTryEventError;
use hermes_relayer_components::relay::impls::connection::open_init::MissingConnectionInitEventError;
use hermes_relayer_components::relay::impls::connection::open_try::MissingConnectionTryEventError;
use hermes_relayer_components::relay::impls::create_client::MissingCreateClientEventError;
use hermes_relayer_components::relay::traits::chains::HasRelayChains;
use hermes_runtime::types::error::TokioRuntimeError;

pub struct HandleStarknetRelayError;

delegate_components! {
    HandleStarknetRelayError {
        Error: ReturnError,
        Infallible: HandleInfallible,
        [
            Report,
            TryFromIntError,
            FromUtf8Error,
            TokioRuntimeError,
        ]: ReportError,
        [
            <'a> &'a str,
            String,
        ]:
            DisplayError,
        [
            EmptyMessageResponse,
            <'a, Chain: HasChainIdType, Counterparty: HasChainIdType>
                MissingCreateClientEventError<'a, Chain, Counterparty>,
            <'a, Relay>
                MissingConnectionInitEventError<'a, Relay>,
            <'a, Relay: HasRelayChains>
                MissingConnectionTryEventError<'a, Relay>,
            <'a, Relay>
                MissingChannelInitEventError<'a, Relay>,
            <'a, Relay: HasRelayChains>
                MissingChannelTryEventError<'a, Relay>,
        ]:
            DebugError,
        [
            WrapError<&'static str, Error>,
            WrapError<String, Error>,
        ]:
            WrapErrorDetail,
    }
}
