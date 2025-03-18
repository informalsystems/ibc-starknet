use cgp::core::component::UseContext;
use cgp::prelude::*;
use hermes_encoding_components::impls::encode_mut::combine::CombineEncoders;
use hermes_encoding_components::impls::encode_mut::field::EncodeField;
use hermes_encoding_components::impls::encode_mut::from::DecodeFrom;
use hermes_encoding_components::traits::decode_mut::MutDecoderComponent;
use hermes_encoding_components::traits::encode_mut::MutEncoderComponent;
use hermes_encoding_components::traits::transform::Transformer;

#[derive(Debug, HasField)]
pub struct CometConsensusState {
    pub timestamp: u64,
    pub root: [u32; 8],
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
    type From = Product![u64, [u32; 8]];
    type To = CometConsensusState;

    fn transform(product![timestamp, root]: Self::From) -> CometConsensusState {
        CometConsensusState { timestamp, root }
    }
}
