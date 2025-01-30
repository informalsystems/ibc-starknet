use cgp::core::component::UseContext;
use cgp::prelude::*;
use hermes_cairo_encoding_components::impls::encode_mut::variant_from::EncodeVariantFrom;
use hermes_encoding_components::impls::encode_mut::combine::CombineEncoders;
use hermes_encoding_components::impls::encode_mut::field::EncodeField;
use hermes_encoding_components::impls::encode_mut::from::DecodeFrom;
use hermes_encoding_components::traits::decode_mut::{CanDecodeMut, MutDecoder};
use hermes_encoding_components::traits::encode_mut::{CanEncodeMut, MutEncoder};
use hermes_encoding_components::traits::transform::{Transformer, TransformerRef};
use hermes_wasm_encoding_components::components::{MutDecoderComponent, MutEncoderComponent};
pub use ibc::core::host::types::identifiers::ConnectionId;

use crate::types::client_id::ClientId;
use crate::types::messages::ibc::connection::{BasePrefix, ConnectionVersion};

pub struct EncodeConnectionId;

impl<Encoding, Strategy> MutEncoder<Encoding, Strategy, ConnectionId> for EncodeConnectionId
where
    Encoding: CanEncodeMut<Strategy, Product![String]>,
{
    fn encode_mut(
        encoding: &Encoding,
        value: &ConnectionId,
        buffer: &mut Encoding::EncodeBuffer,
    ) -> Result<(), Encoding::Error> {
        encoding.encode_mut(&product![value.to_string()], buffer)?;
        Ok(())
    }
}

impl<Encoding, Strategy> MutDecoder<Encoding, Strategy, ConnectionId> for EncodeConnectionId
where
    Encoding: CanDecodeMut<Strategy, Product![String]> + CanRaiseAsyncError<&'static str>,
{
    fn decode_mut<'a>(
        encoding: &Encoding,
        buffer: &mut Encoding::DecodeBuffer<'a>,
    ) -> Result<ConnectionId, Encoding::Error> {
        let product![value_str] = encoding.decode_mut(buffer)?;
        value_str
            .parse()
            .map_err(|_| Encoding::raise_error("invalid connection id"))
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
