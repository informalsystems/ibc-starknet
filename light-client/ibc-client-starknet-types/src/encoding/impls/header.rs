use cgp::core::component::UseContext;
use cgp::prelude::*;
use hermes_cosmos_encoding_components::components::{MutDecoderComponent, MutEncoderComponent};
use hermes_encoding_components::impls::encode_mut::combine::CombineEncoders;
use hermes_encoding_components::impls::encode_mut::field::EncodeField;
use hermes_encoding_components::impls::encode_mut::from::DecodeFrom;
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
                    EncodeLengthDelimitedProtoField<1, UseContext>,
                >,
                EncodeField<
                    symbol!("consensus_state"),
                    EncodeLengthDelimitedProtoField<2, UseContext>,
                >,
            ]>,
        MutDecoderComponent: DecodeFrom<
            Self,
            CombineEncoders<HList![
                DecodeRequiredProtoField<1, UseContext>,
                DecodeRequiredProtoField<2, UseContext>,
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
