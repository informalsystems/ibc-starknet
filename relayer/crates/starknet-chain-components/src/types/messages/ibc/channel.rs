use cgp::core::component::UseContext;
use cgp::prelude::*;
use hermes_cairo_encoding_components::impls::encode_mut::variant_from::EncodeVariantFrom;
use hermes_cairo_encoding_components::types::either::Either;
use hermes_cairo_encoding_components::Sum;
use hermes_encoding_components::impls::encode_mut::combine::CombineEncoders;
use hermes_encoding_components::impls::encode_mut::field::EncodeField;
use hermes_encoding_components::impls::encode_mut::from::DecodeFrom;
use hermes_encoding_components::traits::decode_mut::MutDecoderComponent;
use hermes_encoding_components::traits::encode_mut::MutEncoderComponent;
use hermes_encoding_components::traits::transform::{Transformer, TransformerRef};

use super::packet::StateProof;
use crate::types::channel_id::ChannelId;
use crate::types::connection_id::ConnectionId;
use crate::types::cosmos::height::Height;

#[derive(HasField, Debug, PartialEq, Clone)]
pub struct PortId {
    pub port_id: String,
}

pub struct EncodePortId;

delegate_components! {
    EncodePortId {
        MutEncoderComponent: CombineEncoders<Product![
            EncodeField<symbol!("port_id"), UseContext>,
        ]>,
        MutDecoderComponent: DecodeFrom<Self, UseContext>,
    }
}

impl Transformer for EncodePortId {
    type From = String;
    type To = PortId;

    fn transform(port_id: Self::From) -> PortId {
        PortId { port_id }
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
    type From = Product![String, ConnectionId, String, String, ChannelOrdering];
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
            port_id_on_a: PortId {
                port_id: port_id_on_a,
            },
            conn_id_on_a,
            port_id_on_b: PortId {
                port_id: port_id_on_b,
            },
            version_proposal: AppVersion {
                version: version_proposal,
            },
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
    type From = Product![String, String, String, String, StateProof, Height];
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
            port_id_on_a: PortId {
                port_id: port_id_on_a,
            },
            chan_id_on_a: ChannelId {
                channel_id: chan_id_on_a,
            },
            chan_id_on_b: ChannelId {
                channel_id: chan_id_on_b,
            },
            version_on_b: AppVersion {
                version: version_on_b,
            },
            proof_chan_end_on_b,
            proof_height_on_b,
        }
    }
}
