use cgp::core::component::UseContext;
use cgp::prelude::*;
use hermes_encoding_components::impls::encode_mut::combine::CombineEncoders;
use hermes_encoding_components::impls::encode_mut::field::EncodeField;
use hermes_encoding_components::impls::encode_mut::from::DecodeFrom;
use hermes_encoding_components::traits::decode_mut::{
    CanDecodeMut, MutDecoder, MutDecoderComponent,
};
use hermes_encoding_components::traits::encode_mut::{
    CanEncodeMut, MutEncoder, MutEncoderComponent,
};
use hermes_encoding_components::traits::transform::Transformer;
pub use ibc::core::commitment_types::commitment::CommitmentPrefix as BasePrefix;
pub use ibc::core::connection::types::version::Version as ConnectionVersion;

use super::packet::StateProof;
use crate::types::client_id::ClientId;
use crate::types::connection_id::ConnectionId;
use crate::types::cosmos::height::Height;

pub struct EncodeConnectionVersion;

impl<Encoding, Strategy> MutEncoder<Encoding, Strategy, ConnectionVersion>
    for EncodeConnectionVersion
where
    Encoding:
        CanEncodeMut<Strategy, Product![String, [String; 2]]> + CanRaiseAsyncError<&'static str>,
{
    fn encode_mut(
        encoding: &Encoding,
        value: &ConnectionVersion,
        buffer: &mut Encoding::EncodeBuffer,
    ) -> Result<(), Encoding::Error> {
        // FIXME: ibc-rs type doesn't have public fields
        #[derive(serde::Deserialize)]
        struct DummyConnectionVersion {
            pub identifier: String,
            pub features: [String; 2],
        }

        let DummyConnectionVersion {
            identifier,
            features,
        } = serde_json::to_value(value)
            .and_then(serde_json::from_value)
            .map_err(|_| Encoding::raise_error("invalid connection version"))?;

        encoding.encode_mut(&product![identifier, features], buffer)?;
        Ok(())
    }
}

impl<Encoding, Strategy> MutDecoder<Encoding, Strategy, ConnectionVersion>
    for EncodeConnectionVersion
where
    Encoding:
        CanDecodeMut<Strategy, Product![String, [String; 2]]> + CanRaiseAsyncError<&'static str>,
{
    fn decode_mut<'a>(
        encoding: &Encoding,
        buffer: &mut Encoding::DecodeBuffer<'a>,
    ) -> Result<ConnectionVersion, Encoding::Error> {
        let product![identifier, features] = encoding.decode_mut(buffer)?;

        // FIXME: ibc-rs type doesn't new method
        #[derive(serde::Serialize)]
        struct DummyConnectionVersion {
            pub identifier: String,
            pub features: [String; 2],
        }

        let connection_version = serde_json::to_value(DummyConnectionVersion {
            identifier,
            features,
        })
        .and_then(serde_json::from_value)
        .map_err(|_| Encoding::raise_error("invalid connection version"))?;

        Ok(connection_version)
    }
}

pub struct EncodeBasePrefix;

impl<Encoding, Strategy> MutEncoder<Encoding, Strategy, BasePrefix> for EncodeBasePrefix
where
    Encoding: CanEncodeMut<Strategy, Product![String]> + CanRaiseAsyncError<&'static str>,
{
    fn encode_mut(
        encoding: &Encoding,
        value: &BasePrefix,
        buffer: &mut Encoding::EncodeBuffer,
    ) -> Result<(), Encoding::Error> {
        encoding.encode_mut(
            &product![String::from_utf8(value.clone().into_vec())
                .map_err(|_| Encoding::raise_error("invalid utf8 string for commitment prefix"))?],
            buffer,
        )?;

        Ok(())
    }
}

impl<Encoding, Strategy> MutDecoder<Encoding, Strategy, BasePrefix> for EncodeBasePrefix
where
    Encoding: CanDecodeMut<Strategy, Product![String]> + CanRaiseAsyncError<&'static str>,
{
    fn decode_mut<'a>(
        encoding: &Encoding,
        buffer: &mut Encoding::DecodeBuffer<'a>,
    ) -> Result<BasePrefix, Encoding::Error> {
        let product![value_str] = encoding.decode_mut(buffer)?;
        Ok(BasePrefix::from_bytes(value_str.as_bytes()))
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
