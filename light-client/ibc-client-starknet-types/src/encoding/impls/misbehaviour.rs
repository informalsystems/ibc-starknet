use cgp::core::component::UseContext;
use hermes_cosmos_encoding_components::impls::EncodeClientIdField;
use hermes_encoding_components::impls::{CombineEncoders, DecodeFrom, EncodeField};
use hermes_encoding_components::traits::{MutDecoderComponent, MutEncoderComponent, Transformer};
use hermes_prelude::*;
use hermes_protobuf_encoding_components::impls::{
    DecodeRequiredProtoField, EncodeLengthDelimitedProtoField,
};
use ibc_core::host::types::identifiers::ClientId;

use crate::header::StarknetHeader;
use crate::misbehaviour::StarknetMisbehaviour;

pub struct EncodeStarknetMisbehaviour;

delegate_components! {
    EncodeStarknetMisbehaviour {
        MutEncoderComponent:
            CombineEncoders<Product![
                EncodeField<
                    symbol!("client_id"),
                    EncodeClientIdField<1>,
                >,
                EncodeField<
                    symbol!("header_1"),
                    EncodeLengthDelimitedProtoField<2, UseContext>,
                >,
                EncodeField<
                    symbol!("header_2"),
                    EncodeLengthDelimitedProtoField<3, UseContext>,
                >,
            ]>,
        MutDecoderComponent: DecodeFrom<
            Self,
            CombineEncoders<Product![
                EncodeClientIdField<1>,
                DecodeRequiredProtoField<2, UseContext>,
                DecodeRequiredProtoField<3, UseContext>,
            ]>
        >,
    }
}

impl Transformer for EncodeStarknetMisbehaviour {
    type From = Product![ClientId, StarknetHeader, StarknetHeader];

    type To = StarknetMisbehaviour;

    fn transform(product![client_id, header_1, header_2,]: Self::From) -> Self::To {
        StarknetMisbehaviour {
            client_id,
            header_1,
            header_2,
        }
    }
}
