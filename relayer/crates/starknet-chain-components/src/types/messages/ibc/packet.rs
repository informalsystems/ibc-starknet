use cgp::core::component::UseContext;
use cgp::prelude::*;
use hermes_encoding_components::impls::encode_mut::combine::CombineEncoders;
use hermes_encoding_components::impls::encode_mut::field::EncodeField;
use hermes_encoding_components::impls::encode_mut::from::DecodeFrom;
use hermes_encoding_components::traits::decode_mut::MutDecoderComponent;
use hermes_encoding_components::traits::encode_mut::MutEncoderComponent;
use hermes_encoding_components::traits::transform::Transformer;
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
            Product![
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
    type From = Product![u64, String, String, String, String, Vec<Felt>, Height, u64];

    type To = Packet;

    fn transform(
        product![
            sequence,
            src_port_id,
            src_channel_id,
            dst_port_id,
            dst_channel_id,
            data,
            timeout_height,
            timeout_timestamp
        ]: Self::From,
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

#[derive(HasField)]
pub struct StateProof {
    pub proof: Vec<Felt>,
}

pub struct EncodeStateProof;

delegate_components! {
    EncodeStateProof {
        MutEncoderComponent: CombineEncoders<
            Product![
                EncodeField<symbol!("proof"), UseContext>,
            ],
        >,
        MutDecoderComponent: DecodeFrom<Self, UseContext>,
    }
}

impl Transformer for EncodeStateProof {
    type From = Product![Vec<Felt>];
    type To = StateProof;

    fn transform(product![proof]: Self::From) -> StateProof {
        StateProof { proof }
    }
}

#[derive(HasField)]
pub struct MsgRecvPacket {
    pub packet: Packet,
    pub proof_commitment_on_a: StateProof,
    pub proof_height_on_a: Height,
}

pub struct EncodeMsgRecvPacket;

delegate_components! {
    EncodeMsgRecvPacket {
        MutEncoderComponent: CombineEncoders<
            Product![
                EncodeField<symbol!("packet"), UseContext>,
                EncodeField<symbol!("proof_commitment_on_a"), UseContext>,
                EncodeField<symbol!("proof_height_on_a"), UseContext>,
            ],
        >,
        MutDecoderComponent: DecodeFrom<Self, UseContext>,
    }
}

impl Transformer for EncodeMsgRecvPacket {
    type From = Product![Packet, StateProof, Height];
    type To = MsgRecvPacket;

    fn transform(
        product![packet, proof_commitment_on_a, proof_height_on_a]: Self::From,
    ) -> MsgRecvPacket {
        MsgRecvPacket {
            packet,
            proof_commitment_on_a,
            proof_height_on_a,
        }
    }
}
