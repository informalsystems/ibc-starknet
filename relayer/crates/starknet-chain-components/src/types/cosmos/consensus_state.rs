use cgp::prelude::*;
use hermes_encoding_components::impls::encode_mut::combine::CombineEncoders;
use hermes_encoding_components::impls::encode_mut::field::EncodeField;
use hermes_encoding_components::impls::with_context::WithContext;
use hermes_encoding_components::HList;
use starknet::core::types::Felt;

#[derive(Debug, HasField)]
pub struct CometConsensusState {
    pub timestamp: u64,
    pub root: Felt,
}

pub type EncodeCometConsenussState = CombineEncoders<
    HList![
        EncodeField<symbol!("timestamp"), WithContext>,
        EncodeField<symbol!("root"), WithContext>,
    ],
>;
