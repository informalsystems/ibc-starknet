use cgp::prelude::*;
use hermes_encoding_components::traits::decode_mut::{
    CanDecodeMut, MutDecoder, MutDecoderComponent,
};
use hermes_encoding_components::traits::encode_mut::{
    CanEncodeMut, MutEncoder, MutEncoderComponent,
};
pub use ibc::core::channel::types::channel::{
    ChannelEnd, Counterparty as ChannelCounterparty, State as ChannelState,
};
use ibc::core::channel::types::error::ChannelError;
pub use ibc::core::host::types::identifiers::ChannelId;

use super::connection_id::ConnectionId;
use super::messages::ibc::channel::{AppVersion, ChannelOrdering, PortId};

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

impl Into<ChannelState> for RawChannelState {
    fn into(self) -> ChannelState {
        match self {
            Self::Uninitialized => ChannelState::Uninitialized,
            Self::Init => ChannelState::Init,
            Self::TryOpen => ChannelState::TryOpen,
            Self::Open => ChannelState::Open,
            Self::Closed => ChannelState::Closed,
        }
    }
}

#[derive(HasFields)]
pub struct RawChannelCounterparty {
    pub port_id: PortId,
    pub channel_id: Option<ChannelId>,
}

impl<'a> From<&'a ChannelCounterparty> for RawChannelCounterparty {
    fn from(value: &'a ChannelCounterparty) -> Self {
        Self {
            port_id: value.port_id.clone(),
            channel_id: value.channel_id.clone(),
        }
    }
}

impl Into<ChannelCounterparty> for RawChannelCounterparty {
    fn into(self) -> ChannelCounterparty {
        ChannelCounterparty {
            port_id: self.port_id,
            channel_id: self.channel_id,
        }
    }
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
        if value.connection_hops.len() != 1 {
            return Err(Encoding::raise_error("invalid connection hops"));
        }

        let raw = RawChannelEnd {
            state: (&value.state).into(),
            ordering: value.ordering,
            remote: value.counterparty().into(),
            connection_id: value.connection_hops[0].clone(),
            version: value.version.clone(),
        };

        encoding.encode_mut(&raw, buffer)
    }
}

#[cgp_provider(MutDecoderComponent)]
impl<Encoding, Strategy> MutDecoder<Encoding, Strategy, ChannelEnd> for EncodeChannelEnd
where
    Encoding: CanDecodeMut<Strategy, RawChannelEnd> + CanRaiseAsyncError<ChannelError>,
{
    fn decode_mut<'a>(
        encoding: &Encoding,
        buffer: &mut Encoding::DecodeBuffer<'a>,
    ) -> Result<ChannelEnd, Encoding::Error> {
        let raw = encoding.decode_mut(buffer)?;

        ChannelEnd::new(
            raw.state.into(),
            raw.ordering,
            raw.remote.into(),
            vec![raw.connection_id],
            raw.version,
        )
        .map_err(Encoding::raise_error)
    }
}
