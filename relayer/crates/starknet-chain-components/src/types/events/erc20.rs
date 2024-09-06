use cgp::core::error::CanRaiseError;
use hermes_cairo_encoding_components::strategy::ViaCairo;
use hermes_cairo_encoding_components::types::as_felt::AsFelt;
use hermes_cairo_encoding_components::HList;
use hermes_encoding_components::traits::decode::{CanDecode, Decoder};
use hermes_encoding_components::traits::has_encoding::HasEncoding;
use hermes_encoding_components::traits::types::encoded::HasEncodedType;
use starknet::core::types::{Felt, U256};
use starknet::macros::selector;

use crate::types::event::{StarknetEvent, UnknownEvent};

#[derive(Debug)]
pub enum Erc20Event {
    Transfer(TransferEvent),
    Approval(ApprovalEvent),
}

#[derive(Debug)]
pub struct TransferEvent {
    pub from: Felt,
    pub to: Felt,
    pub value: U256,
}

#[derive(Debug)]
pub struct ApprovalEvent {
    pub owner: Felt,
    pub spender: Felt,
    pub value: U256,
}

pub struct DecodeErc20Events;

impl<Encoding, Strategy> Decoder<Encoding, Strategy, Erc20Event> for DecodeErc20Events
where
    Encoding: HasEncodedType<Encoded = StarknetEvent>
        + CanDecode<Strategy, TransferEvent>
        + CanDecode<Strategy, ApprovalEvent>
        + for<'a> CanRaiseError<UnknownEvent<'a>>,
{
    fn decode(encoding: &Encoding, event: &StarknetEvent) -> Result<Erc20Event, Encoding::Error> {
        let selector = event
            .selector
            .ok_or_else(|| Encoding::raise_error(UnknownEvent { event }))?;

        if selector == selector!("Transfer") {
            Ok(Erc20Event::Transfer(encoding.decode(event)?))
        } else if selector == selector!("Approval") {
            Ok(Erc20Event::Approval(encoding.decode(event)?))
        } else {
            Err(Encoding::raise_error(UnknownEvent { event }))
        }
    }
}

impl<EventEncoding, CairoEncoding, Strategy> Decoder<EventEncoding, Strategy, TransferEvent>
    for DecodeErc20Events
where
    EventEncoding: HasEncodedType<Encoded = StarknetEvent>
        + HasEncoding<AsFelt, Encoding = CairoEncoding>
        + CanRaiseError<CairoEncoding::Error>,
    CairoEncoding: HasEncodedType<Encoded = Vec<Felt>>
        + CanDecode<ViaCairo, HList![Felt, Felt]>
        + CanDecode<ViaCairo, U256>,
{
    fn decode(
        encoding: &EventEncoding,
        event: &StarknetEvent,
    ) -> Result<TransferEvent, EventEncoding::Error> {
        let cairo_encoding = encoding.encoding();

        let (from, (to, ())) = cairo_encoding
            .decode(&event.keys)
            .map_err(EventEncoding::raise_error)?;

        let value = cairo_encoding
            .decode(&event.data)
            .map_err(EventEncoding::raise_error)?;

        Ok(TransferEvent { from, to, value })
    }
}

impl<EventEncoding, CairoEncoding, Strategy> Decoder<EventEncoding, Strategy, ApprovalEvent>
    for DecodeErc20Events
where
    EventEncoding: HasEncodedType<Encoded = StarknetEvent>
        + HasEncoding<AsFelt, Encoding = CairoEncoding>
        + CanRaiseError<CairoEncoding::Error>,
    CairoEncoding: HasEncodedType<Encoded = Vec<Felt>>
        + CanDecode<ViaCairo, HList![Felt, Felt]>
        + CanDecode<ViaCairo, U256>,
{
    fn decode(
        encoding: &EventEncoding,
        event: &StarknetEvent,
    ) -> Result<ApprovalEvent, EventEncoding::Error> {
        let cairo_encoding = encoding.encoding();

        let (owner, (spender, ())) = cairo_encoding
            .decode(&event.keys)
            .map_err(EventEncoding::raise_error)?;

        let value = cairo_encoding
            .decode(&event.data)
            .map_err(EventEncoding::raise_error)?;

        Ok(ApprovalEvent {
            owner,
            spender,
            value,
        })
    }
}
