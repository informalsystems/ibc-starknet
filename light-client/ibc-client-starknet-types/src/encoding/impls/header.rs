use alloc::vec::Vec;

use cgp::core::component::UseContext;
use hermes_encoding_components::impls::{CombineEncoders, DecodeFrom, EncodeField};
use hermes_encoding_components::traits::{MutDecoderComponent, MutEncoderComponent, Transformer};
use hermes_prelude::*;
use hermes_protobuf_encoding_components::impls::{
    DecodeRequiredProtoField, EncodeByteField, EncodeLengthDelimitedProtoField,
};

use crate::header::StarknetHeader;

pub struct EncodeStarknetHeader;

delegate_components! {
    EncodeStarknetHeader {
        MutEncoderComponent:
            CombineEncoders<Product![
                EncodeField<
                    symbol!("block_header"),
                    EncodeByteField<1>,
                >,
                EncodeField<
                    symbol!("block_signature"),
                    EncodeByteField<2>,
                >,
                EncodeField<
                    symbol!("storage_proof"),
                    EncodeByteField<3>,
                >,
            ]>,
        MutDecoderComponent: DecodeFrom<
            Self,
            CombineEncoders<Product![
                EncodeByteField<1>,
                EncodeByteField<2>,
                EncodeByteField<3>,
            ]>
        >,
    }
}

impl Transformer for EncodeStarknetHeader {
    type From = Product![Vec<u8>, Vec<u8>, Vec<u8>];

    type To = StarknetHeader;

    fn transform(product![block_header, block_signature, storage_proof]: Self::From) -> Self::To {
        StarknetHeader {
            block_header,
            block_signature,
            storage_proof,
        }
    }
}
