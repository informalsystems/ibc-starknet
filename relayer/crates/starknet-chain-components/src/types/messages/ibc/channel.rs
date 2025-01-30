use cgp::core::component::UseContext;
use cgp::prelude::*;
use hermes_cairo_encoding_components::impls::encode_mut::variant_from::EncodeVariantFrom;
use hermes_encoding_components::impls::encode_mut::combine::CombineEncoders;
use hermes_encoding_components::impls::encode_mut::field::EncodeField;
use hermes_encoding_components::impls::encode_mut::from::DecodeFrom;
use hermes_encoding_components::traits::decode_mut::{
    CanDecodeMut, MutDecoder, MutDecoderComponent,
};
use hermes_encoding_components::traits::encode_mut::{
    CanEncodeMut, MutEncoder, MutEncoderComponent,
};
use hermes_encoding_components::traits::transform::{Transformer, TransformerRef};
pub use ibc::core::host::types::identifiers::PortId;

use super::packet::StateProof;
use crate::types::channel_id::ChannelId;
use crate::types::connection_id::ConnectionId;
use crate::types::cosmos::height::Height;

pub struct EncodePortId;

impl<Encoding, Strategy> MutEncoder<Encoding, Strategy, PortId> for EncodePortId
where
    Encoding: CanEncodeMut<Strategy, Product![String]>,
{
    fn encode_mut(
        encoding: &Encoding,
        value: &PortId,
        buffer: &mut Encoding::EncodeBuffer,
    ) -> Result<(), Encoding::Error> {
        encoding.encode_mut(&product![value.to_string()], buffer)?;
        Ok(())
    }
}

impl<Encoding, Strategy> MutDecoder<Encoding, Strategy, PortId> for EncodePortId
where
    Encoding: CanDecodeMut<Strategy, Product![String]> + CanRaiseAsyncError<&'static str>,
{
    fn decode_mut<'a>(
        encoding: &Encoding,
        buffer: &mut Encoding::DecodeBuffer<'a>,
    ) -> Result<PortId, Encoding::Error> {
        let product![value_str] = encoding.decode_mut(buffer)?;
        value_str
            .parse()
            .map_err(|_| Encoding::raise_error("invalid channel id"))
    }
}
#[derive(HasField, Debug, PartialEq, Clone)]
pub struct AppVersion {
    pub version: String,
}

pub struct EncodeAppVersion;

delegate_components! {
    EncodeAppVersion {
        MutEncoderComponent: CombineEncoders<Product![
            EncodeField<symbol!("version"), UseContext>,
        ]>,
        MutDecoderComponent: DecodeFrom<Self, UseContext>,
    }
}

impl Transformer for EncodeAppVersion {
    type From = String;
    type To = AppVersion;

    fn transform(version: Self::From) -> AppVersion {
        AppVersion { version }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ChannelOrdering {
    Unordered,
    Ordered,
}

pub struct EncodeChannelOrdering;

delegate_components! {
    EncodeChannelOrdering {
        [
            MutEncoderComponent,
            MutDecoderComponent,
        ]: EncodeVariantFrom<EncodeChannelOrdering>,
    }
}

impl TransformerRef for EncodeChannelOrdering {
    type From = ChannelOrdering;
    type To<'a> = Sum![(), ()];

    fn transform<'a>(value: &'a Self::From) -> Self::To<'a> {
        match value {
            ChannelOrdering::Unordered => Either::Left(()),
            ChannelOrdering::Ordered => Either::Right(Either::Left(())),
        }
    }
}

impl Transformer for EncodeChannelOrdering {
    type From = Sum![(), ()];
    type To = ChannelOrdering;

    fn transform(value: Self::From) -> Self::To {
        match value {
            Either::Left(()) => ChannelOrdering::Unordered,
            Either::Right(Either::Left(())) => ChannelOrdering::Ordered,
            Either::Right(Either::Right(value)) => match value {},
        }
    }
}

#[derive(HasField)]
pub struct MsgChanOpenInit {
    pub port_id_on_a: PortId,
    pub conn_id_on_a: ConnectionId,
    pub port_id_on_b: PortId,
    pub version_proposal: AppVersion,
    pub ordering: ChannelOrdering,
}

pub struct EncodeMsgChanOpenInit;

delegate_components! {
    EncodeMsgChanOpenInit {
        MutEncoderComponent: CombineEncoders<Product![
            EncodeField<symbol!("port_id_on_a"), UseContext>,
            EncodeField<symbol!("conn_id_on_a"), UseContext>,
            EncodeField<symbol!("port_id_on_b"), UseContext>,
            EncodeField<symbol!("version_proposal"), UseContext>,
            EncodeField<symbol!("ordering"), UseContext>,
        ]>,
        MutDecoderComponent: DecodeFrom<Self, UseContext>,
    }
}

impl Transformer for EncodeMsgChanOpenInit {
    type From = Product![PortId, ConnectionId, PortId, AppVersion, ChannelOrdering];
    type To = MsgChanOpenInit;

    fn transform(
        product![
            port_id_on_a,
            conn_id_on_a,
            port_id_on_b,
            version_proposal,
            ordering
        ]: Self::From,
    ) -> MsgChanOpenInit {
        MsgChanOpenInit {
            port_id_on_a,
            conn_id_on_a,
            port_id_on_b,
            version_proposal,
            ordering,
        }
    }
}

#[derive(HasField)]
pub struct MsgChanOpenTry {
    pub port_id_on_b: PortId,
    pub conn_id_on_b: ConnectionId,
    pub port_id_on_a: PortId,
    pub chan_id_on_a: ChannelId,
    pub version_on_a: AppVersion,
    pub proof_chan_end_on_a: StateProof,
    pub proof_height_on_a: Height,
    pub ordering: ChannelOrdering,
}

pub struct EncodeMsgChanOpenTry;

delegate_components! {
    EncodeMsgChanOpenTry {
        MutEncoderComponent: CombineEncoders<Product![
            EncodeField<symbol!("port_id_on_b"), UseContext>,
            EncodeField<symbol!("conn_id_on_b"), UseContext>,
            EncodeField<symbol!("port_id_on_a"), UseContext>,
            EncodeField<symbol!("chan_id_on_a"), UseContext>,
            EncodeField<symbol!("version_on_a"), UseContext>,
            EncodeField<symbol!("proof_chan_end_on_a"), UseContext>,
            EncodeField<symbol!("proof_height_on_a"), UseContext>,
            EncodeField<symbol!("ordering"), UseContext>,
        ]>,
        MutDecoderComponent: DecodeFrom<Self, UseContext>,
    }
}

impl Transformer for EncodeMsgChanOpenTry {
    type From = Product![
        PortId,
        ConnectionId,
        PortId,
        ChannelId,
        AppVersion,
        StateProof,
        Height,
        ChannelOrdering
    ];
    type To = MsgChanOpenTry;

    fn transform(
        product![
            port_id_on_b,
            conn_id_on_b,
            port_id_on_a,
            chan_id_on_a,
            version_on_a,
            proof_chan_end_on_a,
            proof_height_on_a,
            ordering
        ]: Self::From,
    ) -> MsgChanOpenTry {
        MsgChanOpenTry {
            port_id_on_b,
            conn_id_on_b,
            port_id_on_a,
            chan_id_on_a,
            version_on_a,
            proof_chan_end_on_a,
            proof_height_on_a,
            ordering,
        }
    }
}

#[derive(HasField)]
pub struct MsgChanOpenAck {
    pub port_id_on_a: PortId,
    pub chan_id_on_a: ChannelId,
    pub chan_id_on_b: ChannelId,
    pub version_on_b: AppVersion,
    pub proof_chan_end_on_b: StateProof,
    pub proof_height_on_b: Height,
}

pub struct EncodeMsgChanOpenAck;

delegate_components! {
    EncodeMsgChanOpenAck {
        MutEncoderComponent: CombineEncoders<Product![
            EncodeField<symbol!("port_id_on_a"), UseContext>,
            EncodeField<symbol!("chan_id_on_a"), UseContext>,
            EncodeField<symbol!("chan_id_on_b"), UseContext>,
            EncodeField<symbol!("version_on_b"), UseContext>,
            EncodeField<symbol!("proof_chan_end_on_b"), UseContext>,
            EncodeField<symbol!("proof_height_on_b"), UseContext>,
        ]>,
        MutDecoderComponent: DecodeFrom<Self, UseContext>,
    }
}

impl Transformer for EncodeMsgChanOpenAck {
    type From = Product![PortId, ChannelId, ChannelId, AppVersion, StateProof, Height];
    type To = MsgChanOpenAck;

    fn transform(
        product![
            port_id_on_a,
            chan_id_on_a,
            chan_id_on_b,
            version_on_b,
            proof_chan_end_on_b,
            proof_height_on_b
        ]: Self::From,
    ) -> MsgChanOpenAck {
        MsgChanOpenAck {
            port_id_on_a,
            chan_id_on_a,
            chan_id_on_b,
            version_on_b,
            proof_chan_end_on_b,
            proof_height_on_b,
        }
    }
}

#[derive(HasField)]
pub struct MsgChanOpenConfirm {
    pub port_id_on_b: PortId,
    pub chan_id_on_b: ChannelId,
    pub proof_chan_end_on_a: StateProof,
    pub proof_height_on_a: Height,
}

pub struct EncodeMsgChanOpenConfirm;

delegate_components! {
    EncodeMsgChanOpenConfirm {
        MutEncoderComponent: CombineEncoders<Product![
            EncodeField<symbol!("port_id_on_b"), UseContext>,
            EncodeField<symbol!("chan_id_on_b"), UseContext>,
            EncodeField<symbol!("proof_chan_end_on_a"), UseContext>,
            EncodeField<symbol!("proof_height_on_a"), UseContext>,
        ]>,
        MutDecoderComponent: DecodeFrom<Self, UseContext>,
    }
}

impl Transformer for EncodeMsgChanOpenConfirm {
    type From = Product![PortId, ChannelId, StateProof, Height];
    type To = MsgChanOpenConfirm;

    fn transform(
        product![
            port_id_on_b,
            chan_id_on_b,
            proof_chan_end_on_a,
            proof_height_on_a
        ]: Self::From,
    ) -> MsgChanOpenConfirm {
        MsgChanOpenConfirm {
            port_id_on_b,
            chan_id_on_b,
            proof_chan_end_on_a,
            proof_height_on_a,
        }
    }
}
