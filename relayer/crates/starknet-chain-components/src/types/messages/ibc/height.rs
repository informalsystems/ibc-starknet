use cgp::prelude::*;
use hermes_cairo_encoding_components::HList;
use hermes_encoding_components::impls::encode_mut::combine::CombineEncoders;
use hermes_encoding_components::impls::encode_mut::field::EncodeField;
use hermes_encoding_components::impls::encode_mut::from::DecodeFrom;
use hermes_encoding_components::impls::with_context::EncodeWithContext;
use hermes_encoding_components::traits::decode_mut::MutDecoderComponent;
use hermes_encoding_components::traits::encode_mut::MutEncoderComponent;
use hermes_encoding_components::traits::transform::Transformer;

#[derive(HasField)]
pub struct Height {
    pub revision_number: u64,
    pub revision_height: u64,
}

pub struct EncodeHeight;

delegate_components! {
    EncodeHeight {
        MutEncoderComponent: CombineEncoders<
            HList![
                EncodeField<symbol!("revision_number"), EncodeWithContext>,
                EncodeField<symbol!("revision_height"), EncodeWithContext>,
            ],
        >,
        MutDecoderComponent: DecodeFrom<Self, EncodeWithContext>,
    }
}

impl Transformer for EncodeHeight {
    type From = (u64, u64);
    type To = Height;

    fn transform((revision_number, revision_height): Self::From) -> Height {
        Height {
            revision_number,
            revision_height,
        }
    }
}
