use cgp::core::component::UseContext;
use cgp::prelude::*;
use hermes_encoding_components::impls::encode_mut::combine::CombineEncoders;
use hermes_encoding_components::impls::encode_mut::field::EncodeField;
use hermes_encoding_components::impls::encode_mut::from::DecodeFrom;
use hermes_encoding_components::traits::decode_mut::MutDecoderComponent;
use hermes_encoding_components::traits::encode_mut::MutEncoderComponent;
use hermes_encoding_components::traits::transform::Transformer;
use hermes_encoding_components::HList;
use starknet::core::types::Felt;

use crate::types::cosmos::height::Height;

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

pub struct EncodePacket;

delegate_components! {
    EncodePacket {
        MutEncoderComponent: CombineEncoders<
            HList![
                EncodeField<symbol!("sequence"), UseContext>,
                EncodeField<symbol!("src_port_id"), UseContext>,
                EncodeField<symbol!("src_channel_id"), UseContext>,
                EncodeField<symbol!("dst_port_id"), UseContext>,
                EncodeField<symbol!("dst_channel_id"), UseContext>,
                EncodeField<symbol!("data"), UseContext>,
                EncodeField<symbol!("timeout_height"), UseContext>,
                EncodeField<symbol!("timeout_timestamp"), UseContext>,
            ],
        >,
        MutDecoderComponent: DecodeFrom<Self, UseContext>,
    }
}

impl Transformer for EncodePacket {
    type From = HList![u64, String, String, String, String, Vec<Felt>, Height, u64,];

    type To = Packet;

    fn transform(
        (
            sequence,
            (
                src_port_id,
                (
                    src_channel_id,
                    (
                        dst_port_id,
                        (dst_channel_id, (data, (timeout_height, (timeout_timestamp, ())))),
                    ),
                ),
            ),
        ): Self::From,
    ) -> Packet {
        Packet {
            sequence,
            src_port_id,
            src_channel_id,
            dst_port_id,
            dst_channel_id,
            data,
            timeout_height,
            timeout_timestamp,
        }
    }
}
