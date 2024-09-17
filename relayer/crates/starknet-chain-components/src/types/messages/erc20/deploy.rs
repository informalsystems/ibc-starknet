use cgp::prelude::*;
use hermes_cairo_encoding_components::HList;
use hermes_encoding_components::impls::encode_mut::combine::CombineEncoders;
use hermes_encoding_components::impls::encode_mut::field::EncodeField;
use hermes_encoding_components::impls::with_context::EncodeWithContext;
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
        EncodeField<symbol!("name"), EncodeWithContext>,
        EncodeField<symbol!("symbol"), EncodeWithContext>,
        EncodeField<symbol!("fixed_supply"), EncodeWithContext>,
        EncodeField<symbol!("recipient"), EncodeWithContext>,
        EncodeField<symbol!("owner"), EncodeWithContext>,
    ],
>;
