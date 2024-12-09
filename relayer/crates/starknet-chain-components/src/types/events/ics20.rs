use cgp::prelude::*;
use hermes_cairo_encoding_components::strategy::ViaCairo;
use hermes_cairo_encoding_components::types::as_felt::AsFelt;
use hermes_encoding_components::traits::decode::{CanDecode, Decoder};
use hermes_encoding_components::traits::has_encoding::HasEncoding;
use hermes_encoding_components::traits::types::encoded::HasEncodedType;
use starknet::core::types::{Felt, U256};
use starknet::macros::selector;

use crate::types::event::{StarknetEvent, UnknownEvent};
use crate::types::messages::ibc::denom::PrefixedDenom;
use crate::types::messages::ibc::ibc_transfer::Participant;

#[derive(Debug)]
pub enum IbcTransferEvent {
    Receive(ReceiveIbcTransferEvent),
    CreateToken(CreateIbcTokenEvent),
}

#[derive(Debug)]
pub struct ReceiveIbcTransferEvent {
    pub sender: Participant,
    pub receiver: Participant,
    pub denom: PrefixedDenom,
    pub amount: U256,
    pub memo: String,
    pub success: bool,
}

#[derive(Debug)]
pub struct CreateIbcTokenEvent {
    pub name: String,
    pub symbol: String,
    pub address: Felt,
    pub initial_supply: U256,
}

pub struct DecodeIbcTransferEvents;

impl<Encoding, Strategy> Decoder<Encoding, Strategy, IbcTransferEvent> for DecodeIbcTransferEvents
where
    Encoding: HasEncodedType<Encoded = StarknetEvent>
        + CanDecode<Strategy, ReceiveIbcTransferEvent>
        + CanDecode<Strategy, CreateIbcTokenEvent>
        + for<'a> CanRaiseError<UnknownEvent<'a>>,
{
    fn decode(
        encoding: &Encoding,
        event: &StarknetEvent,
    ) -> Result<IbcTransferEvent, Encoding::Error> {
        let selector = event
            .selector
            .ok_or_else(|| Encoding::raise_error(UnknownEvent { event }))?;

        if selector == selector!("RecvEvent") {
            Ok(IbcTransferEvent::Receive(encoding.decode(event)?))
        } else if selector == selector!("CreateTokenEvent") {
            Ok(IbcTransferEvent::CreateToken(encoding.decode(event)?))
        } else {
            Err(Encoding::raise_error(UnknownEvent { event }))
        }
    }
}

impl<EventEncoding, CairoEncoding, Strategy>
    Decoder<EventEncoding, Strategy, ReceiveIbcTransferEvent> for DecodeIbcTransferEvents
where
    EventEncoding: HasEncodedType<Encoded = StarknetEvent>
        + HasEncoding<AsFelt, Encoding = CairoEncoding>
        + CanRaiseError<CairoEncoding::Error>,
    CairoEncoding: HasEncodedType<Encoded = Vec<Felt>>
        + CanDecode<ViaCairo, Product![Participant, Participant, PrefixedDenom]>
        + CanDecode<ViaCairo, Product![U256, String, bool]>,
{
    fn decode(
        event_encoding: &EventEncoding,
        event: &StarknetEvent,
    ) -> Result<ReceiveIbcTransferEvent, EventEncoding::Error> {
        let cairo_encoding = event_encoding.encoding();

        let product![sender, receiver, denom] = cairo_encoding
            .decode(&event.keys)
            .map_err(EventEncoding::raise_error)?;

        let product![amount, memo, success] = cairo_encoding
            .decode(&event.data)
            .map_err(EventEncoding::raise_error)?;

        Ok(ReceiveIbcTransferEvent {
            sender,
            receiver,
            denom,
            amount,
            memo,
            success,
        })
    }
}

impl<EventEncoding, CairoEncoding, Strategy> Decoder<EventEncoding, Strategy, CreateIbcTokenEvent>
    for DecodeIbcTransferEvents
where
    EventEncoding: HasEncodedType<Encoded = StarknetEvent>
        + HasEncoding<AsFelt, Encoding = CairoEncoding>
        + CanRaiseError<CairoEncoding::Error>,
    CairoEncoding: HasEncodedType<Encoded = Vec<Felt>>
        + CanDecode<ViaCairo, Product![String, String, Felt]>
        + CanDecode<ViaCairo, Product![U256]>,
{
    fn decode(
        event_encoding: &EventEncoding,
        event: &StarknetEvent,
    ) -> Result<CreateIbcTokenEvent, EventEncoding::Error> {
        let cairo_encoding = event_encoding.encoding();

        let product![name, symbol, address] = cairo_encoding
            .decode(&event.keys)
            .map_err(EventEncoding::raise_error)?;

        let product![initial_supply] = cairo_encoding
            .decode(&event.data)
            .map_err(EventEncoding::raise_error)?;

        Ok(CreateIbcTokenEvent {
            name,
            symbol,
            address,
            initial_supply,
        })
    }
}
