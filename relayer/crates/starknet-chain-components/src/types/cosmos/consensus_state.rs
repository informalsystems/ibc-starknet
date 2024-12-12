use cgp::core::component::UseContext;
use cgp::prelude::*;
use hermes_encoding_components::impls::encode_mut::combine::CombineEncoders;
use hermes_encoding_components::impls::encode_mut::field::EncodeField;
use hermes_encoding_components::impls::encode_mut::from::DecodeFrom;
use hermes_encoding_components::traits::transform::Transformer;
use hermes_wasm_encoding_components::components::{MutDecoderComponent, MutEncoderComponent};

#[derive(Debug, HasField)]
pub struct CometConsensusState {
    pub timestamp: u64,
    pub root: Vec<u8>,
}

pub struct EncodeCometConsensusState;

delegate_components! {
    EncodeCometConsensusState {
        MutEncoderComponent: CombineEncoders<
            Product![
                EncodeField<symbol!("timestamp"), UseContext>,
                EncodeField<symbol!("root"), UseContext>,
            ],
        >,
        MutDecoderComponent: DecodeFrom<Self, UseContext>,
    }
}

impl Transformer for EncodeCometConsensusState {
    type From = Product![u64, Vec<u8>];
    type To = CometConsensusState;

    fn transform(product![timestamp, root]: Self::From) -> CometConsensusState {
        CometConsensusState { timestamp, root }
    }
}
