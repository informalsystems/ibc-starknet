use alloc::vec::Vec;

use cgp::core::component::UseContext;
use hermes_cosmos_encoding_components::impls::EncodeChainIdField;
use hermes_encoding_components::impls::{CombineEncoders, DecodeFrom, EncodeField};
use hermes_encoding_components::traits::{MutDecoderComponent, MutEncoderComponent, Transformer};
use hermes_prelude::*;
use hermes_protobuf_encoding_components::impls::{
    DecodeRequiredProtoField, EncodeByteField, EncodeLengthDelimitedProtoField,
};
use ibc_core::client::types::Height;
use ibc_core::host::types::identifiers::ChainId;

use crate::StarknetClientState;

pub struct EncodeStarknetClientState;

delegate_components! {
    EncodeStarknetClientState {
        MutEncoderComponent:
            CombineEncoders<Product![
                EncodeField<
                    symbol!("latest_height"),
                    EncodeLengthDelimitedProtoField<1, UseContext>,
                >,
                EncodeField<
                    symbol!("chain_id"),
                    EncodeChainIdField<2>,
                >,
                EncodeField<
                    symbol!("sequencer_public_key"),
                    EncodeByteField<3>,
                >,
                EncodeField<
                    symbol!("ibc_contract_address"),
                    EncodeByteField<4>,
                >,
            ]>,
        MutDecoderComponent: DecodeFrom<
            Self,
            CombineEncoders<Product![
                DecodeRequiredProtoField<1, UseContext>,
                EncodeChainIdField<2>,
                EncodeByteField<3>,
                EncodeByteField<4>,
            ]>
        >,
    }
}

impl Transformer for EncodeStarknetClientState {
    type From = Product![Height, ChainId, Vec<u8>, Vec<u8>];

    type To = StarknetClientState;

    fn transform(
        product![
            latest_height,
            chain_id,
            sequencer_public_key,
            ibc_contract_address,
        ]: Self::From,
    ) -> Self::To {
        StarknetClientState {
            latest_height,
            chain_id,
            sequencer_public_key,
            ibc_contract_address,
        }
    }
}
