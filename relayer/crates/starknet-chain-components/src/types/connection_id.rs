use core::time::Duration;

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
pub use ibc::core::connection::types::{
    Counterparty as ConnectionCounterparty, State as ConnectionState,
};
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

pub struct EncodeDuration;

impl<Encoding, Strategy> MutEncoder<Encoding, Strategy, Duration> for EncodeDuration
where
    Encoding: CanEncodeMut<Strategy, Product![u64]>,
{
    fn encode_mut(
        encoding: &Encoding,
        value: &Duration,
        buffer: &mut Encoding::EncodeBuffer,
    ) -> Result<(), Encoding::Error> {
        encoding.encode_mut(&product![value.as_secs()], buffer)?;
        Ok(())
    }
}

impl<Encoding, Strategy> MutDecoder<Encoding, Strategy, Duration> for EncodeDuration
where
    Encoding: CanDecodeMut<Strategy, Product![u64]>,
{
    fn decode_mut<'a>(
        encoding: &Encoding,
        buffer: &mut Encoding::DecodeBuffer<'a>,
    ) -> Result<Duration, Encoding::Error> {
        let product![secs] = encoding.decode_mut(buffer)?;
        Ok(Duration::from_secs(secs))
    }
}

#[derive(HasField)]
pub struct ConnectionEnd {
    pub state: ConnectionState,
    pub client_id: ClientId,
    pub counterparty: ConnectionCounterparty,
    pub version: ConnectionVersion,
    pub delay_period: Duration,
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
        Duration
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

pub struct EncodeConnectionCounterparty;

impl<Encoding, Strategy> MutEncoder<Encoding, Strategy, ConnectionCounterparty>
    for EncodeConnectionCounterparty
where
    Encoding: CanEncodeMut<Strategy, Product![ClientId, Product![String], BasePrefix]>,
{
    fn encode_mut(
        encoding: &Encoding,
        value: &ConnectionCounterparty,
        buffer: &mut Encoding::EncodeBuffer,
    ) -> Result<(), Encoding::Error> {
        match &value.connection_id {
            Some(connection_id) => encoding.encode_mut(
                &product![
                    value.client_id.clone(),
                    product![connection_id.to_string()],
                    value.prefix.clone()
                ],
                buffer,
            )?,
            None => encoding.encode_mut(
                &product![
                    value.client_id.clone(),
                    product![String::new()],
                    value.prefix.clone()
                ],
                buffer,
            )?,
        }

        Ok(())
    }
}

impl<Encoding, Strategy> MutDecoder<Encoding, Strategy, ConnectionCounterparty>
    for EncodeConnectionCounterparty
where
    Encoding: CanDecodeMut<Strategy, Product![ClientId, Product![String], BasePrefix]>
        + CanRaiseAsyncError<&'static str>,
{
    fn decode_mut<'a>(
        encoding: &Encoding,
        buffer: &mut Encoding::DecodeBuffer<'a>,
    ) -> Result<ConnectionCounterparty, Encoding::Error> {
        let product![client_id, product![connection_id_str], base_prefix] =
            encoding.decode_mut(buffer)?;

        if connection_id_str.is_empty() {
            Ok(ConnectionCounterparty::new(client_id, None, base_prefix))
        } else {
            Ok(ConnectionCounterparty::new(
                client_id,
                Some(
                    connection_id_str
                        .parse()
                        .map_err(|_| Encoding::raise_error("invalid connection id"))?,
                ),
                base_prefix,
            ))
        }
    }
}
