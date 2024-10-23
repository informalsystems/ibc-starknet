use cgp::core::component::UseContext;
use cgp::prelude::*;
use hermes_encoding_components::impls::encode_mut::combine::CombineEncoders;
use hermes_encoding_components::impls::encode_mut::field::EncodeField;
use hermes_encoding_components::HList;
use hermes_wasm_encoding_components::components::MutEncoderComponent;
use starknet::core::types::Felt;

use crate::types::cosmos::height::Height;

#[derive(Debug, HasField)]
pub struct CometUpdateHeader {
    pub trusted_height: Height,
    pub target_height: Height,
    pub time: u64,
    pub root: Felt,
}

pub struct EncodeCometUpdateHeader;

delegate_components! {
    EncodeCometUpdateHeader {
        MutEncoderComponent: CombineEncoders<
            HList![
                EncodeField<symbol!("trusted_height"), UseContext>,
                EncodeField<symbol!("target_height"), UseContext>,
                EncodeField<symbol!("time"), UseContext>,
                EncodeField<symbol!("root"), UseContext>,
            ],
        >,
    }
}
