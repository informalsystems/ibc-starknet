use cgp_core::error::CanRaiseError;
use hermes_cairo_encoding_components::strategy::ViaCairo;
use hermes_cairo_encoding_components::HList;
use hermes_encoding_components::traits::decoder::CanDecode;
use hermes_encoding_components::traits::encoded::HasEncodedType;
use hermes_encoding_components::traits::has_encoding::HasEncoding;
use hermes_relayer_components::chain::traits::types::event::HasEventType;
use starknet::core::types::{Felt, U256};
use starknet::macros::selector;

use crate::traits::event::{CanParseEvent, EventParser, EventSelectorMissing, UnknownEvent};
use crate::types::event::StarknetEvent;
use crate::types::messages::ibc::denom::PrefixedDenom;

#[derive(Debug)]
pub enum IbcTransferEvent {
    Receive(ReceiveIbcTransferEvent),
}

#[derive(Debug)]
pub struct ReceiveIbcTransferEvent {
    pub sender: Felt,
    pub receiver: Felt,
    pub denom: PrefixedDenom,
    pub amount: U256,
    pub memo: String,
}

pub struct ParseIbcTransferEvent;

impl<Chain> EventParser<Chain, IbcTransferEvent> for ParseIbcTransferEvent
where
    Chain: HasEventType<Event = StarknetEvent>
        + CanParseEvent<ReceiveIbcTransferEvent>
        + for<'a> CanRaiseError<EventSelectorMissing<'a, Chain>>
        + for<'a> CanRaiseError<UnknownEvent<'a, Chain>>,
{
    fn parse_event(chain: &Chain, event: &StarknetEvent) -> Result<IbcTransferEvent, Chain::Error> {
        let selector = event
            .selector
            .ok_or_else(|| Chain::raise_error(EventSelectorMissing { event }))?;

        if selector == selector!("RecvEvent") {
            Ok(IbcTransferEvent::Receive(chain.parse_event(event)?))
        } else {
            Err(Chain::raise_error(UnknownEvent { event }))
        }
    }
}

impl<Chain, Encoding> EventParser<Chain, ReceiveIbcTransferEvent> for ParseIbcTransferEvent
where
    Chain: HasEventType<Event = StarknetEvent>
        + HasEncoding<Encoding = Encoding>
        + CanRaiseError<Encoding::Error>,
    Encoding: HasEncodedType<Encoded = Vec<Felt>>
        + CanDecode<ViaCairo, HList![Felt, Felt, PrefixedDenom]>
        + CanDecode<ViaCairo, HList![U256, String]>,
{
    fn parse_event(
        chain: &Chain,
        event: &StarknetEvent,
    ) -> Result<ReceiveIbcTransferEvent, Chain::Error> {
        let encoding = chain.encoding();

        let (sender, (receiver, (denom, ()))) =
            encoding.decode(&event.keys).map_err(Chain::raise_error)?;

        let (amount, (memo, ())) = encoding.decode(&event.data).map_err(Chain::raise_error)?;

        Ok(ReceiveIbcTransferEvent {
            sender,
            receiver,
            denom,
            amount,
            memo,
        })
    }
}
