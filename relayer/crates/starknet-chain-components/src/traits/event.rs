use core::fmt::Debug;
use std::marker::PhantomData;

use cgp_core::prelude::*;
use hermes_relayer_components::chain::traits::types::event::HasEventType;

#[derive_component(EventDecoderComponent, EventDecoder<Chain>)]
pub trait CanDecodeEvent<Event>: HasEventType + HasErrorType {
    fn decode_event(&self, event: &Self::Event) -> Result<Event, Self::Error>;
}

pub struct UnknownEvent<'a, Chain: HasEventType> {
    pub event: &'a Chain::Event,
}

pub struct EventSelectorMissing<'a, Chain: HasEventType> {
    pub event: &'a Chain::Event,
}

pub struct DelegateEventDecoders<Components>(pub PhantomData<Components>);

impl<Chain, Components, Event> EventDecoder<Chain, Event> for DelegateEventDecoders<Components>
where
    Chain: HasEventType + HasErrorType,
    Components: DelegateComponent<Event>,
    Components::Delegate: EventDecoder<Chain, Event>,
{
    fn decode_event(chain: &Chain, event: &Chain::Event) -> Result<Event, Chain::Error> {
        Components::Delegate::decode_event(chain, event)
    }
}

impl<'a, Chain> Debug for EventSelectorMissing<'a, Chain>
where
    Chain: HasEventType<Event: Debug>,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        writeln!(
            f,
            "cannot parse event with missing selector: {:?}",
            self.event
        )
    }
}

impl<'a, Chain> Debug for UnknownEvent<'a, Chain>
where
    Chain: HasEventType<Event: Debug>,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        writeln!(f, "failed to parse unknown event: {:?}", self.event)
    }
}
