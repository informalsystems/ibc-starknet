use core::convert::Infallible;
use core::num::TryFromIntError;
use std::string::FromUtf8Error;

use cgp::prelude::*;
use eyre::Report;
use hermes_core::chain_components::traits::{EmptyMessageResponse, HasChainIdType};
use hermes_core::relayer_components::relay::impls::{
    MissingChannelInitEventError, MissingChannelTryEventError, MissingConnectionInitEventError,
    MissingConnectionTryEventError, MissingCreateClientEventError,
};
use hermes_core::relayer_components::relay::traits::HasRelayChains;
use hermes_cosmos::error::handlers::{
    DebugError, DisplayError, HandleInfallible, ReportError, ReturnError,
};
use hermes_cosmos::error::types::Error;
use hermes_cosmos::runtime::types::error::TokioRuntimeError;

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
    }
}
