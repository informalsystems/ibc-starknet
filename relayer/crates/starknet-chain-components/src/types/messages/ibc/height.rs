use cgp_core::prelude::*;
use hermes_cairo_encoding_components::impls::encode_mut::combine::CombineEncoders;
use hermes_cairo_encoding_components::impls::encode_mut::field::EncodeField;
use hermes_cairo_encoding_components::HList;

#[derive(HasField)]
pub struct Height {
    pub revision_number: u64,
    pub revision_height: u64,
}

pub type EncodeHeight = CombineEncoders<
    HList![
        EncodeField<symbol!("revision_number")>,
        EncodeField<symbol!("revision_height")>,
    ],
>;
