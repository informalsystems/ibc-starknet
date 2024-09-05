use cgp::prelude::*;
use hermes_cairo_encoding_components::impls::encode_mut::combine::CombineEncoders;
use hermes_cairo_encoding_components::impls::encode_mut::field::EncodeField;
use hermes_cairo_encoding_components::impls::encode_mut::from::DecodeFrom;
use hermes_cairo_encoding_components::traits::transform::Transformer;
use hermes_cairo_encoding_components::HList;
use hermes_encoding_components::traits::decode_mut::MutDecoderComponent;
use hermes_encoding_components::traits::encode_mut::MutEncoderComponent;

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
                EncodeField<symbol!("revision_number")>,
                EncodeField<symbol!("revision_height")>,
            ],
        >,
        MutDecoderComponent: DecodeFrom<Self>,
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
