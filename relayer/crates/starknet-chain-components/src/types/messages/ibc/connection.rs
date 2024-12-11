use cgp::core::component::UseContext;
use cgp::prelude::*;
use hermes_encoding_components::impls::encode_mut::combine::CombineEncoders;
use hermes_encoding_components::impls::encode_mut::field::EncodeField;
use hermes_encoding_components::impls::encode_mut::from::DecodeFrom;
use hermes_encoding_components::traits::decode_mut::MutDecoderComponent;
use hermes_encoding_components::traits::encode_mut::MutEncoderComponent;
use hermes_encoding_components::traits::transform::Transformer;

use super::packet::StateProof;
use crate::types::client_id::ClientId;
use crate::types::connection_id::ConnectionId;
use crate::types::cosmos::height::Height;

#[derive(HasField, Clone)]
pub struct ConnectionVersion {
    pub identifier: String,
    pub features: [String; 2],
}

pub struct EncodeConnectionVersion;

delegate_components! {
    EncodeConnectionVersion {
        MutEncoderComponent: CombineEncoders<Product![
            EncodeField<symbol!("identifier"), UseContext>,
            EncodeField<symbol!("features"), UseContext>,
        ]>,
        MutDecoderComponent: DecodeFrom<Self, UseContext>,
    }
}

impl Transformer for EncodeConnectionVersion {
    type From = (String, [String; 2]);
    type To = ConnectionVersion;

    fn transform((identifier, features): Self::From) -> ConnectionVersion {
        ConnectionVersion {
            identifier,
            features,
        }
    }
}

#[derive(HasField)]
pub struct BasePrefix {
    pub prefix: String,
}

pub struct EncodeBasePrefix;

delegate_components! {
    EncodeBasePrefix {
        MutEncoderComponent: CombineEncoders<Product![
            EncodeField<symbol!("prefix"), UseContext>,
        ]>,
        MutDecoderComponent: DecodeFrom<Self, UseContext>,
    }
}

impl Transformer for EncodeBasePrefix {
    type From = String;
    type To = BasePrefix;

    fn transform(prefix: Self::From) -> BasePrefix {
        BasePrefix { prefix }
    }
}

#[derive(HasField)]
pub struct MsgConnOpenInit {
    pub client_id_on_a: ClientId,
    pub client_id_on_b: ClientId,
    pub prefix_on_b: BasePrefix,
    pub version: ConnectionVersion,
    pub delay_period: u64,
}

pub struct EncodeMsgConnOpenInit;

delegate_components! {
    EncodeMsgConnOpenInit {
        MutEncoderComponent: CombineEncoders<Product![
            EncodeField<symbol!("client_id_on_a"), UseContext>,
            EncodeField<symbol!("client_id_on_b"), UseContext>,
            EncodeField<symbol!("prefix_on_b"), UseContext>,
            EncodeField<symbol!("version"), UseContext>,
            EncodeField<symbol!("delay_period"), UseContext>,
        ]>,
        MutDecoderComponent: DecodeFrom<Self, UseContext>,
    }
}

impl Transformer for EncodeMsgConnOpenInit {
    type From = Product![ClientId, ClientId, BasePrefix, ConnectionVersion, u64];
    type To = MsgConnOpenInit;

    fn transform(
        product![
            client_id_on_a,
            client_id_on_b,
            prefix_on_b,
            version,
            delay_period
        ]: Self::From,
    ) -> MsgConnOpenInit {
        MsgConnOpenInit {
            client_id_on_a,
            client_id_on_b,
            prefix_on_b,
            version,
            delay_period,
        }
    }
}

#[derive(HasField)]
pub struct MsgConnOpenAck {
    pub conn_id_on_a: ConnectionId,
    pub conn_id_on_b: ConnectionId,
    pub proof_conn_end_on_b: StateProof,
    pub proof_height_on_b: Height,
    pub version: ConnectionVersion,
}

pub struct EncodeMsgConnOpenAck;

delegate_components! {
    EncodeMsgConnOpenAck {
        MutEncoderComponent: CombineEncoders<Product![
            EncodeField<symbol!("conn_id_on_a"), UseContext>,
            EncodeField<symbol!("conn_id_on_b"), UseContext>,
            EncodeField<symbol!("proof_conn_end_on_b"), UseContext>,
            EncodeField<symbol!("proof_height_on_b"), UseContext>,
            EncodeField<symbol!("version"), UseContext>,
        ]>,
        MutDecoderComponent: DecodeFrom<Self, UseContext>,
    }
}

impl Transformer for EncodeMsgConnOpenAck {
    type From = Product![
        ConnectionId,
        ConnectionId,
        StateProof,
        Height,
        ConnectionVersion
    ];
    type To = MsgConnOpenAck;

    fn transform(
        product![
            conn_id_on_a,
            conn_id_on_b,
            proof_conn_end_on_b,
            proof_height_on_b,
            version
        ]: Self::From,
    ) -> MsgConnOpenAck {
        MsgConnOpenAck {
            conn_id_on_a,
            conn_id_on_b,
            proof_conn_end_on_b,
            proof_height_on_b,
            version,
        }
    }
}

#[derive(HasField)]
pub struct MsgConnOpenTry {
    pub client_id_on_b: ClientId,
    pub client_id_on_a: ClientId,
    pub conn_id_on_a: ConnectionId,
    pub prefix_on_a: BasePrefix,
    pub version_on_a: ConnectionVersion,
    pub proof_conn_end_on_a: StateProof,
    pub proof_height_on_a: Height,
    pub delay_period: u64,
}

pub struct EncodeMsgConnOpenTry;

delegate_components! {
    EncodeMsgConnOpenTry {
        MutEncoderComponent: CombineEncoders<Product![
            EncodeField<symbol!("client_id_on_b"), UseContext>,
            EncodeField<symbol!("client_id_on_a"), UseContext>,
            EncodeField<symbol!("conn_id_on_a"), UseContext>,
            EncodeField<symbol!("prefix_on_a"), UseContext>,
            EncodeField<symbol!("version_on_a"), UseContext>,
            EncodeField<symbol!("proof_conn_end_on_a"), UseContext>,
            EncodeField<symbol!("proof_height_on_a"), UseContext>,
            EncodeField<symbol!("delay_period"), UseContext>,
        ]>,
        MutDecoderComponent: DecodeFrom<Self, UseContext>,
    }
}

impl Transformer for EncodeMsgConnOpenTry {
    type From = Product![
        ClientId,
        ClientId,
        ConnectionId,
        BasePrefix,
        ConnectionVersion,
        StateProof,
        Height,
        u64
    ];
    type To = MsgConnOpenTry;

    fn transform(
        product![
            client_id_on_b,
            client_id_on_a,
            conn_id_on_a,
            prefix_on_a,
            version_on_a,
            proof_conn_end_on_a,
            proof_height_on_a,
            delay_period
        ]: Self::From,
    ) -> MsgConnOpenTry {
        MsgConnOpenTry {
            client_id_on_b,
            client_id_on_a,
            conn_id_on_a,
            prefix_on_a,
            version_on_a,
            proof_conn_end_on_a,
            proof_height_on_a,
            delay_period,
        }
    }
}

#[derive(HasField)]
pub struct MsgConnOpenConfirm {
    pub conn_id_on_b: ConnectionId,
    pub proof_conn_end_on_a: StateProof,
    pub proof_height_on_a: Height,
}

pub struct EncodeMsgConnOpenConfirm;

delegate_components! {
    EncodeMsgConnOpenConfirm {
        MutEncoderComponent: CombineEncoders<Product![
            EncodeField<symbol!("conn_id_on_b"), UseContext>,
            EncodeField<symbol!("proof_conn_end_on_a"), UseContext>,
            EncodeField<symbol!("proof_height_on_a"), UseContext>,
        ]>,
        MutDecoderComponent: DecodeFrom<Self, UseContext>,
    }
}

impl Transformer for EncodeMsgConnOpenConfirm {
    type From = Product![ConnectionId, StateProof, Height];
    type To = MsgConnOpenConfirm;

    fn transform(
        product![conn_id_on_b, proof_conn_end_on_a, proof_height_on_a]: Self::From,
    ) -> MsgConnOpenConfirm {
        MsgConnOpenConfirm {
            conn_id_on_b,
            proof_conn_end_on_a,
            proof_height_on_a,
        }
    }
}
