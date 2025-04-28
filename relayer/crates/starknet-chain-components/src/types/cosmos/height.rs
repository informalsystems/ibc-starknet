use cgp::core::component::UseContext;
use cgp::prelude::*;
use hermes_core::encoding_components::impls::{CombineEncoders, DecodeFrom, EncodeField};
use hermes_core::encoding_components::traits::{
    MutDecoderComponent, MutEncoderComponent, Transformer,
};

#[derive(Debug, Clone, HasField)]
pub struct Height {
    pub revision_number: u64,
    pub revision_height: u64,
}

pub struct EncodeHeight;

delegate_components! {
    EncodeHeight {
        MutEncoderComponent: CombineEncoders<
            Product![
                EncodeField<symbol!("revision_number"), UseContext>,
                EncodeField<symbol!("revision_height"), UseContext>,
            ],
        >,
        MutDecoderComponent: DecodeFrom<Self, UseContext>,
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
