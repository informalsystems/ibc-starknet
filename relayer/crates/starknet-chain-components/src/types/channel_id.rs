use cgp::core::component::UseContext;
use cgp::prelude::*;
use hermes_cairo_encoding_components::impls::encode_mut::variant_from::EncodeVariantFrom;
use hermes_encoding_components::impls::encode_mut::combine::CombineEncoders;
use hermes_encoding_components::impls::encode_mut::field::EncodeField;
use hermes_encoding_components::impls::encode_mut::from::DecodeFrom;
use hermes_encoding_components::traits::transform::{Transformer, TransformerRef};
use hermes_wasm_encoding_components::components::{MutDecoderComponent, MutEncoderComponent};

use super::connection_id::ConnectionId;
use super::messages::ibc::channel::{AppVersion, ChannelOrdering, PortId};

#[derive(Debug, PartialEq, Clone, HasField, Eq, Ord, PartialOrd)]
pub struct ChannelId {
    pub channel_id: String,
}

pub struct EncodeChannelId;

delegate_components! {
    EncodeChannelId {
        MutEncoderComponent: CombineEncoders<Product![
            EncodeField<symbol!("channel_id"), UseContext>,
        ]>,
        MutDecoderComponent: DecodeFrom<Self, UseContext>,
    }
}

impl Transformer for EncodeChannelId {
    type From = String;
    type To = ChannelId;

    fn transform(channel_id: Self::From) -> ChannelId {
        ChannelId { channel_id }
    }
}
#[derive(Debug, PartialEq, Clone)]
pub enum ChannelState {
    Uninitialized,
    Init,
    TryOpen,
    Open,
    Closed,
}

pub struct EncodeChannelState;

delegate_components! {
    EncodeChannelState {
        [
            MutEncoderComponent,
            MutDecoderComponent,
        ]: EncodeVariantFrom<Self>,
    }
}

impl TransformerRef for EncodeChannelState {
    type From = ChannelState;
    type To<'a> = Sum![(), (), (), (), ()];

    fn transform<'a>(from: &'a ChannelState) -> Sum![(), (), (), (), ()] {
        match from {
            ChannelState::Uninitialized => Either::Left(()),
            ChannelState::Init => Either::Right(Either::Left(())),
            ChannelState::TryOpen => Either::Right(Either::Right(Either::Left(()))),
            ChannelState::Open => Either::Right(Either::Right(Either::Right(Either::Left(())))),
            ChannelState::Closed => Either::Right(Either::Right(Either::Right(Either::Right(
                Either::Left(()),
            )))),
        }
    }
}

impl Transformer for EncodeChannelState {
    type From = Sum![(), (), (), (), ()];
    type To = ChannelState;

    fn transform(value: Sum![(), (), (), (), ()]) -> ChannelState {
        match value {
            Either::Left(()) => ChannelState::Uninitialized,
            Either::Right(Either::Left(())) => ChannelState::Init,
            Either::Right(Either::Right(Either::Left(()))) => ChannelState::TryOpen,
            Either::Right(Either::Right(Either::Right(Either::Left(())))) => ChannelState::Open,
            Either::Right(Either::Right(Either::Right(Either::Right(Either::Left(()))))) => {
                ChannelState::Closed
            }
            Either::Right(Either::Right(Either::Right(Either::Right(Either::Right(value))))) => {
                match value {}
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone, HasField)]
pub struct ChannelCounterparty {
    pub port_id: PortId,
    pub channel_id: ChannelId,
}

pub struct EncodeChannelCounterparty;

delegate_components! {
    EncodeChannelCounterparty {
        MutEncoderComponent: CombineEncoders<Product![
            EncodeField<symbol!("port_id"), UseContext>,
            EncodeField<symbol!("channel_id"), UseContext>,
        ]>,
        MutDecoderComponent: DecodeFrom<Self, UseContext>,
    }
}

impl Transformer for EncodeChannelCounterparty {
    type From = Product![PortId, ChannelId];
    type To = ChannelCounterparty;

    fn transform(product![port_id, channel_id]: Self::From) -> ChannelCounterparty {
        ChannelCounterparty {
            port_id,
            channel_id,
        }
    }
}

#[derive(Debug, PartialEq, Clone, HasField)]
pub struct ChannelEnd {
    pub state: ChannelState,
    pub ordering: ChannelOrdering,
    pub remote: ChannelCounterparty,
    pub connection_id: ConnectionId,
    pub version: AppVersion,
}

pub struct EncodeChannelEnd;

delegate_components! {
    EncodeChannelEnd {
        MutEncoderComponent: CombineEncoders<Product![
            EncodeField<symbol!("state"), UseContext>,
            EncodeField<symbol!("ordering"), UseContext>,
            EncodeField<symbol!("remote"), UseContext>,
            EncodeField<symbol!("connection_id"), UseContext>,
            EncodeField<symbol!("version"), UseContext>,
        ]>,
        MutDecoderComponent: DecodeFrom<Self, UseContext>,
    }
}

impl Transformer for EncodeChannelEnd {
    type From = Product![
        ChannelState,
        ChannelOrdering,
        ChannelCounterparty,
        ConnectionId,
        AppVersion,
    ];
    type To = ChannelEnd;

    fn transform(
        product![state, ordering, remote, connection_id, version]: Self::From,
    ) -> ChannelEnd {
        ChannelEnd {
            state,
            ordering,
            remote,
            connection_id,
            version,
        }
    }
}
