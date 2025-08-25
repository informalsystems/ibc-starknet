use cgp::core::component::UseContext;
use hermes_core::encoding_components::impls::{CombineEncoders, DecodeFrom, EncodeField};
use hermes_core::encoding_components::traits::{
    CanDecodeMut, CanEncodeMut, MutDecoder, MutDecoderComponent, MutEncoder, MutEncoderComponent,
    Transformer,
};
use hermes_prelude::*;
pub use ibc::core::host::types::identifiers::Sequence;
use starknet::core::types::Felt;

use crate::types::Height;

#[derive(HasField, Clone, Debug)]
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

#[derive(HasField, HasFields)]
pub struct StateProof {
    pub proof: Vec<u8>,
}

#[derive(HasField, HasFields)]
pub struct MsgRecvPacket {
    pub packet: Packet,
    pub proof_commitment_on_a: StateProof,
    pub proof_height_on_a: Height,
}

#[derive(HasField, HasFields, Debug, Clone)]
pub struct Acknowledgement {
    pub ack: Vec<u8>,
}

#[derive(HasField, HasFields)]
pub struct MsgAckPacket {
    pub packet: Packet,
    pub acknowledgement: Acknowledgement,
    pub proof_ack_on_b: StateProof,
    pub proof_height_on_b: Height,
}

pub struct EncodeSequence;

#[cgp_provider(MutEncoderComponent)]
impl<Encoding, Strategy> MutEncoder<Encoding, Strategy, Sequence> for EncodeSequence
where
    Encoding: CanEncodeMut<Strategy, Product![u64]>,
{
    fn encode_mut(
        encoding: &Encoding,
        sequence: &Sequence,
        buffer: &mut Encoding::EncodeBuffer,
    ) -> Result<(), Encoding::Error> {
        encoding.encode_mut(&product![sequence.value()], buffer)?;
        Ok(())
    }
}

#[cgp_provider(MutDecoderComponent)]
impl<Encoding, Strategy> MutDecoder<Encoding, Strategy, Sequence> for EncodeSequence
where
    Encoding: CanDecodeMut<Strategy, Product![u64]>,
{
    fn decode_mut<'a>(
        encoding: &Encoding,
        buffer: &mut Encoding::DecodeBuffer<'a>,
    ) -> Result<Sequence, Encoding::Error> {
        let product![value] = encoding.decode_mut(buffer)?;
        Ok(Sequence::from(value))
    }
}

#[derive(HasField, HasFields)]
pub struct MsgTimeoutPacket {
    pub packet: Packet,
    pub next_seq_recv_on_b: Sequence,
    pub proof_unreceived_on_b: StateProof,
    pub proof_height_on_b: Height,
}

#[derive(Debug, HasFields)]
pub enum AckStatus {
    Success(Acknowledgement),
    Error(Acknowledgement),
}
