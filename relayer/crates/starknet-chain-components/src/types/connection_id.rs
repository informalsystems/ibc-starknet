use cgp::core::component::UseContext;
use cgp::prelude::*;
use hermes_encoding_components::impls::encode_mut::combine::CombineEncoders;
use hermes_encoding_components::impls::encode_mut::field::EncodeField;
use hermes_encoding_components::impls::encode_mut::from::DecodeFrom;
use hermes_encoding_components::traits::transform::Transformer;
use hermes_wasm_encoding_components::components::{MutDecoderComponent, MutEncoderComponent};

#[derive(Debug, PartialEq, Clone, HasField)]
pub struct ConnectionId {
    pub connection_id: String,
}

pub struct EncodeConnectionId;

delegate_components! {
    EncodeConnectionId {
        MutEncoderComponent: CombineEncoders<Product![
            EncodeField<symbol!("connection_id"), UseContext>,
        ]>,
        MutDecoderComponent: DecodeFrom<Self, UseContext>,
    }
}

impl Transformer for EncodeConnectionId {
    type From = String;
    type To = ConnectionId;

    fn transform(connection_id: Self::From) -> ConnectionId {
        ConnectionId { connection_id }
    }
}
