use cgp::core::component::UseContext;
use cgp::prelude::*;
use hermes_cairo_encoding_components::impls::encode_mut::variant_from::EncodeVariantFrom;
use hermes_encoding_components::impls::encode_mut::combine::CombineEncoders;
use hermes_encoding_components::impls::encode_mut::field::EncodeField;
use hermes_encoding_components::impls::encode_mut::from::DecodeFrom;
use hermes_encoding_components::traits::decode_mut::MutDecoderComponent;
use hermes_encoding_components::traits::encode_mut::MutEncoderComponent;
use hermes_encoding_components::traits::transform::{Transformer, TransformerRef};
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

#[derive(HasField, Debug, Clone)]
pub struct Acknowledgement {
    pub ack: Vec<Felt>,
}

pub struct EncodeAcknowledgement;

delegate_components! {
    EncodeAcknowledgement {
        MutEncoderComponent: CombineEncoders<
            Product![
                EncodeField<symbol!("ack"), UseContext>,
            ],
        >,
        MutDecoderComponent: DecodeFrom<Self, UseContext>,
    }
}

impl Transformer for EncodeAcknowledgement {
    type From = Product![Vec<Felt>];
    type To = Acknowledgement;

    fn transform(product![ack]: Self::From) -> Acknowledgement {
        Acknowledgement { ack }
    }
}

#[derive(HasField)]
pub struct MsgAckPacket {
    pub packet: Packet,
    pub acknowledgement: Acknowledgement,
    pub proof_ack_on_b: StateProof,
    pub proof_height_on_b: Height,
}

pub struct EncodeMsgAckPacket;

delegate_components! {
    EncodeMsgAckPacket {
        MutEncoderComponent: CombineEncoders<
            Product![
                EncodeField<symbol!("packet"), UseContext>,
                EncodeField<symbol!("acknowledgement"), UseContext>,
                EncodeField<symbol!("proof_ack_on_b"), UseContext>,
                EncodeField<symbol!("proof_height_on_b"), UseContext>,
            ],
        >,
        MutDecoderComponent: DecodeFrom<Self, UseContext>,
    }
}

impl Transformer for EncodeMsgAckPacket {
    type From = Product![Packet, Acknowledgement, StateProof, Height];
    type To = MsgAckPacket;

    fn transform(
        product![packet, acknowledgement, proof_ack_on_b, proof_height_on_b]: Self::From,
    ) -> MsgAckPacket {
        MsgAckPacket {
            packet,
            acknowledgement,
            proof_ack_on_b,
            proof_height_on_b,
        }
    }
}

#[derive(HasField, Debug)]
pub struct Sequence {
    pub sequence: u64,
}

pub struct EncodeSequence;

delegate_components! {
    EncodeSequence {
        MutEncoderComponent: CombineEncoders<
            Product![
                EncodeField<symbol!("sequence"), UseContext>,
            ],
        >,
        MutDecoderComponent: DecodeFrom<Self, UseContext>,
    }
}

impl Transformer for EncodeSequence {
    type From = Product![u64];
    type To = Sequence;

    fn transform(product![sequence]: Self::From) -> Sequence {
        Sequence { sequence }
    }
}

#[derive(HasField)]
pub struct MsgTimeoutPacket {
    pub packet: Packet,
    pub next_seq_recv_on_b: Sequence,
    pub proof_unreceived_on_b: StateProof,
    pub proof_height_on_b: Height,
}

pub struct EncodeMsgTimeoutPacket;

delegate_components! {
    EncodeMsgTimeoutPacket {
        MutEncoderComponent: CombineEncoders<
            Product![
                EncodeField<symbol!("packet"), UseContext>,
                EncodeField<symbol!("next_seq_recv_on_b"), UseContext>,
                EncodeField<symbol!("proof_unreceived_on_b"), UseContext>,
                EncodeField<symbol!("proof_height_on_b"), UseContext>,
            ],
        >,
        MutDecoderComponent: DecodeFrom<Self, UseContext>,
    }
}

impl Transformer for EncodeMsgTimeoutPacket {
    type From = Product![Packet, Sequence, StateProof, Height];
    type To = MsgTimeoutPacket;

    fn transform(
        product![
            packet,
            next_seq_recv_on_b,
            proof_unreceived_on_b,
            proof_height_on_b
        ]: Self::From,
    ) -> MsgTimeoutPacket {
        MsgTimeoutPacket {
            packet,
            next_seq_recv_on_b,
            proof_unreceived_on_b,
            proof_height_on_b,
        }
    }
}

#[derive(Debug)]
pub enum AckStatus {
    Success(Acknowledgement),
    Error(Acknowledgement),
}

pub struct EncodeAckStatus;

delegate_components! {
    EncodeAckStatus {
        [
            MutEncoderComponent,
            MutDecoderComponent,
        ]: EncodeVariantFrom<EncodeAckStatus>,
    }
}

impl TransformerRef for EncodeAckStatus {
    type From = AckStatus;
    type To<'a> = Sum![Acknowledgement, Acknowledgement];

    fn transform<'a>(value: &'a Self::From) -> Self::To<'a> {
        match value {
            AckStatus::Success(ack) => Either::Left(ack.clone()),
            AckStatus::Error(ack) => Either::Right(Either::Left(ack.clone())),
        }
    }
}

impl Transformer for EncodeAckStatus {
    type From = Sum![Acknowledgement, Acknowledgement];
    type To = AckStatus;

    fn transform(value: Self::From) -> Self::To {
        match value {
            Either::Left(ack) => AckStatus::Success(ack),
            Either::Right(Either::Left(ack)) => AckStatus::Error(ack),
            Either::Right(Either::Right(value)) => match value {},
        }
    }
}
