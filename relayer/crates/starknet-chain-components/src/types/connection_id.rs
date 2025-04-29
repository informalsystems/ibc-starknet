use core::time::Duration;

use hermes_cairo_encoding_components::impls::encode_mut::variant_from::EncodeVariantFrom;
use hermes_core::encoding_components::traits::{
    CanDecodeMut, CanEncodeMut, MutDecoder, MutDecoderComponent, MutEncoder, MutEncoderComponent,
    Transformer, TransformerRef,
};
use hermes_prelude::*;
use ibc::core::connection::types::{
    Counterparty as ConnectionCounterparty, State as ConnectionState,
};

use crate::types::{BasePrefix, ClientId, ConnectionEnd, ConnectionVersion};

pub struct EncodeDuration;

#[cgp_provider(MutEncoderComponent)]
impl<Encoding, Strategy> MutEncoder<Encoding, Strategy, Duration> for EncodeDuration
where
    Encoding: CanEncodeMut<Strategy, Product![u64, u32]>,
{
    fn encode_mut(
        encoding: &Encoding,
        value: &Duration,
        buffer: &mut Encoding::EncodeBuffer,
    ) -> Result<(), Encoding::Error> {
        encoding.encode_mut(&product![value.as_secs(), value.subsec_nanos()], buffer)?;
        Ok(())
    }
}

#[cgp_provider(MutDecoderComponent)]
impl<Encoding, Strategy> MutDecoder<Encoding, Strategy, Duration> for EncodeDuration
where
    Encoding: CanDecodeMut<Strategy, Product![u64, u32]>,
{
    fn decode_mut<'a>(
        encoding: &Encoding,
        buffer: &mut Encoding::DecodeBuffer<'a>,
    ) -> Result<Duration, Encoding::Error> {
        let product![secs, nanos] = encoding.decode_mut(buffer)?;
        Ok(Duration::new(secs, nanos))
    }
}

pub struct EncodeConnectionEnd;

#[cgp_provider(MutEncoderComponent)]
impl<Encoding, Strategy> MutEncoder<Encoding, Strategy, ConnectionEnd> for EncodeConnectionEnd
where
    Encoding: CanEncodeMut<
            Strategy,
            Product![
                ConnectionState,
                ClientId,
                ConnectionCounterparty,
                ConnectionVersion,
                Duration
            ],
        > + CanRaiseAsyncError<&'static str>,
{
    fn encode_mut(
        encoding: &Encoding,
        value: &ConnectionEnd,
        buffer: &mut Encoding::EncodeBuffer,
    ) -> Result<(), Encoding::Error> {
        // FIXME: ibc-rs type doesn't have public fields

        #[derive(serde::Deserialize)]
        struct DummyConnectionEnd {
            pub state: ConnectionState,
            pub client_id: ClientId,
            pub counterparty: ConnectionCounterparty,
            pub version: ConnectionVersion,
            pub delay_period: Duration,
        }

        let DummyConnectionEnd {
            state,
            client_id,
            counterparty,
            version,
            delay_period,
        } = serde_json::to_value(value)
            .and_then(serde_json::from_value)
            .map_err(|_| Encoding::raise_error("invalid connection end"))?;

        encoding.encode_mut(
            &product![state, client_id, counterparty, version, delay_period,],
            buffer,
        )?;
        Ok(())
    }
}

#[cgp_provider(MutDecoderComponent)]
impl<Encoding, Strategy> MutDecoder<Encoding, Strategy, ConnectionEnd> for EncodeConnectionEnd
where
    Encoding: CanDecodeMut<
            Strategy,
            Product![
                ConnectionState,
                ClientId,
                ConnectionCounterparty,
                ConnectionVersion,
                Duration
            ],
        > + CanRaiseAsyncError<&'static str>,
{
    fn decode_mut<'a>(
        encoding: &Encoding,
        buffer: &mut Encoding::DecodeBuffer<'a>,
    ) -> Result<ConnectionEnd, Encoding::Error> {
        let product![state, client_id, counterparty, version, delay_period] =
            encoding.decode_mut(buffer)?;

        ConnectionEnd::new(state, client_id, counterparty, vec![version], delay_period)
            .map_err(|_| Encoding::raise_error("invalid connection end"))
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

#[cgp_provider(MutEncoderComponent)]
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

#[cgp_provider(MutDecoderComponent)]
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
                        .map_err(|_| Encoding::raise_error("invalid connection counterparty"))?,
                ),
                base_prefix,
            ))
        }
    }
}
