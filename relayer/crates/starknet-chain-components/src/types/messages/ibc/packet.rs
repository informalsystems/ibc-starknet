use cgp_core::prelude::*;
use hermes_cairo_encoding_components::impls::encode_mut::combine::CombineEncoders;
use hermes_cairo_encoding_components::impls::encode_mut::field::EncodeField;
use hermes_cairo_encoding_components::HList;
use starknet::core::types::Felt;

use crate::types::messages::ibc::height::Height;

#[derive(HasField)]
pub struct Packet {
    pub sequence: u64,
    pub src_port_id: String,
    pub src_channel_id: String,
    pub dst_port_id: String,
    pub dst_channel_id: String,
    pub data: Vec<Felt>,
    pub timeout_height: Height,
    pub timeout_timestamp: u64,
}

pub type EncodePacket = CombineEncoders<
    HList![
        EncodeField<symbol!("sequence")>,
        EncodeField<symbol!("src_port_id")>,
        EncodeField<symbol!("src_channel_id")>,
        EncodeField<symbol!("dst_port_id")>,
        EncodeField<symbol!("dst_channel_id")>,
        EncodeField<symbol!("data")>,
        EncodeField<symbol!("timeout_height")>,
        EncodeField<symbol!("timeout_timestamp")>,
    ],
>;
