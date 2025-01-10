use cgp::core::component::UseContext;
use cgp::prelude::*;
use hermes_encoding_components::impls::encode_mut::combine::CombineEncoders;
use hermes_encoding_components::impls::encode_mut::field::EncodeField;
use hermes_encoding_components::impls::encode_mut::from::DecodeFrom;
use hermes_encoding_components::traits::transform::Transformer;
use hermes_protobuf_encoding_components::components::{MutDecoderComponent, MutEncoderComponent};
use hermes_protobuf_encoding_components::impls::encode_mut::proto_field::decode_required::DecodeRequiredProtoField;
use hermes_protobuf_encoding_components::impls::encode_mut::proto_field::encode::EncodeLengthDelimitedProtoField;
use ibc_core::client::types::Height;
use starknet::core::types::Felt;

use crate::StarknetClientState;

pub struct EncodeStarknetClientState;

delegate_components! {
    EncodeStarknetClientState {
        MutEncoderComponent:
            CombineEncoders<Product![
                EncodeField<
                    symbol!("latest_height"),
                    EncodeLengthDelimitedProtoField<1, UseContext>,
                >,
                EncodeField<
                    symbol!("chain_id"),
                    EncodeLengthDelimitedProtoField<2, UseContext>,
                >,
            ]>,
        MutDecoderComponent: DecodeFrom<
            Self,
            CombineEncoders<Product![
                DecodeRequiredProtoField<1, UseContext>,
                DecodeRequiredProtoField<2, UseContext>,
            ]>
        >,
    }
}

impl Transformer for EncodeStarknetClientState {
    type From = Product![Height, Felt];

    type To = StarknetClientState;

    fn transform(product![latest_height, chain_id]: Self::From) -> Self::To {
        StarknetClientState { latest_height, chain_id: chain_id.to_string() }
    }
}
