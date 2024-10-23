use cgp::core::component::UseContext;
use cgp::prelude::*;
use hermes_encoding_components::impls::encode_mut::combine::CombineEncoders;
use hermes_encoding_components::impls::encode_mut::field::EncodeField;
use hermes_encoding_components::impls::encode_mut::from::DecodeFrom;
use hermes_encoding_components::traits::transform::Transformer;
use hermes_encoding_components::HList;
use hermes_wasm_encoding_components::components::{MutDecoderComponent, MutEncoderComponent};
use starknet::core::types::Felt;

#[derive(Debug, Clone, HasField)]
pub struct ClientId {
    pub client_type: Felt,
    pub sequence: u64,
}

pub struct EncodeClientId;

delegate_components! {
    EncodeClientId {
        MutEncoderComponent: CombineEncoders<HList![
            EncodeField<symbol!("client_type"), UseContext>,
            EncodeField<symbol!("sequence"), UseContext>,
        ]>,
        MutDecoderComponent: DecodeFrom<Self, UseContext>,
    }
}

impl Transformer for EncodeClientId {
    type From = (Felt, u64);
    type To = ClientId;

    fn transform((client_type, sequence): Self::From) -> ClientId {
        ClientId {
            client_type,
            sequence,
        }
    }
}
