use cgp_core::prelude::*;
use hermes_cairo_encoding_components::impls::encode_mut::combine::CombineEncoders;
use hermes_cairo_encoding_components::impls::encode_mut::field::EncodeField;
use hermes_cairo_encoding_components::impls::encode_mut::variant_from::EncodeVariantFrom;
use hermes_cairo_encoding_components::traits::transform::{Transformer, TransformerRef};
use hermes_cairo_encoding_components::types::either::Either;
use hermes_cairo_encoding_components::{HList, Sum};
use hermes_encoding_components::traits::decode_mut::MutDecoderComponent;
use hermes_encoding_components::traits::encode_mut::MutEncoderComponent;
use starknet::core::types::{Felt, U256};

use crate::types::messages::ibc::denom::PrefixedDenom;

#[derive(HasField)]
pub struct IbcTransferMessage {
    pub denom: PrefixedDenom,
    pub amount: U256,
    pub sender: Participant,
    pub receiver: Participant,
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Participant {
    Native(Felt),
    External(Vec<Felt>),
}

pub struct EncodeParticipant;

delegate_components! {
    EncodeParticipant {
        [
            MutEncoderComponent,
            MutDecoderComponent,
        ]: EncodeVariantFrom<EncodeParticipant>,
    }
}

impl TransformerRef for EncodeParticipant {
    type From = Participant;
    type To<'a> = Sum![Felt, &'a Vec<Felt>];

    fn transform<'a>(from: &'a Participant) -> Sum![Felt, &'a Vec<Felt>] {
        match from {
            Participant::Native(address) => Either::Left(*address),
            Participant::External(address) => Either::Right(Either::Left(address)),
        }
    }
}

impl Transformer for EncodeParticipant {
    type From = Sum![Felt, Vec<Felt>];
    type To = Participant;

    fn transform(value: Sum![Felt, Vec<Felt>]) -> Participant {
        match value {
            Either::Left(value) => Participant::Native(value),
            Either::Right(Either::Left(value)) => Participant::External(value),
            Either::Right(Either::Right(value)) => match value {},
        }
    }
}
