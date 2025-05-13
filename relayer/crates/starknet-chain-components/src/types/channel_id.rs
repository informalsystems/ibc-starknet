use hermes_core::encoding_components::traits::{
    CanDecodeMut, CanEncodeMut, MutDecoder, MutDecoderComponent, MutEncoder, MutEncoderComponent,
};
use hermes_prelude::*;
use ibc::core::channel::types::channel::{
    ChannelEnd, Counterparty as ChannelCounterparty, State as ChannelState,
};
use ibc::core::channel::types::error::ChannelError;
use ibc::core::host::types::error::IdentifierError;
use ibc::core::host::types::identifiers::PortId;

use crate::types::{AppVersion, ChannelOrdering, ConnectionId};

#[derive(HasFields)]
pub struct RawChannelEnd {
    pub state: RawChannelState,
    pub ordering: ChannelOrdering,
    pub remote: RawChannelCounterparty,
    pub connection_id: ConnectionId,
    pub version: AppVersion,
}

#[derive(HasFields)]
pub enum RawChannelState {
    Uninitialized,
    Init,
    TryOpen,
    Open,
    Closed,
}

impl<'a> From<&'a ChannelState> for RawChannelState {
    fn from(value: &'a ChannelState) -> Self {
        match value {
            ChannelState::Uninitialized => Self::Uninitialized,
            ChannelState::Init => Self::Init,
            ChannelState::TryOpen => Self::TryOpen,
            ChannelState::Open => Self::Open,
            ChannelState::Closed => Self::Closed,
        }
    }
}

impl From<RawChannelState> for ChannelState {
    fn from(val: RawChannelState) -> Self {
        match val {
            RawChannelState::Uninitialized => Self::Uninitialized,
            RawChannelState::Init => Self::Init,
            RawChannelState::TryOpen => Self::TryOpen,
            RawChannelState::Open => Self::Open,
            RawChannelState::Closed => Self::Closed,
        }
    }
}

#[derive(HasFields)]
pub struct RawChannelCounterparty {
    pub port_id: PortId,
    pub channel_id: String,
}

pub struct EncodeChannelEnd;

#[cgp_provider(MutEncoderComponent)]
impl<Encoding, Strategy> MutEncoder<Encoding, Strategy, ChannelEnd> for EncodeChannelEnd
where
    Encoding: CanEncodeMut<Strategy, RawChannelEnd> + CanRaiseAsyncError<&'static str>,
{
    fn encode_mut(
        encoding: &Encoding,
        value: &ChannelEnd,
        buffer: &mut Encoding::EncodeBuffer,
    ) -> Result<(), Encoding::Error> {
        let counterparty = value.counterparty();

        let channel_id = counterparty
            .channel_id
            .as_ref()
            .map(ToString::to_string)
            .unwrap_or_default();

        let [connection_id]: [ConnectionId; 1] = value
            .connection_hops
            .clone()
            .try_into()
            .map_err(|_| Encoding::raise_error("expect exactly one connection id to exist"))?;

        let raw = RawChannelEnd {
            state: (&value.state).into(),
            ordering: value.ordering,
            remote: RawChannelCounterparty {
                port_id: counterparty.port_id.clone(),
                channel_id,
            },
            connection_id,
            version: value.version.clone(),
        };

        encoding.encode_mut(&raw, buffer)
    }
}

#[cgp_provider(MutDecoderComponent)]
impl<Encoding, Strategy> MutDecoder<Encoding, Strategy, ChannelEnd> for EncodeChannelEnd
where
    Encoding: CanDecodeMut<Strategy, RawChannelEnd>
        + CanRaiseAsyncError<ChannelError>
        + CanRaiseAsyncError<IdentifierError>,
{
    fn decode_mut<'a>(
        encoding: &Encoding,
        buffer: &mut Encoding::DecodeBuffer<'a>,
    ) -> Result<ChannelEnd, Encoding::Error> {
        let raw = encoding.decode_mut(buffer)?;

        let raw_channel_id = raw.remote.channel_id;
        let channel_id = if raw_channel_id.is_empty() {
            None
        } else {
            Some(raw_channel_id.parse().map_err(Encoding::raise_error)?)
        };

        ChannelEnd::new(
            raw.state.into(),
            raw.ordering,
            ChannelCounterparty {
                port_id: raw.remote.port_id,
                channel_id,
            },
            vec![raw.connection_id],
            raw.version,
        )
        .map_err(Encoding::raise_error)
    }
}
