use core::fmt::{Display, Formatter};

use cgp::core::component::UseContext;
use cgp::prelude::*;
use hermes_cairo_encoding_components::impls::encode_mut::variant_from::EncodeVariantFrom;
use hermes_encoding_components::impls::encode_mut::combine::CombineEncoders;
use hermes_encoding_components::impls::encode_mut::field::EncodeField;
use hermes_encoding_components::impls::encode_mut::from::DecodeFrom;
use hermes_encoding_components::traits::transform::{Transformer, TransformerRef};
use hermes_wasm_encoding_components::components::{MutDecoderComponent, MutEncoderComponent};

use crate::types::client_id::ClientId;
use crate::types::messages::ibc::connection::{BasePrefix, ConnectionVersion};

#[derive(Debug, PartialEq, Clone, HasField)]
pub struct ConnectionId {
    pub connection_id: String,
}

pub struct EncodeConnectionId;

delegate_components! {
    EncodeConnectionId {
        MutEncoderComponent: CombineEncoders<Product![
            EncodeField<symbol!("connection_id"), UseContext>,
        ]>,
        MutDecoderComponent: DecodeFrom<Self, UseContext>,
    }
}

impl Transformer for EncodeConnectionId {
    type From = String;
    type To = ConnectionId;

    fn transform(connection_id: Self::From) -> ConnectionId {
        ConnectionId { connection_id }
    }
}

impl Display for ConnectionId {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        self.connection_id.fmt(f)
    }
}

#[derive(HasField)]
pub struct ConnectionEnd {
    pub state: ConnectionState,
    pub client_id: ClientId,
    pub counterparty: ConnectionCounterparty,
    pub version: ConnectionVersion,
    pub delay_period: u64,
}

pub struct EncodeConnectionEnd;

delegate_components! {
    EncodeConnectionEnd {
        MutEncoderComponent: CombineEncoders<Product![
            EncodeField<symbol!("state"), UseContext>,
            EncodeField<symbol!("client_id"), UseContext>,
            EncodeField<symbol!("counterparty"), UseContext>,
            EncodeField<symbol!("version"), UseContext>,
            EncodeField<symbol!("delay_period"), UseContext>,
        ]>,
        MutDecoderComponent: DecodeFrom<Self, UseContext>,
    }
}

impl Transformer for EncodeConnectionEnd {
    type From = Product![
        ConnectionState,
        ClientId,
        ConnectionCounterparty,
        ConnectionVersion,
        u64
    ];
    type To = ConnectionEnd;

    fn transform(
        product![state, client_id, counterparty, version, delay_period,]: Self::From,
    ) -> ConnectionEnd {
        ConnectionEnd {
            state,
            client_id,
            counterparty,
            version,
            delay_period,
        }
    }
}

pub enum ConnectionState {
    Uninitialized,
    Init,
    TryOpen,
    Open,
}

pub struct EncodeConnectionState;

delegate_components! {
    EncodeConnectionState {
        [
            MutEncoderComponent,
            MutDecoderComponent,
        ]: EncodeVariantFrom<Self>,
    }
}

impl TransformerRef for EncodeConnectionState {
    type From = ConnectionState;
    type To<'a> = Sum![(), (), (), ()];

    fn transform<'a>(from: &'a ConnectionState) -> Sum![(), (), (), ()] {
        match from {
            ConnectionState::Uninitialized => Either::Left(()),
            ConnectionState::Init => Either::Right(Either::Left(())),
            ConnectionState::TryOpen => Either::Right(Either::Right(Either::Left(()))),
            ConnectionState::Open => Either::Right(Either::Right(Either::Right(Either::Left(())))),
        }
    }
}

impl Transformer for EncodeConnectionState {
    type From = Sum![(), (), (), ()];
    type To = ConnectionState;

    fn transform(value: Sum![(), (), (), ()]) -> ConnectionState {
        match value {
            Either::Left(()) => ConnectionState::Uninitialized,
            Either::Right(Either::Left(())) => ConnectionState::Init,
            Either::Right(Either::Right(Either::Left(()))) => ConnectionState::TryOpen,
            Either::Right(Either::Right(Either::Right(Either::Left(())))) => ConnectionState::Open,
            Either::Right(Either::Right(Either::Right(Either::Right(value)))) => match value {},
        }
    }
}

#[derive(HasField)]
pub struct ConnectionCounterparty {
    pub client_id: ClientId,
    pub connection_id: ConnectionId,
    pub prefix: BasePrefix,
}

pub struct EncodeConnectionCounterparty;

delegate_components! {
    EncodeConnectionCounterparty {
        MutEncoderComponent: CombineEncoders<Product![
            EncodeField<symbol!("client_id"), UseContext>,
            EncodeField<symbol!("connection_id"), UseContext>,
            EncodeField<symbol!("prefix"), UseContext>,
        ]>,
        MutDecoderComponent: DecodeFrom<Self, UseContext>,
    }
}

impl Transformer for EncodeConnectionCounterparty {
    type From = Product![ClientId, ConnectionId, BasePrefix];
    type To = ConnectionCounterparty;

    fn transform(
        product![client_id, connection_id, prefix,]: Self::From,
    ) -> ConnectionCounterparty {
        ConnectionCounterparty {
            client_id,
            connection_id,
            prefix,
        }
    }
}
