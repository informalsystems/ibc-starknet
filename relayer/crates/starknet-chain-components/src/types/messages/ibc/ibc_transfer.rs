use cgp_core::prelude::*;
use hermes_cairo_encoding_components::impls::encode_mut::combine::CombineEncoders;
use hermes_cairo_encoding_components::impls::encode_mut::field::EncodeField;
use hermes_cairo_encoding_components::HList;
use starknet::core::types::{Felt, U256};

use crate::types::messages::ibc::denom::PrefixedDenom;

#[derive(HasField)]
pub struct IbcTransferMessage {
    pub denom: PrefixedDenom,
    pub amount: U256,
    pub sender: Vec<Felt>,
    pub receiver: Felt,
    pub memo: String,
}

pub type EncodeIbcTransferMessage = CombineEncoders<
    HList![
        EncodeField<symbol!("denom")>,
        EncodeField<symbol!("amount")>,
        EncodeField<symbol!("sender")>,
        EncodeField<symbol!("receiver")>,
        EncodeField<symbol!("memo")>,
    ],
>;
