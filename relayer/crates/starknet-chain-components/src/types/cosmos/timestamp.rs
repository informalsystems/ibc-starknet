use cgp::core::component::UseContext;
use cgp::prelude::*;
use hermes_encoding_components::impls::encode_mut::combine::CombineEncoders;
use hermes_encoding_components::impls::encode_mut::field::EncodeField;
use hermes_encoding_components::impls::encode_mut::from::DecodeFrom;
use hermes_encoding_components::traits::decode_mut::MutDecoderComponent;
use hermes_encoding_components::traits::encode_mut::MutEncoderComponent;
use hermes_encoding_components::traits::transform::Transformer;

#[derive(Debug, Clone, HasField)]
pub struct Timestamp {
    pub timestamp: u64,
}

pub struct EncodeTimestamp;

delegate_components! {
    EncodeTimestamp {
        MutEncoderComponent: CombineEncoders<
            Product![
                EncodeField<symbol!("timestamp"), UseContext>,
            ],
        >,
        MutDecoderComponent: DecodeFrom<Self, UseContext>,
    }
}

impl Transformer for EncodeTimestamp {
    type From = u64;
    type To = Timestamp;

    fn transform(timestamp: Self::From) -> Timestamp {
        Timestamp { timestamp }
    }
}
