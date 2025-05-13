use core::time::Duration;

use hermes_core::encoding_components::traits::{
    CanDecodeMut, CanEncodeMut, MutDecoder, MutDecoderComponent, MutEncoder, MutEncoderComponent,
};
use hermes_prelude::*;
pub use ibc::core::commitment_types::commitment::CommitmentPrefix as BasePrefix;
pub use ibc::core::connection::types::version::Version as ConnectionVersion;

use crate::types::{ClientId, ConnectionId, Height, StateProof};

pub struct EncodeConnectionVersion;

#[cgp_provider(MutEncoderComponent)]
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

#[cgp_provider(MutDecoderComponent)]
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

#[cgp_provider(MutEncoderComponent)]
impl<Encoding, Strategy> MutEncoder<Encoding, Strategy, BasePrefix> for EncodeBasePrefix
where
    Encoding: CanEncodeMut<Strategy, String> + CanRaiseAsyncError<&'static str>,
{
    fn encode_mut(
        encoding: &Encoding,
        value: &BasePrefix,
        buffer: &mut Encoding::EncodeBuffer,
    ) -> Result<(), Encoding::Error> {
        encoding.encode_mut(
            &String::from_utf8(value.clone().into_vec())
                .map_err(|_| Encoding::raise_error("invalid utf8 string for commitment prefix"))?,
            buffer,
        )?;

        Ok(())
    }
}

#[cgp_provider(MutDecoderComponent)]
impl<Encoding, Strategy> MutDecoder<Encoding, Strategy, BasePrefix> for EncodeBasePrefix
where
    Encoding: CanDecodeMut<Strategy, String> + CanRaiseAsyncError<&'static str>,
{
    fn decode_mut<'a>(
        encoding: &Encoding,
        buffer: &mut Encoding::DecodeBuffer<'a>,
    ) -> Result<BasePrefix, Encoding::Error> {
        let value_str = encoding.decode_mut(buffer)?;
        Ok(BasePrefix::from_bytes(value_str.as_bytes()))
    }
}

#[derive(HasField, HasFields)]
pub struct MsgConnOpenInit {
    pub client_id_on_a: ClientId,
    pub client_id_on_b: ClientId,
    pub prefix_on_b: BasePrefix,
    pub version: ConnectionVersion,
    pub delay_period: Duration,
}

#[derive(HasField, HasFields)]
pub struct MsgConnOpenAck {
    pub conn_id_on_a: ConnectionId,
    pub conn_id_on_b: ConnectionId,
    pub proof_conn_end_on_b: StateProof,
    pub proof_height_on_b: Height,
    pub version: ConnectionVersion,
}

#[derive(HasField, HasFields)]
pub struct MsgConnOpenTry {
    pub client_id_on_b: ClientId,
    pub client_id_on_a: ClientId,
    pub conn_id_on_a: ConnectionId,
    pub prefix_on_a: BasePrefix,
    pub version_on_a: ConnectionVersion,
    pub proof_conn_end_on_a: StateProof,
    pub proof_height_on_a: Height,
    pub delay_period: Duration,
}

#[derive(HasField, HasFields)]
pub struct MsgConnOpenConfirm {
    pub conn_id_on_b: ConnectionId,
    pub proof_conn_end_on_a: StateProof,
    pub proof_height_on_a: Height,
}
