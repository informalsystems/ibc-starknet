use cgp::core::component::UseContext;
use cgp::prelude::*;
use hermes_encoding_components::impls::encode_mut::combine::CombineEncoders;
use hermes_encoding_components::impls::encode_mut::field::EncodeField;

use crate::impls::types::address::StarknetAddress;
#[derive(HasField)]
pub struct DeployErc20TokenMessage {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub owner: StarknetAddress,
}

pub type EncodeDeployErc20TokenMessage = CombineEncoders<
    Product![
        EncodeField<symbol!("name"), UseContext>,
        EncodeField<symbol!("symbol"), UseContext>,
        EncodeField<symbol!("decimals"), UseContext>,
        EncodeField<symbol!("owner"), UseContext>,
    ],
>;
