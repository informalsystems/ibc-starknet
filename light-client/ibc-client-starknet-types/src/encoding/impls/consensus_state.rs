use cgp::core::component::UseContext;
use cgp::prelude::*;
use hermes_encoding_components::impls::encode_mut::combine::CombineEncoders;
use hermes_encoding_components::impls::encode_mut::field::EncodeField;
use hermes_encoding_components::impls::encode_mut::from::DecodeFrom;
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
                    EncodeLengthDelimitedProtoField<1, UseContext>,
                >,
                EncodeField<
                    symbol!("time"),
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

impl Transformer for EncodeStarknetConsensusState {
    type From = HList![CommitmentRoot, Timestamp];

    type To = StarknetConsensusState;

    fn transform(HList![root, time]: Self::From) -> Self::To {
        StarknetConsensusState { root, time }
    }
}
