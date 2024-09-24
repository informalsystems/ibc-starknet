use cgp::prelude::*;
use hermes_encoding_components::impls::encode_mut::combine::CombineEncoders;
use hermes_encoding_components::impls::encode_mut::field::EncodeField;
use hermes_encoding_components::impls::with_context::WithContext;
use hermes_encoding_components::HList;
use starknet::core::types::{Felt, U256};

#[derive(HasField)]
pub struct DeployErc20TokenMessage {
    pub name: String,
    pub symbol: String,
    pub fixed_supply: U256,
    pub recipient: Felt,
    pub owner: Felt,
}

pub type EncodeDeployErc20TokenMessage = CombineEncoders<
    HList![
        EncodeField<symbol!("name"), WithContext>,
        EncodeField<symbol!("symbol"), WithContext>,
        EncodeField<symbol!("fixed_supply"), WithContext>,
        EncodeField<symbol!("recipient"), WithContext>,
        EncodeField<symbol!("owner"), WithContext>,
    ],
>;
