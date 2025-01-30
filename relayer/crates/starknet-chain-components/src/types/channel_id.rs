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
pub use ibc::core::channel::types::channel::Counterparty as ChannelCounterparty;
pub use ibc::core::host::types::identifiers::ChannelId;

use super::connection_id::ConnectionId;
use super::messages::ibc::channel::{AppVersion, ChannelOrdering, PortId};

pub struct EncodeChannelId;

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

pub struct EncodeChannelCounterparty;

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
                Some(channel_id_str.parse().map_err(|_| {
                    Encoding::raise_error("invalid channel counterparty channel id")
                })?),
            ))
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
