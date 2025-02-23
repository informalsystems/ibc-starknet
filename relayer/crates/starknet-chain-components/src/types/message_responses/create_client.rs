use cgp::core::component::UseContext;
use cgp::prelude::*;
use hermes_encoding_components::impls::encode_mut::from::DecodeFrom;
use hermes_encoding_components::traits::decode_mut::MutDecoderComponent;
use hermes_encoding_components::traits::transform::Transformer;

use crate::types::client_id::ClientId;
use crate::types::cosmos::height::Height;

#[derive(Debug, HasField)]
pub struct CreateClientResponse {
    pub client_id: ClientId,
    pub height: Height,
}

pub struct DecodeCreateClientResponse;

delegate_components! {
    DecodeCreateClientResponse {
        MutDecoderComponent: DecodeFrom<Self, UseContext>,
    }
}

impl Transformer for DecodeCreateClientResponse {
    type From = (ClientId, Height);

    type To = CreateClientResponse;

    fn transform((client_id, height): Self::From) -> CreateClientResponse {
        CreateClientResponse { client_id, height }
    }
}
