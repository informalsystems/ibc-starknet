use cgp::prelude::*;
use hermes_encoding_components::impls::encode_mut::combine::CombineEncoders;
use hermes_encoding_components::impls::encode_mut::field::EncodeField;
use hermes_encoding_components::impls::encode_mut::from::DecodeFrom;
use hermes_encoding_components::impls::with_context::EncodeWithContext;
use hermes_encoding_components::traits::transform::Transformer;
use hermes_encoding_components::HList;
use hermes_protobuf_encoding_components::components::{MutDecoderComponent, MutEncoderComponent};
use hermes_protobuf_encoding_components::impls::encode_mut::proto_field::decode_required::DecodeRequiredProtoField;
use hermes_protobuf_encoding_components::impls::encode_mut::proto_field::encode::EncodeLengthDelimitedProtoField;
use ibc_core::client::types::Height;

use crate::StarknetClientState;

pub struct EncodeStarknetClientState;

delegate_components! {
    EncodeStarknetClientState {
        MutEncoderComponent:
            CombineEncoders<HList![
                EncodeField<
                    symbol!("latest_height"),
                    EncodeLengthDelimitedProtoField<1, EncodeWithContext>,
                >,
            ]>,
        MutDecoderComponent: DecodeFrom<
            Self,
            CombineEncoders<HList![
                DecodeRequiredProtoField<1, EncodeWithContext>,
            ]>
        >,
    }
}

impl Transformer for EncodeStarknetClientState {
    type From = HList![Height];

    type To = StarknetClientState;

    fn transform(HList![latest_height]: Self::From) -> Self::To {
        StarknetClientState { latest_height }
    }
}
