use cgp::prelude::*;
use hermes_cosmos_encoding_components::components::{MutDecoderComponent, MutEncoderComponent};
use hermes_encoding_components::impls::encode_mut::combine::CombineEncoders;
use hermes_encoding_components::impls::encode_mut::field::EncodeField;
use hermes_encoding_components::impls::encode_mut::from::DecodeFrom;
use hermes_encoding_components::impls::with_context::EncodeWithContext;
use hermes_encoding_components::traits::transform::Transformer;
use hermes_encoding_components::HList;
use hermes_protobuf_encoding_components::impls::encode_mut::proto_field::decode_required::DecodeRequiredProtoField;
use hermes_protobuf_encoding_components::impls::encode_mut::proto_field::encode::EncodeLengthDelimitedProtoField;
use ibc_core::client::types::Height;

use crate::header::StarknetHeader;
use crate::StarknetConsensusState;

pub struct EncodeStarknetHeader;

delegate_components! {
    EncodeStarknetHeader {
        MutEncoderComponent:
            CombineEncoders<HList![
                EncodeField<
                    symbol!("height"),
                    EncodeLengthDelimitedProtoField<1, EncodeWithContext>,
                >,
                EncodeField<
                    symbol!("consensus_state"),
                    EncodeLengthDelimitedProtoField<2, EncodeWithContext>,
                >,
            ]>,
        MutDecoderComponent: DecodeFrom<
            Self,
            CombineEncoders<HList![
                DecodeRequiredProtoField<1, EncodeWithContext>,
                DecodeRequiredProtoField<2, EncodeWithContext>,
            ]>
        >,
    }
}

impl Transformer for EncodeStarknetHeader {
    type From = HList![Height, StarknetConsensusState];

    type To = StarknetHeader;

    fn transform(HList![height, consensus_state]: Self::From) -> Self::To {
        StarknetHeader {
            height,
            consensus_state,
        }
    }
}
