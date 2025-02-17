use std::fmt::Display;

use cgp::core::component::UseContext;
use cgp::prelude::*;
use hermes_cairo_encoding_components::impls::encode_mut::variant_from::EncodeVariantFrom;
use hermes_encoding_components::impls::encode_mut::combine::CombineEncoders;
use hermes_encoding_components::impls::encode_mut::field::EncodeField;
use hermes_encoding_components::impls::encode_mut::from::DecodeFrom;
use hermes_encoding_components::traits::decode_mut::MutDecoderComponent;
use hermes_encoding_components::traits::encode_mut::MutEncoderComponent;
use hermes_encoding_components::traits::transform::{Transformer, TransformerRef};
use starknet::core::types::{Felt, U256};

use super::channel::PortId;
use crate::impls::types::address::StarknetAddress;
use crate::types::channel_id::ChannelId;
use crate::types::cosmos::height::Height;
use crate::types::cosmos::timestamp::Timestamp;
use crate::types::messages::ibc::denom::PrefixedDenom;

#[derive(HasField)]
pub struct TransferPacketData {
    pub denom: PrefixedDenom,
    pub amount: U256,
    pub sender: Participant,
    pub receiver: Participant,
    pub memo: String,
}

pub struct EncodeTransferPacketData;

delegate_components! {
    EncodeTransferPacketData {
        MutEncoderComponent: CombineEncoders<
            Product![
                EncodeField<symbol!("denom"), UseContext>,
                EncodeField<symbol!("amount"), UseContext>,
                EncodeField<symbol!("sender"), UseContext>,
                EncodeField<symbol!("receiver"), UseContext>,
                EncodeField<symbol!("memo"), UseContext>,
            ],
        >,
        MutDecoderComponent: DecodeFrom<Self, UseContext>,
    }
}

impl Transformer for EncodeTransferPacketData {
    type From = Product![PrefixedDenom, U256, Participant, Participant, String];
    type To = TransferPacketData;

    fn transform(
        product![denom, amount, sender, receiver, memo,]: Self::From,
    ) -> TransferPacketData {
        TransferPacketData {
            denom,
            amount,
            sender,
            receiver,
            memo,
        }
    }
}

#[derive(HasField)]
pub struct MsgTransfer {
    pub port_id_on_a: PortId,
    pub chan_id_on_a: ChannelId,
    pub denom: PrefixedDenom,
    pub amount: U256,
    pub receiver: String,
    pub memo: String,
    pub timeout_height_on_b: Height,
    pub timeout_timestamp_on_b: Timestamp,
}

pub struct EncodeMsgTransfer;

delegate_components! {
    EncodeMsgTransfer {
        MutEncoderComponent: CombineEncoders<
            Product![
                EncodeField<symbol!("port_id_on_a"), UseContext>,
                EncodeField<symbol!("chan_id_on_a"), UseContext>,
                EncodeField<symbol!("denom"), UseContext>,
                EncodeField<symbol!("amount"), UseContext>,
                EncodeField<symbol!("receiver"), UseContext>,
                EncodeField<symbol!("memo"), UseContext>,
                EncodeField<symbol!("timeout_height_on_b"), UseContext>,
                EncodeField<symbol!("timeout_timestamp_on_b"), UseContext>,
            ],
        >,
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Participant {
    Native(StarknetAddress),
    External(String),
}

impl Display for Participant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Participant::Native(address) => write!(f, "{}", address),
            Participant::External(address) => write!(f, "{}", address),
        }
    }
}

pub struct EncodeParticipant;

delegate_components! {
    EncodeParticipant {
        [
            MutEncoderComponent,
            MutDecoderComponent,
        ]: EncodeVariantFrom<Self>,
    }
}

impl TransformerRef for EncodeParticipant {
    type From = Participant;
    type To<'a> = Sum![Felt, &'a String];

    fn transform<'a>(from: &'a Participant) -> Sum![Felt, &'a String] {
        match from {
            Participant::Native(address) => Either::Left(**address),
            Participant::External(address) => Either::Right(Either::Left(address)),
        }
    }
}

impl Transformer for EncodeParticipant {
    type From = Sum![Felt, String];
    type To = Participant;

    fn transform(value: Sum![Felt, String]) -> Participant {
        match value {
            Either::Left(value) => Participant::Native(value.into()),
            Either::Right(Either::Left(value)) => Participant::External(value),
            Either::Right(Either::Right(value)) => match value {},
        }
    }
}
