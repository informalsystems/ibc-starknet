use hermes_cairo_encoding_components::strategy::ViaCairo;
use hermes_cairo_encoding_components::types::as_felt::AsFelt;
use hermes_core::encoding_components::traits::{
    CanDecode, Decoder, DecoderComponent, HasEncodedType, HasEncoding,
};
use hermes_prelude::*;
use starknet::core::types::{Felt, U256};
use starknet::macros::selector;

use crate::impls::types::address::StarknetAddress;
use crate::types::event::{StarknetEvent, UnknownEvent};

#[derive(Debug)]
pub enum Erc20Event {
    Transfer(TransferEvent),
    Approval(ApprovalEvent),
}

#[derive(Debug)]
pub struct TransferEvent {
    pub from: StarknetAddress,
    pub to: StarknetAddress,
    pub value: U256,
}

#[derive(Debug)]
pub struct ApprovalEvent {
    pub owner: StarknetAddress,
    pub spender: StarknetAddress,
    pub value: U256,
}

pub struct DecodeErc20Events;

#[cgp_provider(DecoderComponent)]
impl<Encoding, Strategy> Decoder<Encoding, Strategy, Erc20Event> for DecodeErc20Events
where
    Encoding: HasEncodedType<Encoded = StarknetEvent>
        + CanDecode<Strategy, TransferEvent>
        + CanDecode<Strategy, ApprovalEvent>
        + for<'a> CanRaiseAsyncError<UnknownEvent<'a>>,
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

#[cgp_provider(DecoderComponent)]
impl<EventEncoding, CairoEncoding, Strategy> Decoder<EventEncoding, Strategy, TransferEvent>
    for DecodeErc20Events
where
    EventEncoding: HasEncodedType<Encoded = StarknetEvent>
        + HasEncoding<AsFelt, Encoding = CairoEncoding>
        + CanRaiseAsyncError<CairoEncoding::Error>,
    CairoEncoding: HasEncodedType<Encoded = Vec<Felt>>
        + CanDecode<ViaCairo, Product![StarknetAddress, StarknetAddress]>
        + CanDecode<ViaCairo, U256>,
{
    fn decode(
        encoding: &EventEncoding,
        event: &StarknetEvent,
    ) -> Result<TransferEvent, EventEncoding::Error> {
        let cairo_encoding = encoding.encoding();

        let product![from, to] = cairo_encoding
            .decode(&event.keys)
            .map_err(EventEncoding::raise_error)?;

        let value = cairo_encoding
            .decode(&event.data)
            .map_err(EventEncoding::raise_error)?;

        Ok(TransferEvent { from, to, value })
    }
}

#[cgp_provider(DecoderComponent)]
impl<EventEncoding, CairoEncoding, Strategy> Decoder<EventEncoding, Strategy, ApprovalEvent>
    for DecodeErc20Events
where
    EventEncoding: HasEncodedType<Encoded = StarknetEvent>
        + HasEncoding<AsFelt, Encoding = CairoEncoding>
        + CanRaiseAsyncError<CairoEncoding::Error>,
    CairoEncoding: HasEncodedType<Encoded = Vec<Felt>>
        + CanDecode<ViaCairo, Product![StarknetAddress, StarknetAddress]>
        + CanDecode<ViaCairo, U256>,
{
    fn decode(
        encoding: &EventEncoding,
        event: &StarknetEvent,
    ) -> Result<ApprovalEvent, EventEncoding::Error> {
        let cairo_encoding = encoding.encoding();

        let product![owner, spender] = cairo_encoding
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
