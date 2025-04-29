use cgp::core::component::UseContext;
use hermes_encoding_components::impls::{CombineEncoders, DecodeFrom, EncodeField};
use hermes_encoding_components::traits::{MutDecoderComponent, MutEncoderComponent, Transformer};
use hermes_prelude::*;
use hermes_protobuf_encoding_components::impls::{
    DecodeRequiredProtoField, EncodeLengthDelimitedProtoField,
};
use ibc_core::commitment_types::commitment::CommitmentRoot;
use ibc_core::primitives::Timestamp;

use crate::StarknetConsensusState;

pub struct EncodeStarknetConsensusState;

delegate_components! {
    EncodeStarknetConsensusState {
        MutEncoderComponent:
            CombineEncoders<Product![
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
            CombineEncoders<Product![
                DecodeRequiredProtoField<1, UseContext>,
                DecodeRequiredProtoField<2, UseContext>,
            ]>
        >,
    }
}

impl Transformer for EncodeStarknetConsensusState {
    type From = Product![CommitmentRoot, Timestamp];

    type To = StarknetConsensusState;

    fn transform(product![root, time]: Self::From) -> Self::To {
        StarknetConsensusState { root, time }
    }
}
