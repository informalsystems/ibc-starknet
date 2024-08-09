use cgp_core::prelude::*;
use hermes_cairo_encoding_components::impls::encode_mut::combine::Combine;
use hermes_cairo_encoding_components::impls::encode_mut::field::EncodeField;
use starknet::core::types::{Felt, U256};

#[derive(HasField)]
pub struct DeployErc20TokenMessage {
    pub name: String,
    pub symbol: String,
    pub fixed_supply: U256,
    pub recipient: Felt,
    pub owner: Felt,
}

pub type DeployErc20TokenMessageEncoder = Combine<
    EncodeField<symbol!("name")>,
    Combine<
        EncodeField<symbol!("symbol")>,
        Combine<
            EncodeField<symbol!("fixed_supply")>,
            Combine<EncodeField<symbol!("recipient")>, EncodeField<symbol!("owner")>>,
        >,
    >,
>;
