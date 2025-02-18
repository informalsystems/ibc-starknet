use cgp::prelude::*;
use hermes_cairo_encoding_components::impls::encode_mut::variant_from::EncodeVariantFrom;
use hermes_encoding_components::traits::decode_mut::{CanDecodeMut, MutDecoder};
use hermes_encoding_components::traits::encode_mut::{CanEncodeMut, MutEncoder};
use hermes_encoding_components::traits::transform::{Transformer, TransformerRef};
use hermes_wasm_encoding_components::components::{MutDecoderComponent, MutEncoderComponent};
pub use ibc::core::channel::types::channel::{
    ChannelEnd, Counterparty as ChannelCounterparty, State as ChannelState,
};
pub use ibc::core::host::types::identifiers::ChannelId;

use super::connection_id::ConnectionId;
use super::messages::ibc::channel::{AppVersion, ChannelOrdering, PortId};

pub struct EncodeChannelId;

#[cgp_provider(MutEncoderComponent)]
impl<Encoding, Strategy> MutEncoder<Encoding, Strategy, ChannelId> for EncodeChannelId
where
    Encoding: CanEncodeMut<Strategy, Product![String]>,
{
    fn encode_mut(
        encoding: &Encoding,
        value: &ChannelId,
        buffer: &mut Encoding::EncodeBuffer,
    ) -> Result<(), Encoding::Error> {
        encoding.encode_mut(&product![value.to_string()], buffer)?;
        Ok(())
    }
}

#[cgp_provider(MutDecoderComponent)]
impl<Encoding, Strategy> MutDecoder<Encoding, Strategy, ChannelId> for EncodeChannelId
where
    Encoding: CanDecodeMut<Strategy, Product![String]> + CanRaiseAsyncError<&'static str>,
{
    fn decode_mut<'a>(
        encoding: &Encoding,
        buffer: &mut Encoding::DecodeBuffer<'a>,
    ) -> Result<ChannelId, Encoding::Error> {
        let product![value_str] = encoding.decode_mut(buffer)?;
        value_str
            .parse()
            .map_err(|_| Encoding::raise_error("invalid channel id"))
    }
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

pub struct EncodeChannelCounterparty;

#[cgp_provider(MutEncoderComponent)]
impl<Encoding, Strategy> MutEncoder<Encoding, Strategy, ChannelCounterparty>
    for EncodeChannelCounterparty
where
    Encoding: CanEncodeMut<Strategy, Product![PortId, Product![String]]>,
{
    fn encode_mut(
        encoding: &Encoding,
        value: &ChannelCounterparty,
        buffer: &mut Encoding::EncodeBuffer,
    ) -> Result<(), Encoding::Error> {
        match &value.channel_id {
            Some(channel_id) => {
                encoding.encode_mut(
                    &product![value.port_id.clone(), product![channel_id.to_string()]],
                    buffer,
                )?;
            }
            None => {
                encoding.encode_mut(
                    &product![value.port_id.clone(), product![String::new()]],
                    buffer,
                )?;
            }
        }
        Ok(())
    }
}

#[cgp_provider(MutDecoderComponent)]
impl<Encoding, Strategy> MutDecoder<Encoding, Strategy, ChannelCounterparty>
    for EncodeChannelCounterparty
where
    Encoding: CanDecodeMut<Strategy, Product![PortId, Product![String]]>
        + CanRaiseAsyncError<&'static str>,
{
    fn decode_mut<'a>(
        encoding: &Encoding,
        buffer: &mut Encoding::DecodeBuffer<'a>,
    ) -> Result<ChannelCounterparty, Encoding::Error> {
        let product![port_id, product![channel_id_str]] = encoding.decode_mut(buffer)?;
        if channel_id_str.is_empty() {
            Ok(ChannelCounterparty::new(port_id, None))
        } else {
            Ok(ChannelCounterparty::new(
                port_id,
                Some(
                    channel_id_str
                        .parse()
                        .map_err(|_| Encoding::raise_error("invalid channel counterparty"))?,
                ),
            ))
        }
    }
}

pub struct EncodeChannelEnd;

#[cgp_provider(MutEncoderComponent)]
impl<Encoding, Strategy> MutEncoder<Encoding, Strategy, ChannelEnd> for EncodeChannelEnd
where
    Encoding: CanEncodeMut<
            Strategy,
            Product![
                ChannelState,
                ChannelOrdering,
                ChannelCounterparty,
                ConnectionId,
                AppVersion,
            ],
        > + CanRaiseAsyncError<&'static str>,
{
    fn encode_mut(
        encoding: &Encoding,
        value: &ChannelEnd,
        buffer: &mut Encoding::EncodeBuffer,
    ) -> Result<(), Encoding::Error> {
        if value.connection_hops.len() != 1 {
            return Err(Encoding::raise_error("invalid connection hops"));
        }

        encoding.encode_mut(
            &product![
                value.state,
                value.ordering,
                value.counterparty().clone(),
                value.connection_hops[0].clone(),
                value.version.clone(),
            ],
            buffer,
        )?;
        Ok(())
    }
}

#[cgp_provider(MutDecoderComponent)]
impl<Encoding, Strategy> MutDecoder<Encoding, Strategy, ChannelEnd> for EncodeChannelEnd
where
    Encoding: CanDecodeMut<
            Strategy,
            Product![
                ChannelState,
                ChannelOrdering,
                ChannelCounterparty,
                ConnectionId,
                AppVersion,
            ],
        > + CanRaiseAsyncError<&'static str>,
{
    fn decode_mut<'a>(
        encoding: &Encoding,
        buffer: &mut Encoding::DecodeBuffer<'a>,
    ) -> Result<ChannelEnd, Encoding::Error> {
        let product![state, ordering, counterparty, connection_id, version,] =
            encoding.decode_mut(buffer)?;

        ChannelEnd::new(state, ordering, counterparty, vec![connection_id], version)
            .map_err(|_| Encoding::raise_error("invalid channel end"))
    }
}
