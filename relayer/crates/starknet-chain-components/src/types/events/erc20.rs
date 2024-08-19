use cgp_core::error::CanRaiseError;
use hermes_cairo_encoding_components::strategy::ViaCairo;
use hermes_cairo_encoding_components::HList;
use hermes_encoding_components::traits::decoder::CanDecode;
use hermes_encoding_components::traits::encoded::HasEncodedType;
use hermes_encoding_components::traits::has_encoding::HasEncoding;
use hermes_relayer_components::chain::traits::types::event::HasEventType;
use starknet::core::types::{Felt, OrderedEvent, U256};
use starknet::macros::selector;

use crate::traits::event::{CanDecodeEvent, EventDecoder, EventSelectorMissing};

pub enum Erc20Event {
    Transfer(TransferEvent),
    Approval(ApprovalEvent),
}

pub struct TransferEvent {
    pub from: Felt,
    pub to: Felt,
    pub value: U256,
}

pub struct ApprovalEvent {
    pub owner: Felt,
    pub spender: Felt,
    pub value: U256,
}

pub struct DecodeErc20Events;

impl<Chain> EventDecoder<Chain, Erc20Event> for DecodeErc20Events
where
    Chain: HasEventType<Event = OrderedEvent>
        + CanDecodeEvent<TransferEvent>
        + CanDecodeEvent<ApprovalEvent>
        + for<'a> CanRaiseError<EventSelectorMissing<'a, Chain>>,
{
    fn decode_event(
        chain: &Chain,
        event: &OrderedEvent,
    ) -> Result<Option<Erc20Event>, Chain::Error> {
        let selector = event
            .keys
            .get(0)
            .ok_or_else(|| Chain::raise_error(EventSelectorMissing { event }))?;

        if selector == &selector!("Transfer") {
            Ok(chain.decode_event(event)?.map(Erc20Event::Transfer))
        } else if selector == &selector!("Approval") {
            Ok(chain.decode_event(event)?.map(Erc20Event::Approval))
        } else {
            Ok(None)
        }
    }
}

impl<Chain, Encoding> EventDecoder<Chain, TransferEvent> for DecodeErc20Events
where
    Chain: HasEventType<Event = OrderedEvent>
        + HasEncoding<Encoding = Encoding>
        + CanRaiseError<Encoding::Error>,
    Encoding: HasEncodedType<Encoded = Vec<Felt>>
        + CanDecode<ViaCairo, HList![Felt, Felt]>
        + CanDecode<ViaCairo, U256>,
{
    fn decode_event(
        chain: &Chain,
        event: &OrderedEvent,
    ) -> Result<Option<TransferEvent>, Chain::Error> {
        let encoding = chain.encoding();

        let (from, (to, ())) = encoding.decode(&event.keys).map_err(Chain::raise_error)?;

        let value = encoding.decode(&event.data).map_err(Chain::raise_error)?;

        Ok(Some(TransferEvent { from, to, value }))
    }
}

impl<Chain, Encoding> EventDecoder<Chain, ApprovalEvent> for DecodeErc20Events
where
    Chain: HasEventType<Event = OrderedEvent>
        + HasEncoding<Encoding = Encoding>
        + CanRaiseError<Encoding::Error>,
    Encoding: HasEncodedType<Encoded = Vec<Felt>>
        + CanDecode<ViaCairo, HList![Felt, Felt]>
        + CanDecode<ViaCairo, U256>,
{
    fn decode_event(
        chain: &Chain,
        event: &OrderedEvent,
    ) -> Result<Option<ApprovalEvent>, Chain::Error> {
        let encoding = chain.encoding();

        let (owner, (spender, ())) = encoding.decode(&event.keys).map_err(Chain::raise_error)?;

        let value = encoding.decode(&event.data).map_err(Chain::raise_error)?;

        Ok(Some(ApprovalEvent {
            owner,
            spender,
            value,
        }))
    }
}
