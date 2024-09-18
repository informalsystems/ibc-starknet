use cgp::prelude::*;
use hermes_encoding_components::impls::encode_mut::combine::CombineEncoders;
use hermes_encoding_components::impls::encode_mut::field::EncodeField;
use hermes_encoding_components::impls::encode_mut::from::DecodeFrom;
use hermes_encoding_components::impls::with_context::WithContext;
use hermes_encoding_components::traits::transform::Transformer;
use hermes_encoding_components::HList;
use hermes_protobuf_encoding_components::components::{MutDecoderComponent, MutEncoderComponent};
use hermes_protobuf_encoding_components::impls::encode_mut::proto_field::decode_required::DecodeRequiredProtoField;
use hermes_protobuf_encoding_components::impls::encode_mut::proto_field::encode::EncodeLengthDelimitedProtoField;
use ibc_core::commitment_types::commitment::CommitmentRoot;
use ibc_core::primitives::Timestamp;

use crate::StarknetConsensusState;

pub struct EncodeStarknetConsensusState;

delegate_components! {
    EncodeStarknetConsensusState {
        MutEncoderComponent:
            CombineEncoders<HList![
                EncodeField<
                    symbol!("root"),
                    EncodeLengthDelimitedProtoField<1, WithContext>,
                >,
                EncodeField<
                    symbol!("time"),
                    EncodeLengthDelimitedProtoField<2, WithContext>,
                >,
            ]>,
        MutDecoderComponent: DecodeFrom<
            Self,
            CombineEncoders<HList![
                DecodeRequiredProtoField<1, WithContext>,
                DecodeRequiredProtoField<2, WithContext>,
            ]>
        >,
    }
}

impl Transformer for EncodeStarknetConsensusState {
    type From = HList![CommitmentRoot, Timestamp];

    type To = StarknetConsensusState;

    fn transform(HList![root, time]: Self::From) -> Self::To {
        StarknetConsensusState { root, time }
    }
}
