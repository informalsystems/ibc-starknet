use cgp::core::component::UseContext;
use cgp::prelude::*;
use hermes_encoding_components::impls::encode_mut::combine::CombineEncoders;
use hermes_encoding_components::impls::encode_mut::field::EncodeField;
use hermes_encoding_components::impls::encode_mut::from::DecodeFrom;
use hermes_encoding_components::traits::decode_mut::MutDecoderComponent;
use hermes_encoding_components::traits::encode_mut::MutEncoderComponent;
use hermes_encoding_components::traits::transform::Transformer;

use crate::types::client_id::ClientId;

#[derive(HasField)]
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
