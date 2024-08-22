use cgp_core::error::CanRaiseError;
use hermes_cairo_encoding_components::strategy::ViaCairo;
use hermes_cairo_encoding_components::types::as_felt::AsFelt;
use hermes_cairo_encoding_components::HList;
use hermes_encoding_components::traits::decode::CanDecode;
use hermes_encoding_components::traits::has_encoding::HasEncoding;
use hermes_encoding_components::traits::types::encoded::HasEncodedType;
use hermes_relayer_components::chain::traits::types::event::HasEventType;
use starknet::core::types::{Felt, U256};
use starknet::macros::selector;

use crate::traits::event::{CanParseEvent, EventParser, EventSelectorMissing, UnknownEvent};
use crate::types::event::StarknetEvent;
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
pub struct ParseIbcTransferEvent;

impl<Chain> EventParser<Chain, IbcTransferEvent> for ParseIbcTransferEvent
where
    Chain: HasEventType<Event = StarknetEvent>
        + CanParseEvent<ReceiveIbcTransferEvent>
        + CanParseEvent<CreateIbcTokenEvent>
        + for<'a> CanRaiseError<EventSelectorMissing<'a, Chain>>
        + for<'a> CanRaiseError<UnknownEvent<'a, Chain>>,
{
    fn parse_event(chain: &Chain, event: &StarknetEvent) -> Result<IbcTransferEvent, Chain::Error> {
        let selector = event
            .selector
            .ok_or_else(|| Chain::raise_error(EventSelectorMissing { event }))?;

        if selector == selector!("RecvEvent") {
            Ok(IbcTransferEvent::Receive(chain.parse_event(event)?))
        } else if selector == selector!("CreateTokenEvent") {
            Ok(IbcTransferEvent::CreateToken(chain.parse_event(event)?))
        } else {
            Err(Chain::raise_error(UnknownEvent { event }))
        }
    }
}

impl<Chain, Encoding> EventParser<Chain, ReceiveIbcTransferEvent> for ParseIbcTransferEvent
where
    Chain: HasEventType<Event = StarknetEvent>
        + HasEncoding<AsFelt, Encoding = Encoding>
        + CanRaiseError<Encoding::Error>,
    Encoding: HasEncodedType<Encoded = Vec<Felt>>
        + CanDecode<ViaCairo, HList![Participant, Participant, PrefixedDenom]>
        + CanDecode<ViaCairo, HList![U256, String, bool]>,
{
    fn parse_event(
        chain: &Chain,
        event: &StarknetEvent,
    ) -> Result<ReceiveIbcTransferEvent, Chain::Error> {
        let encoding = chain.encoding();

        let (sender, (receiver, (denom, ()))) =
            encoding.decode(&event.keys).map_err(Chain::raise_error)?;

        let (amount, (memo, (success, ()))) =
            encoding.decode(&event.data).map_err(Chain::raise_error)?;

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

impl<Chain, Encoding> EventParser<Chain, CreateIbcTokenEvent> for ParseIbcTransferEvent
where
    Chain: HasEventType<Event = StarknetEvent>
        + HasEncoding<AsFelt, Encoding = Encoding>
        + CanRaiseError<Encoding::Error>,
    Encoding: HasEncodedType<Encoded = Vec<Felt>>
        + CanDecode<ViaCairo, HList![String, String, Felt]>
        + CanDecode<ViaCairo, HList![U256]>,
{
    fn parse_event(
        chain: &Chain,
        event: &StarknetEvent,
    ) -> Result<CreateIbcTokenEvent, Chain::Error> {
        let encoding = chain.encoding();

        let (name, (symbol, (address, ()))) =
            encoding.decode(&event.keys).map_err(Chain::raise_error)?;

        let (initial_supply, ()) = encoding.decode(&event.data).map_err(Chain::raise_error)?;

        Ok(CreateIbcTokenEvent {
            name,
            symbol,
            address,
            initial_supply,
        })
    }
}
