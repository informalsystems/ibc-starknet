use cgp::prelude::*;
use hermes_cairo_encoding_components::impls::encode_mut::variant_from::EncodeVariantFrom;
use hermes_core::encoding_components::traits::{
    MutDecoderComponent, MutEncoderComponent, Transformer, TransformerRef,
};
pub use ibc::core::channel::types::channel::Order as ChannelOrdering;
pub use ibc::core::channel::types::Version as AppVersion;
pub use ibc::core::host::types::identifiers::PortId;

use super::packet::StateProof;
use crate::types::channel_id::ChannelId;
use crate::types::connection_id::ConnectionId;
use crate::types::cosmos::height::Height;

#[derive(HasFields)]
pub enum SerializeOrdering {
    None,
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
