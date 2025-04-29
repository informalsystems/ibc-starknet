use cgp::core::component::UseContext;
use hermes_encoding_components::impls::{CombineEncoders, DecodeFrom, EncodeField};
use hermes_encoding_components::traits::{MutDecoderComponent, MutEncoderComponent, Transformer};
use hermes_prelude::*;
use hermes_protobuf_encoding_components::impls::{
    DecodeRequiredProtoField, EncodeByteField, EncodeLengthDelimitedProtoField,
};
use ibc_core::client::types::Height;

use crate::header::{SignedStarknetHeader, StarknetHeader};
use crate::StarknetConsensusState;

pub struct EncodeStarknetHeader;

delegate_components! {
    EncodeStarknetHeader {
        MutEncoderComponent:
            CombineEncoders<Product![
                EncodeField<
                    symbol!("height"),
                    EncodeLengthDelimitedProtoField<1, UseContext>,
                >,
                EncodeField<
                    symbol!("consensus_state"),
                    EncodeLengthDelimitedProtoField<2, UseContext>,
                >,
            ]>,
        MutDecoderComponent: DecodeFrom<
            Self,
            CombineEncoders<Product![
                DecodeRequiredProtoField<1, UseContext>,
                DecodeRequiredProtoField<2, UseContext>,
            ]>
        >,
    }
}

impl Transformer for EncodeStarknetHeader {
    type From = Product![Height, StarknetConsensusState];

    type To = StarknetHeader;

    fn transform(product![height, consensus_state]: Self::From) -> Self::To {
        StarknetHeader {
            height,
            consensus_state,
        }
    }
}

pub struct EncodeSignedStarknetHeader;

delegate_components! {
    EncodeSignedStarknetHeader {
        MutEncoderComponent:
            CombineEncoders<Product![
                EncodeField<
                    symbol!("header"),
                    EncodeByteField<1>,
                >,
                EncodeField<
                    symbol!("signature"),
                    EncodeByteField<2>,
                >,
            ]>,
        MutDecoderComponent: DecodeFrom<
            Self,
            CombineEncoders<Product![
                EncodeByteField<1>,
                EncodeByteField<2>,
            ]>
        >,
    }
}

impl Transformer for EncodeSignedStarknetHeader {
    type From = Product![Vec<u8>, Vec<u8>];

    type To = SignedStarknetHeader;

    fn transform(product![header, signature]: Self::From) -> Self::To {
        SignedStarknetHeader { header, signature }
    }
}
