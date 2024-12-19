use cgp::core::component::UseContext;
use cgp::prelude::*;
use hermes_encoding_components::impls::encode_mut::combine::CombineEncoders;
use hermes_encoding_components::impls::encode_mut::field::EncodeField;
use hermes_encoding_components::impls::encode_mut::from::DecodeFrom;
use hermes_encoding_components::traits::transform::Transformer;
use hermes_wasm_encoding_components::components::{MutDecoderComponent, MutEncoderComponent};

#[derive(Debug, PartialEq, Clone, HasField, Eq, Ord, PartialOrd)]
pub struct ChannelId {
    pub channel_id: String,
}

pub struct EncodeChannelId;

delegate_components! {
    EncodeChannelId {
        MutEncoderComponent: CombineEncoders<Product![
            EncodeField<symbol!("channel_id"), UseContext>,
        ]>,
        MutDecoderComponent: DecodeFrom<Self, UseContext>,
    }
}

impl Transformer for EncodeChannelId {
    type From = String;
    type To = ChannelId;

    fn transform(channel_id: Self::From) -> ChannelId {
        ChannelId { channel_id }
    }
}
