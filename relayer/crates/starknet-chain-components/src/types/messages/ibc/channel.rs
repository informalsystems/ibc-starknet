use cgp::prelude::*;
use hermes_cairo_encoding_components::impls::encode_mut::variant_from::EncodeVariantFrom;
use hermes_encoding_components::traits::decode_mut::{
    CanDecodeMut, MutDecoder, MutDecoderComponent,
};
use hermes_encoding_components::traits::encode_mut::{
    CanEncodeMut, MutEncoder, MutEncoderComponent,
};
use hermes_encoding_components::traits::transform::{Transformer, TransformerRef};
pub use ibc::core::channel::types::channel::Order as ChannelOrdering;
pub use ibc::core::channel::types::Version as AppVersion;
pub use ibc::core::host::types::identifiers::PortId;

use super::packet::StateProof;
use crate::types::channel_id::ChannelId;
use crate::types::connection_id::ConnectionId;
use crate::types::cosmos::height::Height;

pub struct EncodePortId;

#[cgp_provider(MutEncoderComponent)]
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

#[cgp_provider(MutDecoderComponent)]
impl<Encoding, Strategy> MutDecoder<Encoding, Strategy, PortId> for EncodePortId
where
    Encoding: CanDecodeMut<Strategy, Product![String]> + CanRaiseAsyncError<&'static str>,
{
    fn decode_mut<'a>(
        encoding: &Encoding,
        buffer: &mut Encoding::DecodeBuffer<'a>,
    ) -> Result<PortId, Encoding::Error> {
        let product![value_str] = encoding.decode_mut(buffer)?;
        PortId::new(value_str).map_err(|_| Encoding::raise_error("invalid port id"))
    }
}
pub struct EncodeAppVersion;

#[cgp_provider(MutEncoderComponent)]
impl<Encoding, Strategy> MutEncoder<Encoding, Strategy, AppVersion> for EncodeAppVersion
where
    Encoding: CanEncodeMut<Strategy, Product![String]>,
{
    fn encode_mut(
        encoding: &Encoding,
        value: &AppVersion,
        buffer: &mut Encoding::EncodeBuffer,
    ) -> Result<(), Encoding::Error> {
        encoding.encode_mut(&product![value.to_string()], buffer)?;
        Ok(())
    }
}

#[cgp_provider(MutDecoderComponent)]
impl<Encoding, Strategy> MutDecoder<Encoding, Strategy, AppVersion> for EncodeAppVersion
where
    Encoding: CanDecodeMut<Strategy, Product![String]> + CanRaiseAsyncError<&'static str>,
{
    fn decode_mut<'a>(
        encoding: &Encoding,
        buffer: &mut Encoding::DecodeBuffer<'a>,
    ) -> Result<AppVersion, Encoding::Error> {
        let product![value_str] = encoding.decode_mut(buffer)?;
        Ok(AppVersion::new(value_str))
    }
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
            ChannelOrdering::None => unimplemented!("ChannelOrdering::None is not supported"),
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

#[derive(HasField, HasFields)]
pub struct MsgChanOpenInit {
    pub port_id_on_a: PortId,
    pub conn_id_on_a: ConnectionId,
    pub port_id_on_b: PortId,
    pub version_proposal: AppVersion,
    pub ordering: ChannelOrdering,
}

#[derive(HasField, HasFields)]
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

#[derive(HasField, HasFields)]
pub struct MsgChanOpenAck {
    pub port_id_on_a: PortId,
    pub chan_id_on_a: ChannelId,
    pub chan_id_on_b: ChannelId,
    pub version_on_b: AppVersion,
    pub proof_chan_end_on_b: StateProof,
    pub proof_height_on_b: Height,
}

#[derive(HasField, HasFields)]
pub struct MsgChanOpenConfirm {
    pub port_id_on_b: PortId,
    pub chan_id_on_b: ChannelId,
    pub proof_chan_end_on_a: StateProof,
    pub proof_height_on_a: Height,
}
