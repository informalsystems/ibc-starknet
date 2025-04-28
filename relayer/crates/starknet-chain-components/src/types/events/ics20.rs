use cgp::prelude::*;
use hermes_cairo_encoding_components::strategy::ViaCairo;
use hermes_cairo_encoding_components::types::as_felt::AsFelt;
use hermes_encoding_components::traits::{
    CanDecode, Decoder, DecoderComponent, HasEncodedType, HasEncoding,
};
use starknet::core::types::{Felt, U256};
use starknet::macros::selector;

use crate::impls::types::address::StarknetAddress;
use crate::types::event::{StarknetEvent, UnknownEvent};
use crate::types::messages::ibc::denom::PrefixedDenom;
use crate::types::messages::ibc::packet::{AckStatus, Acknowledgement};

#[derive(Debug)]
pub enum IbcTransferEvent {
    Send(SendIbcTransferEvent),
    Receive(ReceiveIbcTransferEvent),
    Ack(AckIbcTransferEvent),
    AckStatus(AckStatusIbcTransferEvent),
    Timeout(TimeoutIbcTransferEvent),
    CreateToken(CreateIbcTokenEvent),
}

#[derive(Debug)]
pub struct SendIbcTransferEvent {
    pub sender: StarknetAddress,
    pub receiver: String,
    pub denom: PrefixedDenom,
    pub amount: U256,
    pub memo: String,
}

#[derive(Debug)]
pub struct ReceiveIbcTransferEvent {
    pub sender: String,
    pub receiver: StarknetAddress,
    pub denom: PrefixedDenom,
    pub amount: U256,
    pub memo: String,
    pub success: bool,
}

#[derive(Debug)]
pub struct AckIbcTransferEvent {
    pub sender: StarknetAddress,
    pub receiver: String,
    pub denom: PrefixedDenom,

    pub amount: U256,
    pub memo: String,
    pub ack: Acknowledgement,
}

#[derive(Debug)]
pub struct AckStatusIbcTransferEvent {
    pub ack_status: AckStatus,
}

#[derive(Debug)]
pub struct TimeoutIbcTransferEvent {
    pub receiver: String,
    pub denom: PrefixedDenom,
    pub amount: U256,
    pub memo: String,
}

#[derive(Debug)]
pub struct CreateIbcTokenEvent {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub address: StarknetAddress,
}

pub struct DecodeIbcTransferEvents;

#[cgp_provider(DecoderComponent)]
impl<Encoding, Strategy> Decoder<Encoding, Strategy, IbcTransferEvent> for DecodeIbcTransferEvents
where
    Encoding: HasEncodedType<Encoded = StarknetEvent>
        + CanDecode<Strategy, SendIbcTransferEvent>
        + CanDecode<Strategy, ReceiveIbcTransferEvent>
        + CanDecode<Strategy, AckIbcTransferEvent>
        + CanDecode<Strategy, AckStatusIbcTransferEvent>
        + CanDecode<Strategy, TimeoutIbcTransferEvent>
        + CanDecode<Strategy, CreateIbcTokenEvent>
        + for<'a> CanRaiseAsyncError<UnknownEvent<'a>>,
{
    fn decode(
        encoding: &Encoding,
        event: &StarknetEvent,
    ) -> Result<IbcTransferEvent, Encoding::Error> {
        let selector = event
            .selector
            .ok_or_else(|| Encoding::raise_error(UnknownEvent { event }))?;

        if selector == selector!("SendEvent") {
            Ok(IbcTransferEvent::Send(encoding.decode(event)?))
        } else if selector == selector!("RecvEvent") {
            Ok(IbcTransferEvent::Receive(encoding.decode(event)?))
        } else if selector == selector!("AckEvent") {
            Ok(IbcTransferEvent::Ack(encoding.decode(event)?))
        } else if selector == selector!("AckStatusEvent") {
            Ok(IbcTransferEvent::AckStatus(encoding.decode(event)?))
        } else if selector == selector!("TimeoutEvent") {
            Ok(IbcTransferEvent::Timeout(encoding.decode(event)?))
        } else if selector == selector!("CreateTokenEvent") {
            Ok(IbcTransferEvent::CreateToken(encoding.decode(event)?))
        } else {
            Err(Encoding::raise_error(UnknownEvent { event }))
        }
    }
}

#[cgp_provider(DecoderComponent)]
impl<EventEncoding, CairoEncoding, Strategy> Decoder<EventEncoding, Strategy, SendIbcTransferEvent>
    for DecodeIbcTransferEvents
where
    EventEncoding: HasEncodedType<Encoded = StarknetEvent>
        + HasEncoding<AsFelt, Encoding = CairoEncoding>
        + CanRaiseAsyncError<CairoEncoding::Error>,
    CairoEncoding: HasEncodedType<Encoded = Vec<Felt>>
        + CanDecode<ViaCairo, Product![StarknetAddress, String, PrefixedDenom]>
        + CanDecode<ViaCairo, Product![U256, String]>,
{
    fn decode(
        event_encoding: &EventEncoding,
        event: &StarknetEvent,
    ) -> Result<SendIbcTransferEvent, EventEncoding::Error> {
        let cairo_encoding = event_encoding.encoding();

        let product![sender, receiver, denom] = cairo_encoding
            .decode(&event.keys)
            .map_err(EventEncoding::raise_error)?;

        let product![amount, memo] = cairo_encoding
            .decode(&event.data)
            .map_err(EventEncoding::raise_error)?;

        Ok(SendIbcTransferEvent {
            sender,
            receiver,
            denom,
            amount,
            memo,
        })
    }
}

#[cgp_provider(DecoderComponent)]
impl<EventEncoding, CairoEncoding, Strategy>
    Decoder<EventEncoding, Strategy, ReceiveIbcTransferEvent> for DecodeIbcTransferEvents
where
    EventEncoding: HasEncodedType<Encoded = StarknetEvent>
        + HasEncoding<AsFelt, Encoding = CairoEncoding>
        + CanRaiseAsyncError<CairoEncoding::Error>,
    CairoEncoding: HasEncodedType<Encoded = Vec<Felt>>
        + CanDecode<ViaCairo, Product![String, StarknetAddress, PrefixedDenom]>
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

#[cgp_provider(DecoderComponent)]
impl<EventEncoding, CairoEncoding, Strategy> Decoder<EventEncoding, Strategy, AckIbcTransferEvent>
    for DecodeIbcTransferEvents
where
    EventEncoding: HasEncodedType<Encoded = StarknetEvent>
        + HasEncoding<AsFelt, Encoding = CairoEncoding>
        + CanRaiseAsyncError<CairoEncoding::Error>,
    CairoEncoding: HasEncodedType<Encoded = Vec<Felt>>
        + CanDecode<ViaCairo, Product![StarknetAddress, String, PrefixedDenom]>
        + CanDecode<ViaCairo, Product![U256, String, Acknowledgement]>,
{
    fn decode(
        event_encoding: &EventEncoding,
        event: &StarknetEvent,
    ) -> Result<AckIbcTransferEvent, EventEncoding::Error> {
        let cairo_encoding = event_encoding.encoding();

        let product![sender, receiver, denom] = cairo_encoding
            .decode(&event.keys)
            .map_err(EventEncoding::raise_error)?;

        let product![amount, memo, ack] = cairo_encoding
            .decode(&event.data)
            .map_err(EventEncoding::raise_error)?;

        Ok(AckIbcTransferEvent {
            sender,
            receiver,
            denom,
            amount,
            memo,
            ack,
        })
    }
}

#[cgp_provider(DecoderComponent)]
impl<EventEncoding, CairoEncoding, Strategy>
    Decoder<EventEncoding, Strategy, AckStatusIbcTransferEvent> for DecodeIbcTransferEvents
where
    EventEncoding: HasEncodedType<Encoded = StarknetEvent>
        + HasEncoding<AsFelt, Encoding = CairoEncoding>
        + CanRaiseAsyncError<CairoEncoding::Error>
        + for<'a> CanRaiseAsyncError<UnknownEvent<'a>>,
    CairoEncoding: HasEncodedType<Encoded = Vec<Felt>> + CanDecode<ViaCairo, Product![AckStatus]>,
{
    fn decode(
        event_encoding: &EventEncoding,
        event: &StarknetEvent,
    ) -> Result<AckStatusIbcTransferEvent, EventEncoding::Error> {
        let cairo_encoding = event_encoding.encoding();

        let product![ack_status] = cairo_encoding
            .decode(&event.data)
            .map_err(EventEncoding::raise_error)?;

        if !event.keys.is_empty() {
            return Err(EventEncoding::raise_error(UnknownEvent { event }));
        }

        Ok(AckStatusIbcTransferEvent { ack_status })
    }
}

#[cgp_provider(DecoderComponent)]
impl<EventEncoding, CairoEncoding, Strategy>
    Decoder<EventEncoding, Strategy, TimeoutIbcTransferEvent> for DecodeIbcTransferEvents
where
    EventEncoding: HasEncodedType<Encoded = StarknetEvent>
        + HasEncoding<AsFelt, Encoding = CairoEncoding>
        + CanRaiseAsyncError<CairoEncoding::Error>,
    CairoEncoding: HasEncodedType<Encoded = Vec<Felt>>
        + CanDecode<ViaCairo, Product![String, PrefixedDenom]>
        + CanDecode<ViaCairo, Product![U256, String]>,
{
    fn decode(
        event_encoding: &EventEncoding,
        event: &StarknetEvent,
    ) -> Result<TimeoutIbcTransferEvent, EventEncoding::Error> {
        let cairo_encoding = event_encoding.encoding();

        let product![receiver, denom] = cairo_encoding
            .decode(&event.keys)
            .map_err(EventEncoding::raise_error)?;

        let product![amount, memo] = cairo_encoding
            .decode(&event.data)
            .map_err(EventEncoding::raise_error)?;

        Ok(TimeoutIbcTransferEvent {
            receiver,
            denom,
            amount,
            memo,
        })
    }
}

#[cgp_provider(DecoderComponent)]
impl<EventEncoding, CairoEncoding, Strategy> Decoder<EventEncoding, Strategy, CreateIbcTokenEvent>
    for DecodeIbcTransferEvents
where
    EventEncoding: HasEncodedType<Encoded = StarknetEvent>
        + HasEncoding<AsFelt, Encoding = CairoEncoding>
        + CanRaiseAsyncError<CairoEncoding::Error>,
    CairoEncoding: HasEncodedType<Encoded = Vec<Felt>>
        + CanDecode<ViaCairo, Product![String, String, u8, StarknetAddress]>,
{
    fn decode(
        event_encoding: &EventEncoding,
        event: &StarknetEvent,
    ) -> Result<CreateIbcTokenEvent, EventEncoding::Error> {
        let cairo_encoding = event_encoding.encoding();

        let product![name, symbol, decimals, address] = cairo_encoding
            .decode(&event.keys)
            .map_err(EventEncoding::raise_error)?;

        Ok(CreateIbcTokenEvent {
            name,
            symbol,
            decimals,
            address,
        })
    }
}
