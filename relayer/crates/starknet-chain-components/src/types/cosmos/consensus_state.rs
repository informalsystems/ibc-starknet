use cgp::prelude::*;
use hermes_encoding_components::impls::encode_mut::combine::CombineEncoders;
use hermes_encoding_components::impls::encode_mut::field::EncodeField;
use hermes_encoding_components::impls::encode_mut::from::DecodeFrom;
use hermes_encoding_components::impls::with_context::WithContext;
use hermes_encoding_components::traits::transform::Transformer;
use hermes_encoding_components::HList;
use hermes_wasm_encoding_components::components::{MutDecoderComponent, MutEncoderComponent};
use starknet::core::types::Felt;

#[derive(Debug, HasField)]
pub struct CometConsensusState {
    pub timestamp: u64,
    pub root: Felt,
}

pub struct EncodeCometConsensusState;

delegate_components! {
    EncodeCometConsensusState {
        MutEncoderComponent: CombineEncoders<
            HList![
                EncodeField<symbol!("timestamp"), WithContext>,
                EncodeField<symbol!("root"), WithContext>,
            ],
        >,
        MutDecoderComponent: DecodeFrom<Self, WithContext>,
    }
}

impl Transformer for EncodeCometConsensusState {
    type From = HList![u64, Felt];
    type To = CometConsensusState;

    fn transform(HList![timestamp, root]: Self::From) -> CometConsensusState {
        CometConsensusState { timestamp, root }
    }
}
